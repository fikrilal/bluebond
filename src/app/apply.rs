use std::fmt;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::app::plan;
use crate::app::scan::ScanReport;
use crate::convert::bluez_info;
use crate::convert::windows_key_material::WindowsBluetoothKeyMaterial;
use crate::convert::windows_to_bluez::BluezInfoKeySections;
use crate::domain::{BluetoothAddress, SyncPlan, SyncPlanAction, SyncPlanActionType};
use crate::error::{BluebondError, Result};
use crate::infra::backup;
use crate::infra::bluez::{service, store};
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
    pub windows_source_registry_path: Option<String>,
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
    pub manual_selection: Option<ManualApplySelection>,
}

impl ApplyDryRunRequest {
    pub fn new(backup_root: impl Into<PathBuf>) -> Self {
        Self {
            backup_root: backup_root.into(),
            manual_selection: None,
        }
    }

    pub fn with_manual_selection(mut self, manual_selection: ManualApplySelection) -> Self {
        self.manual_selection = Some(manual_selection);
        self
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ApplyExecuteRequest {
    pub backup_base_dir: PathBuf,
    pub manual_selection: Option<ManualApplySelection>,
}

impl ApplyExecuteRequest {
    pub fn new(backup_base_dir: impl Into<PathBuf>) -> Self {
        Self {
            backup_base_dir: backup_base_dir.into(),
            manual_selection: None,
        }
    }

    pub fn with_manual_selection(mut self, manual_selection: ManualApplySelection) -> Self {
        self.manual_selection = Some(manual_selection);
        self
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ApplyExecuteReport {
    pub backup: WrittenBackupSnapshot,
    pub bluez_writes: WrittenBluezInfoRecords,
    pub service: BluetoothServiceRestartReport,
    pub verification: ApplyVerificationReport,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ManualApplySelection {
    pub adapter_address: Option<BluetoothAddress>,
    pub target_device_address: BluetoothAddress,
    pub windows_source_device_address: BluetoothAddress,
    pub target_mode: ManualApplyTargetMode,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ManualApplyTargetMode {
    LinuxTarget,
    WindowsSource,
}

impl ManualApplySelection {
    pub fn from_raw(
        adapter_address: Option<&str>,
        target_device_address: &str,
        windows_source_device_address: &str,
    ) -> Result<Self> {
        Self::from_raw_with_target_mode(
            adapter_address,
            target_device_address,
            windows_source_device_address,
            ManualApplyTargetMode::LinuxTarget,
        )
    }

    pub fn from_raw_with_target_mode(
        adapter_address: Option<&str>,
        target_device_address: &str,
        windows_source_device_address: &str,
        target_mode: ManualApplyTargetMode,
    ) -> Result<Self> {
        Ok(Self {
            adapter_address: adapter_address
                .map(str::parse::<BluetoothAddress>)
                .transpose()?,
            target_device_address: target_device_address.parse()?,
            windows_source_device_address: windows_source_device_address.parse()?,
            target_mode,
        })
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
    pub registry_path: Option<String>,
    pub material: WindowsBluetoothKeyMaterial,
}

impl fmt::Debug for WindowsDeviceKeyMaterial {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WindowsDeviceKeyMaterial")
            .field("linux_adapter_address", &self.linux_adapter_address)
            .field("windows_device_address", &self.windows_device_address)
            .field("registry_path", &self.registry_path)
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
    let sync_plan = build_apply_sync_plan(scan_report, request.manual_selection.as_ref())?;
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

pub fn default_execute_request() -> ApplyExecuteRequest {
    ApplyExecuteRequest::new(Path::new(backup::store::DEFAULT_BACKUP_DIR))
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

pub fn execute_apply(
    scan_report: &ScanReport,
    request: &ApplyExecuteRequest,
) -> Result<ApplyExecuteReport> {
    require_privileged_apply()?;

    let sync_plan = build_apply_sync_plan(scan_report, request.manual_selection.as_ref())?;
    let content_preview = preview_from_scan_report(scan_report, &sync_plan)?;
    let backup_snapshot =
        build_timestamped_backup_snapshot(&content_preview, &request.backup_base_dir);
    let metadata = build_safety_metadata(&backup_snapshot, &content_preview, "apply");
    let backup = write_backup_snapshot(&backup_snapshot, &metadata)?;

    service::stop_bluetooth_service()?;
    let write_result = write_bluez_info_records(&content_preview);
    let start_result = service::start_bluetooth_service();

    let bluez_writes = write_result?;

    if let Err(error) = start_result {
        return Err(BluebondError::BluetoothServiceStartFailed {
            recovery: BLUETOOTH_SERVICE_RECOVERY,
            detail: error.to_string(),
        });
    }

    let verification = verify_post_apply_state(&scan_report.bluez_dir, &content_preview)?;

    Ok(ApplyExecuteReport {
        backup,
        bluez_writes,
        service: BluetoothServiceRestartReport {
            stopped: true,
            started: true,
            recovery_instructions: None,
        },
        verification,
    })
}

pub fn build_apply_sync_plan(
    scan_report: &ScanReport,
    manual_selection: Option<&ManualApplySelection>,
) -> Result<SyncPlan> {
    match manual_selection {
        Some(selection) => build_manual_sync_plan(scan_report, selection),
        None => Ok(plan::build_sync_plan(scan_report)),
    }
}

pub fn build_manual_sync_plan(
    scan_report: &ScanReport,
    selection: &ManualApplySelection,
) -> Result<SyncPlan> {
    let (linux_adapter_address, display_name) = find_manual_linux_target(scan_report, selection)?;
    validate_manual_windows_source(scan_report, linux_adapter_address, selection)?;

    Ok(SyncPlan {
        actions: vec![SyncPlanAction {
            action_type: match selection.target_mode {
                ManualApplyTargetMode::LinuxTarget => SyncPlanActionType::UpdateExistingBluezRecord,
                ManualApplyTargetMode::WindowsSource => SyncPlanActionType::CreateBluezRecord,
            },
            linux_adapter_address,
            linux_target_device_address: match selection.target_mode {
                ManualApplyTargetMode::LinuxTarget => selection.target_device_address,
                ManualApplyTargetMode::WindowsSource => selection.windows_source_device_address,
            },
            bluez_template_device_address: match selection.target_mode {
                ManualApplyTargetMode::LinuxTarget => None,
                ManualApplyTargetMode::WindowsSource => Some(selection.target_device_address),
            },
            windows_source_device_address: selection.windows_source_device_address,
            display_name,
        }],
        skipped: Vec::new(),
    })
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WrittenBackupSnapshot {
    pub root_dir: PathBuf,
    pub metadata_path: PathBuf,
    pub files_written: Vec<PathBuf>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WrittenBluezInfoRecords {
    pub files_written: Vec<PathBuf>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BluetoothServiceRestartReport {
    pub stopped: bool,
    pub started: bool,
    pub recovery_instructions: Option<&'static str>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ApplyVerificationReport {
    pub checked_devices: Vec<ApplyVerificationDevice>,
    pub manual_reconnect_check: &'static str,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ApplyVerificationDevice {
    pub linux_adapter_address: BluetoothAddress,
    pub linux_device_address: BluetoothAddress,
    pub target_info_path: PathBuf,
    pub found: bool,
    pub expected_long_term_key: bool,
    pub has_long_term_key: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct ApplySafetyMetadata {
    pub schema_version: u32,
    pub bluebond_version: String,
    pub operation: String,
    pub snapshot_id: String,
    pub backup_root: PathBuf,
    pub changes: Vec<ApplySafetyMetadataChange>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct ApplySafetyMetadataChange {
    pub display_name: String,
    pub linux_adapter_address: String,
    pub linux_target_device_address: String,
    pub windows_source_device_address: String,
    pub windows_source_registry_path: Option<String>,
    pub target_info_path: PathBuf,
    pub backup_path: Option<PathBuf>,
    pub content_changed: bool,
}

pub fn backup_snapshot_root(base_dir: &Path, snapshot_id: &str) -> PathBuf {
    backup::store::snapshot_root(base_dir, snapshot_id)
}

pub fn build_timestamped_backup_snapshot(
    preview: &BluezInfoContentPreview,
    backup_base_dir: &Path,
) -> BackupSnapshot {
    let snapshot_id = backup::store::timestamped_snapshot_id();
    build_backup_snapshot(preview, backup_snapshot_root(backup_base_dir, &snapshot_id))
}

pub fn build_safety_metadata(
    snapshot: &BackupSnapshot,
    preview: &BluezInfoContentPreview,
    operation: impl Into<String>,
) -> ApplySafetyMetadata {
    let snapshot_id = snapshot
        .root_dir
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("unknown")
        .to_string();

    ApplySafetyMetadata {
        schema_version: 1,
        bluebond_version: env!("CARGO_PKG_VERSION").to_string(),
        operation: operation.into(),
        snapshot_id,
        backup_root: snapshot.root_dir.clone(),
        changes: preview
            .changes
            .iter()
            .map(|change| ApplySafetyMetadataChange {
                display_name: change.display_name.clone(),
                linux_adapter_address: change.linux_adapter_address.to_string(),
                linux_target_device_address: change.linux_target_device_address.to_string(),
                windows_source_device_address: change.windows_source_device_address.to_string(),
                windows_source_registry_path: change.windows_source_registry_path.clone(),
                target_info_path: change.target_info_path.clone(),
                backup_path: snapshot
                    .entries
                    .iter()
                    .find(|entry| entry.source_path == change.target_info_path)
                    .map(|entry| entry.backup_path.clone()),
                content_changed: change.content_changed,
            })
            .collect(),
    }
}

pub fn write_backup_snapshot(
    snapshot: &BackupSnapshot,
    metadata: &ApplySafetyMetadata,
) -> Result<WrittenBackupSnapshot> {
    let mut files_written = Vec::new();

    for entry in &snapshot.entries {
        backup::store::write_backup_file(&entry.backup_path, &entry.content)?;
        files_written.push(entry.backup_path.clone());
    }

    let metadata_json =
        serde_json::to_string_pretty(metadata).map_err(|source| BluebondError::Serialization {
            context: "backup metadata",
            source,
        })?;
    let metadata_path = backup::store::write_metadata(&snapshot.root_dir, &metadata_json)?;

    Ok(WrittenBackupSnapshot {
        root_dir: snapshot.root_dir.clone(),
        metadata_path,
        files_written,
    })
}

pub fn write_bluez_info_records(
    preview: &BluezInfoContentPreview,
) -> Result<WrittenBluezInfoRecords> {
    let mut files_written = Vec::new();

    for change in &preview.changes {
        if !change.content_changed {
            continue;
        }

        store::write_device_info_atomic(&change.target_info_path, &change.next_info_content)?;
        files_written.push(change.target_info_path.clone());
    }

    Ok(WrittenBluezInfoRecords { files_written })
}

pub fn restart_bluetooth_service() -> Result<BluetoothServiceRestartReport> {
    service::stop_bluetooth_service()?;

    match service::start_bluetooth_service() {
        Ok(()) => Ok(BluetoothServiceRestartReport {
            stopped: true,
            started: true,
            recovery_instructions: None,
        }),
        Err(error) => Err(BluebondError::BluetoothServiceStartFailed {
            recovery: BLUETOOTH_SERVICE_RECOVERY,
            detail: error.to_string(),
        }),
    }
}

pub const BLUETOOTH_SERVICE_RECOVERY: &str =
    "run `sudo systemctl start bluetooth.service` and restore from the BlueBond backup if needed";

pub fn bluetooth_service_report_from_outcomes(
    stop_ok: bool,
    start_ok: bool,
) -> Result<BluetoothServiceRestartReport> {
    if !stop_ok {
        return Err(BluebondError::CommandFailed {
            program: "systemctl".to_string(),
            status: Some(1),
            stderr: "failed to stop bluetooth.service".to_string(),
        });
    }

    if !start_ok {
        return Err(BluebondError::BluetoothServiceStartFailed {
            recovery: BLUETOOTH_SERVICE_RECOVERY,
            detail: "failed to start bluetooth.service".to_string(),
        });
    }

    Ok(BluetoothServiceRestartReport {
        stopped: true,
        started: true,
        recovery_instructions: None,
    })
}

pub fn verify_post_apply_state(
    bluez_dir: &Path,
    preview: &BluezInfoContentPreview,
) -> Result<ApplyVerificationReport> {
    let inventory = store::read_inventory(bluez_dir)?;
    let checked_devices = preview
        .changes
        .iter()
        .map(|change| {
            let device = inventory
                .iter()
                .find(|adapter| adapter.address == change.linux_adapter_address)
                .and_then(|adapter| {
                    adapter
                        .devices
                        .iter()
                        .find(|device| device.address == change.linux_target_device_address)
                });
            let expected_long_term_key = change.next_info_content.contains("[LongTermKey]");

            ApplyVerificationDevice {
                linux_adapter_address: change.linux_adapter_address,
                linux_device_address: change.linux_target_device_address,
                target_info_path: change.target_info_path.clone(),
                found: device.is_some(),
                expected_long_term_key,
                has_long_term_key: device.is_some_and(|device| device.has_long_term_key),
            }
        })
        .collect();

    Ok(ApplyVerificationReport {
        checked_devices,
        manual_reconnect_check:
            "reconnect the Bluetooth device and confirm it pairs without re-pairing",
    })
}

impl ApplyVerificationReport {
    pub fn all_expected_records_visible(&self) -> bool {
        self.checked_devices.iter().all(|device| {
            device.found && (device.has_long_term_key || !device.expected_long_term_key)
        })
    }
}

fn preview_action(
    action: &SyncPlanAction,
    request: &BluezInfoPreviewRequest,
) -> Result<BluezInfoContentChange> {
    let material = find_windows_key_material(action, request)?;
    let key_sections = BluezInfoKeySections::from_windows_key_material(material)?;
    let existing_info_content =
        find_existing_info_for_device(action, request, action.linux_target_device_address).cloned();
    let base_info_content = existing_info_content.clone().or_else(|| {
        action
            .bluez_template_device_address
            .and_then(|device_address| {
                find_existing_info_for_device(action, request, device_address).cloned()
            })
    });
    let next_info_content = bluez_info::merge_key_sections(
        base_info_content.as_deref().unwrap_or_default(),
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
        windows_source_registry_path: find_windows_key_material_entry(action, request)
            .and_then(|material| material.registry_path.clone()),
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

fn find_manual_linux_target(
    scan_report: &ScanReport,
    selection: &ManualApplySelection,
) -> Result<(BluetoothAddress, String)> {
    let matches = scan_report
        .adapters
        .iter()
        .filter(|adapter| {
            selection
                .adapter_address
                .is_none_or(|selected| selected == adapter.address)
        })
        .filter_map(|adapter| {
            adapter
                .devices
                .iter()
                .find(|device| device.address == selection.target_device_address)
                .map(|device| (adapter.address, device.display_name().to_string()))
        })
        .collect::<Vec<_>>();

    match matches.as_slice() {
        [] => Err(BluebondError::MissingPreviewInput {
            context: "manual Linux target device",
        }),
        [single] => Ok(single.clone()),
        _ => Err(BluebondError::AmbiguousPreviewInput {
            context: "manual Linux target device adapter",
        }),
    }
}

fn validate_manual_windows_source(
    scan_report: &ScanReport,
    linux_adapter_address: BluetoothAddress,
    selection: &ManualApplySelection,
) -> Result<()> {
    let source = scan_report
        .windows_bluetooth_keys
        .iter()
        .flat_map(|inspection| inspection.adapters.iter())
        .filter(|adapter| adapter.adapter_address == linux_adapter_address)
        .flat_map(|adapter| adapter.devices.iter())
        .find(|device| device.device_address == selection.windows_source_device_address)
        .ok_or(BluebondError::MissingPreviewInput {
            context: "manual Windows source device",
        })?;

    if source.has_key_material {
        Ok(())
    } else {
        Err(BluebondError::MissingPreviewInput {
            context: "manual Windows source key material",
        })
    }
}

fn collect_existing_infos(
    scan_report: &ScanReport,
    plan: &SyncPlan,
) -> Result<Vec<ExistingBluezInfoContent>> {
    let targets = plan
        .actions
        .iter()
        .flat_map(|action| {
            [
                action.linux_target_device_address,
                action
                    .bluez_template_device_address
                    .unwrap_or(action.linux_target_device_address),
            ]
            .into_iter()
            .map(move |device_address| (action.linux_adapter_address, device_address))
        })
        .collect::<std::collections::BTreeSet<_>>();

    targets
        .into_iter()
        .filter_map(|(adapter_address, device_address)| {
            match store::read_device_info_content(
                &scan_report.bluez_dir,
                adapter_address,
                device_address,
            ) {
                Ok(Some(content)) => Some(Ok(ExistingBluezInfoContent {
                    linux_adapter_address: adapter_address,
                    linux_device_address: device_address,
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
                registry_path: Some(source.registry_path),
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
    find_windows_key_material_entry(action, request)
        .map(|material| &material.material)
        .ok_or(BluebondError::InvalidRegistryValue {
            context: "Windows Bluetooth key material",
        })
}

fn find_windows_key_material_entry<'a>(
    action: &SyncPlanAction,
    request: &'a BluezInfoPreviewRequest,
) -> Option<&'a WindowsDeviceKeyMaterial> {
    request.windows_key_materials.iter().find(|material| {
        material.linux_adapter_address == action.linux_adapter_address
            && material.windows_device_address == action.windows_source_device_address
    })
}

fn find_existing_info_for_device<'a>(
    action: &SyncPlanAction,
    request: &'a BluezInfoPreviewRequest,
    device_address: BluetoothAddress,
) -> Option<&'a String> {
    request.existing_infos.iter().find_map(|info| {
        (info.linux_adapter_address == action.linux_adapter_address
            && info.linux_device_address == device_address)
            .then_some(&info.content)
    })
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
