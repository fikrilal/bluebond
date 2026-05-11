use std::fmt;
use std::path::{Path, PathBuf};

use crate::app::plan;
use crate::app::scan::ScanReport;
use crate::convert::bluez_info;
use crate::convert::windows_key_material::WindowsBluetoothKeyMaterial;
use crate::convert::windows_to_bluez::BluezInfoKeySections;
use crate::domain::{BluetoothAddress, SyncPlan, SyncPlanAction};
use crate::error::{BluebondError, Result};
use crate::infra::backup;
use crate::infra::bluez::store;
use crate::infra::linux::privileges;
use crate::infra::windows::bthport;

#[derive(Clone, Eq, PartialEq)]
pub struct BluezInfoContentPreview {
    pub changes: Vec<BluezInfoContentChange>,
}

impl fmt::Debug for BluezInfoContentPreview {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BluezInfoContentPreview")
            .field("changes", &self.changes)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct BluezInfoContentChange {
    pub display_name: String,
    pub linux_adapter_address: BluetoothAddress,
    pub linux_target_device_address: BluetoothAddress,
    pub windows_source_device_address: BluetoothAddress,
    pub target_info_path: PathBuf,
    pub existing_info_content: Option<String>,
    pub next_info_content: String,
    pub content_changed: bool,
}

impl fmt::Debug for BluezInfoContentChange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BluezInfoContentChange")
            .field("display_name", &self.display_name)
            .field("linux_adapter_address", &self.linux_adapter_address)
            .field(
                "linux_target_device_address",
                &self.linux_target_device_address,
            )
            .field(
                "windows_source_device_address",
                &self.windows_source_device_address,
            )
            .field("target_info_path", &self.target_info_path)
            .field(
                "existing_info_content",
                &redacted_optional_content(&self.existing_info_content),
            )
            .field("next_info_content", &"<redacted>")
            .field("content_changed", &self.content_changed)
            .finish()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BluezInfoPreviewRequest {
    pub bluez_dir: PathBuf,
    pub existing_infos: Vec<ExistingBluezInfoContent>,
    pub windows_key_materials: Vec<WindowsDeviceKeyMaterial>,
}

#[derive(Clone, Eq, PartialEq)]
pub struct ApplyDryRunReport {
    pub content_preview: BluezInfoContentPreview,
    pub backup_snapshot: BackupSnapshot,
    pub no_changes_made: bool,
}

impl fmt::Debug for ApplyDryRunReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ApplyDryRunReport")
            .field("content_preview", &self.content_preview)
            .field("backup_snapshot", &self.backup_snapshot)
            .field("no_changes_made", &self.no_changes_made)
            .finish()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ApplyDryRunRequest {
    pub backup_root: PathBuf,
}

impl ApplyDryRunRequest {
    pub fn new(backup_root: impl Into<PathBuf>) -> Self {
        Self {
            backup_root: backup_root.into(),
        }
    }
}

impl BluezInfoPreviewRequest {
    pub fn new(bluez_dir: impl Into<PathBuf>) -> Self {
        Self {
            bluez_dir: bluez_dir.into(),
            existing_infos: Vec::new(),
            windows_key_materials: Vec::new(),
        }
    }

    pub fn with_existing_infos(mut self, existing_infos: Vec<ExistingBluezInfoContent>) -> Self {
        self.existing_infos = existing_infos;
        self
    }

    pub fn with_windows_key_materials(
        mut self,
        windows_key_materials: Vec<WindowsDeviceKeyMaterial>,
    ) -> Self {
        self.windows_key_materials = windows_key_materials;
        self
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ExistingBluezInfoContent {
    pub linux_adapter_address: BluetoothAddress,
    pub linux_device_address: BluetoothAddress,
    pub content: String,
}

impl fmt::Debug for ExistingBluezInfoContent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExistingBluezInfoContent")
            .field("linux_adapter_address", &self.linux_adapter_address)
            .field("linux_device_address", &self.linux_device_address)
            .field("content", &"<redacted>")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct WindowsDeviceKeyMaterial {
    pub linux_adapter_address: BluetoothAddress,
    pub windows_device_address: BluetoothAddress,
    pub material: WindowsBluetoothKeyMaterial,
}

impl fmt::Debug for WindowsDeviceKeyMaterial {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WindowsDeviceKeyMaterial")
            .field("linux_adapter_address", &self.linux_adapter_address)
            .field("windows_device_address", &self.windows_device_address)
            .field("material", &self.material)
            .finish()
    }
}

pub fn preview_bluez_info_content(
    plan: &SyncPlan,
    request: &BluezInfoPreviewRequest,
) -> Result<BluezInfoContentPreview> {
    let changes = plan
        .actions
        .iter()
        .map(|action| preview_action(action, request))
        .collect::<Result<Vec<_>>>()?;

    Ok(BluezInfoContentPreview { changes })
}

pub fn collect_preview_request(
    scan_report: &ScanReport,
    plan: &SyncPlan,
) -> Result<BluezInfoPreviewRequest> {
    let existing_infos = collect_existing_infos(scan_report, plan)?;
    let windows_key_materials = collect_windows_key_materials(scan_report, plan)?;

    Ok(BluezInfoPreviewRequest::new(&scan_report.bluez_dir)
        .with_existing_infos(existing_infos)
        .with_windows_key_materials(windows_key_materials))
}

pub fn preview_from_scan_report(
    scan_report: &ScanReport,
    plan: &SyncPlan,
) -> Result<BluezInfoContentPreview> {
    let request = collect_preview_request(scan_report, plan)?;
    preview_bluez_info_content(plan, &request)
}

pub fn build_dry_run_report(
    scan_report: &ScanReport,
    request: &ApplyDryRunRequest,
) -> Result<ApplyDryRunReport> {
    let sync_plan = plan::build_sync_plan(scan_report);
    let content_preview = preview_from_scan_report(scan_report, &sync_plan)?;
    let backup_snapshot = build_backup_snapshot(&content_preview, &request.backup_root);

    Ok(ApplyDryRunReport {
        content_preview,
        backup_snapshot,
        no_changes_made: true,
    })
}

pub fn default_dry_run_request() -> ApplyDryRunRequest {
    ApplyDryRunRequest::new(Path::new(backup::store::DEFAULT_BACKUP_DIR).join("dry-run-preview"))
}

pub fn require_privileged_apply() -> Result<()> {
    require_privileged_apply_with(privileges::running_as_root())
}

pub fn require_privileged_apply_with(running_as_root: bool) -> Result<()> {
    if running_as_root {
        Ok(())
    } else {
        Err(BluebondError::PrivilegeRequired { operation: "apply" })
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct BackupSnapshot {
    pub root_dir: PathBuf,
    pub entries: Vec<BackupSnapshotEntry>,
}

impl fmt::Debug for BackupSnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BackupSnapshot")
            .field("root_dir", &self.root_dir)
            .field("entries", &self.entries)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct BackupSnapshotEntry {
    pub source_path: PathBuf,
    pub backup_path: PathBuf,
    pub content: String,
}

impl fmt::Debug for BackupSnapshotEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BackupSnapshotEntry")
            .field("source_path", &self.source_path)
            .field("backup_path", &self.backup_path)
            .field("content", &"<redacted>")
            .finish()
    }
}

pub fn build_backup_snapshot(
    preview: &BluezInfoContentPreview,
    snapshot_root: impl Into<PathBuf>,
) -> BackupSnapshot {
    let root_dir = snapshot_root.into();
    let entries = preview
        .changes
        .iter()
        .filter_map(|change| backup_entry_for_change(change, &root_dir))
        .collect();

    BackupSnapshot { root_dir, entries }
}

fn preview_action(
    action: &SyncPlanAction,
    request: &BluezInfoPreviewRequest,
) -> Result<BluezInfoContentChange> {
    let material = find_windows_key_material(action, request)?;
    let key_sections = BluezInfoKeySections::from_windows_key_material(material)?;
    let existing_info_content = find_existing_info(action, request).cloned();
    let next_info_content = bluez_info::merge_key_sections(
        existing_info_content.as_deref().unwrap_or_default(),
        &key_sections,
    );
    let content_changed = existing_info_content
        .as_ref()
        .is_none_or(|existing| existing != &next_info_content);

    Ok(BluezInfoContentChange {
        display_name: action.display_name.clone(),
        linux_adapter_address: action.linux_adapter_address,
        linux_target_device_address: action.linux_target_device_address,
        windows_source_device_address: action.windows_source_device_address,
        target_info_path: target_info_path(
            &request.bluez_dir,
            action.linux_adapter_address,
            action.linux_target_device_address,
        ),
        existing_info_content,
        next_info_content,
        content_changed,
    })
}

fn collect_existing_infos(
    scan_report: &ScanReport,
    plan: &SyncPlan,
) -> Result<Vec<ExistingBluezInfoContent>> {
    plan.actions
        .iter()
        .filter_map(|action| {
            match store::read_device_info_content(
                &scan_report.bluez_dir,
                action.linux_adapter_address,
                action.linux_target_device_address,
            ) {
                Ok(Some(content)) => Some(Ok(ExistingBluezInfoContent {
                    linux_adapter_address: action.linux_adapter_address,
                    linux_device_address: action.linux_target_device_address,
                    content,
                })),
                Ok(None) => None,
                Err(error) => Some(Err(error)),
            }
        })
        .collect()
}

fn collect_windows_key_materials(
    scan_report: &ScanReport,
    plan: &SyncPlan,
) -> Result<Vec<WindowsDeviceKeyMaterial>> {
    plan.actions
        .iter()
        .map(|action| {
            let source = find_windows_device_source(scan_report, action)?;
            let material =
                bthport::read_device_key_material(&source.hive_path, &source.registry_path)?;

            Ok(WindowsDeviceKeyMaterial {
                linux_adapter_address: action.linux_adapter_address,
                windows_device_address: action.windows_source_device_address,
                material,
            })
        })
        .collect()
}

struct WindowsDeviceMaterialSource {
    hive_path: PathBuf,
    registry_path: String,
}

fn find_windows_device_source(
    scan_report: &ScanReport,
    action: &SyncPlanAction,
) -> Result<WindowsDeviceMaterialSource> {
    scan_report
        .windows_bluetooth_keys
        .iter()
        .flat_map(|inspection| {
            inspection.adapters.iter().flat_map(move |adapter| {
                adapter.devices.iter().map(move |device| {
                    (
                        inspection.hive_path.clone(),
                        adapter.adapter_address,
                        device.device_address,
                        device.registry_path.clone(),
                    )
                })
            })
        })
        .find(|(_, adapter_address, device_address, _)| {
            *adapter_address == action.linux_adapter_address
                && *device_address == action.windows_source_device_address
        })
        .map(
            |(hive_path, _, _, registry_path)| WindowsDeviceMaterialSource {
                hive_path,
                registry_path,
            },
        )
        .ok_or(BluebondError::MissingPreviewInput {
            context: "Windows source device registry path",
        })
}

fn backup_entry_for_change(
    change: &BluezInfoContentChange,
    snapshot_root: &Path,
) -> Option<BackupSnapshotEntry> {
    let content = change.existing_info_content.clone()?;

    Some(BackupSnapshotEntry {
        source_path: change.target_info_path.clone(),
        backup_path: snapshot_root
            .join(change.linux_adapter_address.to_string())
            .join(change.linux_target_device_address.to_string())
            .join("info"),
        content,
    })
}

fn find_windows_key_material<'a>(
    action: &SyncPlanAction,
    request: &'a BluezInfoPreviewRequest,
) -> Result<&'a WindowsBluetoothKeyMaterial> {
    request
        .windows_key_materials
        .iter()
        .find(|material| {
            material.linux_adapter_address == action.linux_adapter_address
                && material.windows_device_address == action.windows_source_device_address
        })
        .map(|material| &material.material)
        .ok_or(BluebondError::InvalidRegistryValue {
            context: "Windows Bluetooth key material",
        })
}

fn find_existing_info<'a>(
    action: &SyncPlanAction,
    request: &'a BluezInfoPreviewRequest,
) -> Option<&'a String> {
    request
        .existing_infos
        .iter()
        .find(|info| {
            info.linux_adapter_address == action.linux_adapter_address
                && info.linux_device_address == action.linux_target_device_address
        })
        .map(|info| &info.content)
}

fn target_info_path(
    bluez_dir: &Path,
    adapter_address: BluetoothAddress,
    device_address: BluetoothAddress,
) -> PathBuf {
    bluez_dir
        .join(adapter_address.to_string())
        .join(device_address.to_string())
        .join("info")
}

fn redacted_optional_content(content: &Option<String>) -> &'static str {
    if content.is_some() {
        "<redacted>"
    } else {
        "None"
    }
}
