use std::path::{Path, PathBuf};

use crate::app::apply::{self, ApplySafetyMetadata};
use crate::domain::BluetoothAddress;
use crate::error::{BluebondError, Result};
use crate::infra::backup;
use crate::infra::bluez::{service, store};
use crate::infra::linux::privileges;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RollbackBackupList {
    pub backups: Vec<RollbackBackupSummary>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RollbackBackupSummary {
    pub metadata_path: PathBuf,
    pub snapshot_id: String,
    pub operation: String,
    pub bluebond_version: String,
    pub changes: Vec<RollbackBackupChangeSummary>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RollbackBackupChangeSummary {
    pub display_name: String,
    pub target_info_path: PathBuf,
    pub backup_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RollbackRestoreRequest {
    pub metadata_path: PathBuf,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RollbackRestoreReport {
    pub metadata_path: PathBuf,
    pub restored_files: Vec<PathBuf>,
    pub verification: RollbackVerificationReport,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RollbackVerificationReport {
    pub checked_devices: Vec<RollbackVerificationDevice>,
    pub manual_reconnect_check: &'static str,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RollbackVerificationDevice {
    pub target_info_path: PathBuf,
    pub linux_adapter_address: BluetoothAddress,
    pub linux_device_address: BluetoothAddress,
    pub found: bool,
}

pub fn list_backups(backup_root: &Path) -> Result<RollbackBackupList> {
    let backups = backup::store::list_metadata_files(backup_root)?
        .into_iter()
        .filter_map(|metadata_path| match read_metadata(&metadata_path) {
            Ok(metadata) => Some(RollbackBackupSummary {
                metadata_path,
                snapshot_id: metadata.snapshot_id,
                operation: metadata.operation,
                bluebond_version: metadata.bluebond_version.to_string(),
                changes: metadata
                    .changes
                    .into_iter()
                    .map(|change| RollbackBackupChangeSummary {
                        display_name: change.display_name,
                        target_info_path: change.target_info_path,
                        backup_path: change.backup_path,
                    })
                    .collect(),
            }),
            Err(_) => None,
        })
        .collect();

    Ok(RollbackBackupList { backups })
}

pub fn default_backup_dir() -> PathBuf {
    PathBuf::from(backup::store::DEFAULT_BACKUP_DIR)
}

pub fn restore_backup(request: &RollbackRestoreRequest) -> Result<RollbackRestoreReport> {
    require_privileged_rollback()?;
    let metadata = read_metadata(&request.metadata_path)?;

    service::stop_bluetooth_service()?;
    let restore_result = restore_metadata_files(&metadata);
    let start_result = service::start_bluetooth_service();

    let restored_files = restore_result?;

    if let Err(error) = start_result {
        return Err(BluebondError::BluetoothServiceStartFailed {
            recovery: apply::BLUETOOTH_SERVICE_RECOVERY,
            detail: error.to_string(),
        });
    }

    let verification = verify_restored_state(&metadata)?;

    Ok(RollbackRestoreReport {
        metadata_path: request.metadata_path.clone(),
        restored_files,
        verification,
    })
}

pub fn restore_metadata_files(metadata: &ApplySafetyMetadata) -> Result<Vec<PathBuf>> {
    let mut restored_files = Vec::new();

    for change in &metadata.changes {
        let backup_path =
            change
                .backup_path
                .as_ref()
                .ok_or(BluebondError::MissingPreviewInput {
                    context: "rollback backup path",
                })?;
        let content = std::fs::read_to_string(backup_path).map_err(|source| BluebondError::Io {
            context: "reading rollback backup file",
            source,
        })?;

        store::write_device_info_atomic(&change.target_info_path, &content)?;
        restored_files.push(change.target_info_path.clone());
    }

    Ok(restored_files)
}

pub fn verify_restored_state(metadata: &ApplySafetyMetadata) -> Result<RollbackVerificationReport> {
    let bluez_root = infer_bluez_root(metadata)?;
    let inventory = store::read_inventory(&bluez_root)?;
    let checked_devices = metadata
        .changes
        .iter()
        .filter_map(|change| {
            let adapter_address = change.linux_adapter_address.parse().ok()?;
            let device_address = change.linux_target_device_address.parse().ok()?;
            let found = inventory
                .iter()
                .find(|adapter| adapter.address == adapter_address)
                .and_then(|adapter| {
                    adapter
                        .devices
                        .iter()
                        .find(|device| device.address == device_address)
                })
                .is_some();

            Some(RollbackVerificationDevice {
                target_info_path: change.target_info_path.clone(),
                linux_adapter_address: adapter_address,
                linux_device_address: device_address,
                found,
            })
        })
        .collect();

    Ok(RollbackVerificationReport {
        checked_devices,
        manual_reconnect_check:
            "reconnect the Bluetooth device and confirm the restored state behaves as expected",
    })
}

impl RollbackVerificationReport {
    pub fn all_targets_visible(&self) -> bool {
        self.checked_devices.iter().all(|device| device.found)
    }
}

pub fn require_privileged_rollback() -> Result<()> {
    require_privileged_rollback_with(privileges::running_as_root())
}

pub fn require_privileged_rollback_with(running_as_root: bool) -> Result<()> {
    if running_as_root {
        Ok(())
    } else {
        Err(BluebondError::PrivilegeRequired {
            operation: "rollback",
        })
    }
}

pub fn read_metadata(metadata_path: &Path) -> Result<ApplySafetyMetadata> {
    let metadata_json = backup::store::read_metadata(metadata_path)?;
    serde_json::from_str(&metadata_json).map_err(|source| BluebondError::Serialization {
        context: "backup metadata",
        source,
    })
}

fn infer_bluez_root(metadata: &ApplySafetyMetadata) -> Result<PathBuf> {
    let first_target = metadata
        .changes
        .first()
        .ok_or(BluebondError::MissingPreviewInput {
            context: "rollback metadata target path",
        })?
        .target_info_path
        .clone();

    first_target
        .ancestors()
        .nth(3)
        .map(Path::to_path_buf)
        .ok_or(BluebondError::MissingPreviewInput {
            context: "rollback BlueZ root",
        })
}
