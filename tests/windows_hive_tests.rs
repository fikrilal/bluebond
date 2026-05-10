use std::path::Path;

use bluebond::app::scan::{self, ScanRequest, WindowsSystemHiveStatus};
use bluebond::infra::windows::system_hive;

#[test]
fn validates_explicit_windows_root_with_system_hive() {
    let report = scan::run(
        &ScanRequest::new(Path::new("tests/fixtures/bluez"))
            .with_windows_root(Path::new("tests/fixtures/windows")),
    )
    .unwrap();

    assert_eq!(report.windows_candidates.len(), 1);

    let candidate = &report.windows_candidates[0];
    assert_eq!(candidate.root, Path::new("tests/fixtures/windows"));
    assert_eq!(
        candidate.hive_path,
        Path::new("tests/fixtures/windows/Windows/System32/config/SYSTEM")
    );
    assert_eq!(candidate.status, WindowsSystemHiveStatus::Ready);
}

#[test]
fn reports_missing_system_hive_for_explicit_root() {
    let report = scan::run(
        &ScanRequest::new(Path::new("tests/fixtures/bluez"))
            .with_windows_root(Path::new("tests/fixtures/windows-missing")),
    )
    .unwrap();

    assert_eq!(report.windows_candidates.len(), 1);
    assert_eq!(
        report.windows_candidates[0].status,
        WindowsSystemHiveStatus::Missing
    );
}

#[test]
fn reports_system_hive_path_that_is_not_a_file() {
    let report = scan::run(
        &ScanRequest::new(Path::new("tests/fixtures/bluez"))
            .with_windows_root(Path::new("tests/fixtures/windows-notfile")),
    )
    .unwrap();

    assert_eq!(report.windows_candidates.len(), 1);
    assert_eq!(
        report.windows_candidates[0].status,
        WindowsSystemHiveStatus::NotFile
    );
}

#[test]
fn discovery_returns_non_missing_candidates_from_roots() {
    let candidates = system_hive::discover_candidates_from_roots([
        Path::new("tests/fixtures/windows-missing").to_path_buf(),
        Path::new("tests/fixtures/windows").to_path_buf(),
        Path::new("tests/fixtures/windows-notfile").to_path_buf(),
    ]);

    assert_eq!(candidates.len(), 2);
    assert_eq!(candidates[0].root, Path::new("tests/fixtures/windows"));
    assert_eq!(
        candidates[0].status,
        system_hive::WindowsSystemHiveStatus::Ready
    );
    assert_eq!(
        candidates[1].root,
        Path::new("tests/fixtures/windows-notfile")
    );
    assert_eq!(
        candidates[1].status,
        system_hive::WindowsSystemHiveStatus::NotFile
    );
}

#[test]
fn automatic_discovery_does_not_return_missing_candidates() {
    let candidates = system_hive::discover_candidates();

    assert!(candidates.iter().all(|candidate| !matches!(
        candidate.status,
        system_hive::WindowsSystemHiveStatus::Missing
    )));
}
