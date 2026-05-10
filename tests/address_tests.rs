use bluebond::domain::BluetoothAddress;

#[test]
fn parses_public_adapter_address() {
    let address: BluetoothAddress = "F8:89:D2:83:92:C0".parse().unwrap();

    assert_eq!(address.to_string(), "F8:89:D2:83:92:C0");
}
