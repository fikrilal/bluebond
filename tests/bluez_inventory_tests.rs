use std::path::Path;

use bluebond::app::scan::{self, ScanRequest};
use bluebond::infra::bluez::store;

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
