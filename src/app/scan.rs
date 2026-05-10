use std::path::{Path, PathBuf};

use crate::domain::BluetoothAdapter;
use crate::error::Result;
use crate::infra::bluez::store;
use crate::infra::windows::bthport;
use crate::infra::windows::system_hive;

pub use crate::infra::windows::bthport::{
    WindowsBluetoothAdapterKey, WindowsBluetoothKeyInspection, WindowsBluetoothKeyInspectionStatus,
};
pub use crate::infra::windows::system_hive::{WindowsSystemHiveCandidate, WindowsSystemHiveStatus};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ScanRequest {
    pub bluez_dir: PathBuf,
    pub windows_root: Option<PathBuf>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ScanReport {
    pub bluez_dir: PathBuf,
    pub adapters: Vec<BluetoothAdapter>,
    pub windows_candidates: Vec<WindowsSystemHiveCandidate>,
    pub windows_bluetooth_keys: Vec<WindowsBluetoothKeyInspection>,
}

impl ScanRequest {
    pub fn new(bluez_dir: impl Into<PathBuf>) -> Self {
        Self {
            bluez_dir: bluez_dir.into(),
            windows_root: None,
        }
    }

    pub fn with_windows_root(mut self, windows_root: impl Into<PathBuf>) -> Self {
        self.windows_root = Some(windows_root.into());
        self
    }
}

pub fn run(request: &ScanRequest) -> Result<ScanReport> {
    let adapters = store::read_inventory(&request.bluez_dir)?;
    let windows_candidates = match &request.windows_root {
        Some(root) => vec![system_hive::validate_root(root)],
        None => system_hive::discover_candidates(),
    };
    let windows_bluetooth_keys = windows_candidates
        .iter()
        .filter(|candidate| matches!(candidate.status, WindowsSystemHiveStatus::Ready))
        .map(|candidate| bthport::inspect_adapter_keys(&candidate.hive_path))
        .collect();

    Ok(ScanReport {
        bluez_dir: request.bluez_dir.clone(),
        adapters,
        windows_candidates,
        windows_bluetooth_keys,
    })
}

pub fn default_request() -> ScanRequest {
    ScanRequest::new(Path::new(store::DEFAULT_BLUEZ_DIR))
}
