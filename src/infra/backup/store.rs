use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::{BluebondError, Result};

pub const DEFAULT_BACKUP_DIR: &str = "/var/lib/bluetooth-backups";
pub const METADATA_FILE_NAME: &str = "bluebond-backup.json";

pub fn timestamped_snapshot_id() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default();

    format!("unix-{seconds}")
}

pub fn snapshot_root(base_dir: &Path, snapshot_id: &str) -> PathBuf {
    base_dir.join(snapshot_id)
}

pub fn write_backup_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| BluebondError::Io {
            context: "creating backup directory",
            source,
        })?;
    }

    fs::write(path, content).map_err(|source| BluebondError::Io {
        context: "writing backup file",
        source,
    })
}

pub fn write_metadata(snapshot_root: &Path, metadata_json: &str) -> Result<PathBuf> {
    fs::create_dir_all(snapshot_root).map_err(|source| BluebondError::Io {
        context: "creating backup metadata directory",
        source,
    })?;

    let metadata_path = snapshot_root.join(METADATA_FILE_NAME);
    fs::write(&metadata_path, metadata_json).map_err(|source| BluebondError::Io {
        context: "writing backup metadata",
        source,
    })?;

    Ok(metadata_path)
}
