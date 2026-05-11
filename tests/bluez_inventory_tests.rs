use std::path::Path;

use bluebond::app::scan::{self, ScanRequest};
use bluebond::error::BluebondError;
use bluebond::infra::{bluez::store, linux::privileges};

#[test]
fn reads_bluez_adapters_and_device_info() {
    let report = scan::run(&ScanRequest::new(Path::new("tests/fixtures/bluez"))).unwrap();

    assert_eq!(report.adapters.len(), 1);

    let adapter = &report.adapters[0];
    assert_eq!(adapter.address.to_string(), "F8:89:D2:83:92:C0");
    assert_eq!(adapter.devices.len(), 2);

    let device = &adapter.devices[0];
    assert_eq!(device.address.to_string(), "C6:C0:FD:F1:FB:80");
    assert_eq!(device.name.as_deref(), Some("Legion M600 Mouse"));
    assert_eq!(device.alias.as_deref(), Some("Legion M600 Mouse"));
    assert_eq!(device.address_type.as_deref(), Some("static"));
    assert_eq!(device.paired, Some(true));
    assert_eq!(device.trusted, Some(true));
    assert!(device.has_link_key);
    assert!(device.has_long_term_key);
}

#[test]
fn ignores_non_bluez_boolean_values() {
    let report = scan::run(&ScanRequest::new(Path::new("tests/fixtures/bluez"))).unwrap();
    let adapter = &report.adapters[0];
    let device = adapter
        .devices
        .iter()
        .find(|device| device.address.to_string() == "C6:C0:FD:F1:FB:81")
        .unwrap();

    assert_eq!(device.name.as_deref(), Some("Malformed Bool Device"));
    assert_eq!(device.paired, None);
    assert_eq!(device.trusted, None);
    assert!(device.has_link_key);
    assert!(!device.has_long_term_key);
}

#[test]
fn missing_bluez_store_returns_error() {
    let result = store::read_inventory(Path::new("tests/fixtures/bluez-missing"));

    assert!(result.is_err());
}

#[test]
fn reports_readable_bluez_store_readiness() {
    let readiness = store::store_readiness(Path::new("tests/fixtures/bluez"));

    assert_eq!(readiness, store::StoreReadiness::Readable);
    assert!(readiness.is_readable());
}

#[test]
fn reports_missing_bluez_store_readiness() {
    let readiness = store::store_readiness(Path::new("tests/fixtures/bluez-missing"));

    assert_eq!(readiness, store::StoreReadiness::Missing);
    assert!(!readiness.is_readable());
}

#[cfg(unix)]
#[test]
fn permission_denied_bluez_store_returns_actionable_error() {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    let temp = tempfile::tempdir().unwrap();
    let bluez_dir = temp.path().join("bluez");
    fs::create_dir(&bluez_dir).unwrap();
    fs::set_permissions(&bluez_dir, fs::Permissions::from_mode(0o000)).unwrap();

    let readiness = store::store_readiness(&bluez_dir);
    let result = store::read_inventory(&bluez_dir);

    fs::set_permissions(&bluez_dir, fs::Permissions::from_mode(0o700)).unwrap();

    if privileges::running_as_root() {
        return;
    }

    assert_eq!(readiness, store::StoreReadiness::PermissionDenied);
    assert!(matches!(
        result,
        Err(BluebondError::BluezStoreNotReadable { path }) if path == bluez_dir
    ));
}
