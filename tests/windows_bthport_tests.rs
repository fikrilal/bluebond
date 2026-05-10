use bluebond::domain::BluetoothAddress;
use bluebond::infra::windows::bthport;

#[test]
fn parses_hivexsh_adapter_key_listing() {
    let adapters = bthport::parse_adapter_key_listing(
        "ControlSet001",
        r"ControlSet001\Services\BTHPORT\Parameters\Keys",
        r#"
f889d28392c0
not-a-key
C6C0FDF1FB80
SYSTEM\>
"#,
    );

    assert_eq!(adapters.len(), 2);
    assert_eq!(adapters[0].adapter_address.to_string(), "C6:C0:FD:F1:FB:80");
    assert_eq!(
        adapters[0].registry_path,
        r"ControlSet001\Services\BTHPORT\Parameters\Keys\c6c0fdf1fb80"
    );
    assert_eq!(adapters[1].adapter_address.to_string(), "F8:89:D2:83:92:C0");
}

#[test]
fn deduplicates_adapter_keys_from_listing() {
    let adapters = bthport::parse_adapter_key_listing(
        "ControlSet001",
        r"ControlSet001\Services\BTHPORT\Parameters\Keys",
        "f889d28392c0\nF889D28392C0\n",
    );

    assert_eq!(adapters.len(), 1);
    assert_eq!(adapters[0].adapter_address.to_string(), "F8:89:D2:83:92:C0");
}

#[test]
fn parses_compact_bluetooth_address() {
    let address = BluetoothAddress::from_compact_hex("f889d28392c0").unwrap();

    assert_eq!(address.to_string(), "F8:89:D2:83:92:C0");
    assert_eq!(address.compact_lower(), "f889d28392c0");
}
