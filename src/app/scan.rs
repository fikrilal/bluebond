use std::path::{Path, PathBuf};

use crate::domain::BluetoothAdapter;
use crate::error::Result;
use crate::infra::bluez::store;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ScanRequest {
    pub bluez_dir: PathBuf,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ScanReport {
    pub bluez_dir: PathBuf,
    pub adapters: Vec<BluetoothAdapter>,
}

impl ScanRequest {
    pub fn new(bluez_dir: impl Into<PathBuf>) -> Self {
        Self {
            bluez_dir: bluez_dir.into(),
        }
    }
}

pub fn run(request: &ScanRequest) -> Result<ScanReport> {
    let adapters = store::read_inventory(&request.bluez_dir)?;

    Ok(ScanReport {
        bluez_dir: request.bluez_dir.clone(),
        adapters,
    })
}

pub fn default_request() -> ScanRequest {
    ScanRequest::new(Path::new(store::DEFAULT_BLUEZ_DIR))
}
