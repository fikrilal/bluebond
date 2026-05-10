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
    assert!(adapters[0].devices.is_empty());
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

#[test]
fn parses_hivexsh_device_key_listing() {
    let devices = bthport::parse_device_key_listing(
        r"ControlSet001\Services\BTHPORT\Parameters\Keys\f889d28392c0",
        r#"
c6c0f8f1fb80
not-a-device
C6C0FDF1FB80
SYSTEM\>
"#,
    );

    assert_eq!(devices.len(), 2);
    assert_eq!(devices[0].device_address.to_string(), "C6:C0:F8:F1:FB:80");
    assert_eq!(
        devices[0].registry_path,
        r"ControlSet001\Services\BTHPORT\Parameters\Keys\f889d28392c0\c6c0f8f1fb80"
    );
    assert!(!devices[0].has_key_material);
    assert_eq!(devices[1].device_address.to_string(), "C6:C0:FD:F1:FB:80");
}

#[test]
fn detects_windows_bluetooth_key_material_values() {
    assert!(bthport::parse_key_material_presence(
        r#"
"LTK"=hex(3):85,35,da,32,4d,e7,81,df,a7,46,c4,bd,58,bb,a0,8c
"KeyLength"=dword:00000010
"IRK"=hex(3):0d,3b,63,ea,0f,ce,9a,74,fe,82,ea,8e,ac,58,88,87
"#,
    ));

    assert!(!bthport::parse_key_material_presence(
        r#"
"Address"=hex(11):80,fb,f1,fd,c0,c6,00,00
"AddressType"=dword:00000000
"#,
    ));
}
