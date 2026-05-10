use std::fs::File;
use std::path::{Path, PathBuf};

use crate::infra::command;

const SYSTEM_HIVE_RELATIVE_PATH: &[&str] = &["Windows", "System32", "config", "SYSTEM"];

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WindowsSystemHiveCandidate {
    pub root: PathBuf,
    pub hive_path: PathBuf,
    pub status: WindowsSystemHiveStatus,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum WindowsSystemHiveStatus {
    Ready,
    Missing,
    NotFile,
    Unreadable,
}

pub fn hivexget_available() -> bool {
    command::exists("hivexget")
}

pub fn hivexsh_available() -> bool {
    command::exists("hivexsh")
}

pub fn discover_candidates() -> Vec<WindowsSystemHiveCandidate> {
    discover_candidates_from_roots(common_windows_roots())
}

pub fn discover_candidates_from_roots(
    roots: impl IntoIterator<Item = PathBuf>,
) -> Vec<WindowsSystemHiveCandidate> {
    roots
        .into_iter()
        .filter_map(|root| {
            let candidate = validate_root(&root);
            if matches!(candidate.status, WindowsSystemHiveStatus::Missing) {
                None
            } else {
                Some(candidate)
            }
        })
        .collect()
}

pub fn validate_root(root: &Path) -> WindowsSystemHiveCandidate {
    let hive_path = system_hive_path(root);
    let status = validate_hive_path(&hive_path);

    WindowsSystemHiveCandidate {
        root: root.to_path_buf(),
        hive_path,
        status,
    }
}

fn validate_hive_path(hive_path: &Path) -> WindowsSystemHiveStatus {
    let Ok(metadata) = hive_path.metadata() else {
        return WindowsSystemHiveStatus::Missing;
    };

    if !metadata.is_file() {
        return WindowsSystemHiveStatus::NotFile;
    }

    match File::open(hive_path) {
        Ok(_) => WindowsSystemHiveStatus::Ready,
        Err(_) => WindowsSystemHiveStatus::Unreadable,
    }
}

fn system_hive_path(root: &Path) -> PathBuf {
    SYSTEM_HIVE_RELATIVE_PATH
        .iter()
        .fold(root.to_path_buf(), |path, segment| path.join(segment))
}

fn common_windows_roots() -> Vec<PathBuf> {
    let mut roots = vec![PathBuf::from("/mnt/windows"), PathBuf::from("/windows")];

    if let Some(user) = std::env::var_os("USER") {
        roots.extend(media_roots(Path::new("/media").join(&user).as_path()));
        roots.extend(media_roots(Path::new("/run/media").join(user).as_path()));
    }

    roots
}

fn media_roots(parent: &Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(parent) else {
        return Vec::new();
    };

    entries
        .filter_map(|entry| match entry {
            Ok(entry) => Some(entry.path()),
            Err(_) => None,
        })
        .filter(|path| path.is_dir())
        .collect()
}
