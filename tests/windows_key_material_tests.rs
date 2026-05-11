use bluebond::convert::windows_key_material;

#[test]
fn parses_windows_bluetooth_key_material_from_hivexsh_lsval_output() {
    let material = windows_key_material::parse_hivexsh_lsval_output(
        r#"
"LTK"=hex(3):00,11,22,33,44,55,66,77,88,99,aa,bb,cc,dd,ee,ff
"KeyLength"=dword:00000010
"ERand"=hex(11):10,20,30,40,50,60,70,80
"EDIV"=dword:000071b9
"IRK"=hex(3):ff,ee,dd,cc,bb,aa,99,88,77,66,55,44,33,22,11,00
"Address"=hex(11):80,fb,f1,fd,c0,c6,00,00
"AddressType"=dword:00000000
"CSRK"=hex(3):12,34,56,78,90,ab,cd,ef,fe,dc,ba,09,87,65,43,21
"AuthReq"=dword:0000002d
"#,
    )
    .unwrap();

    assert_eq!(
        material.ltk.unwrap(),
        vec![
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]
    );
    assert_eq!(
        material.irk.unwrap(),
        vec![
            0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22,
            0x11, 0x00,
        ]
    );
    assert_eq!(
        material.csrk.unwrap(),
        vec![
            0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x09, 0x87, 0x65,
            0x43, 0x21,
        ]
    );
    assert_eq!(
        material.erand.unwrap(),
        vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80]
    );
    assert_eq!(material.ediv, Some(0x71b9));
    assert_eq!(material.address.unwrap().to_string(), "C6:C0:FD:F1:FB:80");
    assert_eq!(material.address_type, Some(0));
    assert_eq!(material.key_length, Some(16));
    assert_eq!(material.auth_req, Some(0x2d));
}

#[test]
fn reports_core_key_material_presence() {
    let material = windows_key_material::parse_hivexsh_lsval_output(
        r#"
"LTK"=hex(3):00,11,22,33,44,55,66,77,88,99,aa,bb,cc,dd,ee,ff
"Address"=hex(11):80,fb,f1,fd,c0,c6,00,00
"#,
    )
    .unwrap();

    assert!(material.has_core_key_material());
}

#[test]
fn parses_missing_values_as_none() {
    let material = windows_key_material::parse_hivexsh_lsval_output(
        r#"
"AddressType"=dword:00000001
"#,
    )
    .unwrap();

    assert_eq!(material.ltk, None);
    assert_eq!(material.address, None);
    assert_eq!(material.address_type, Some(1));
    assert!(!material.has_core_key_material());
}

#[test]
fn rejects_malformed_dword() {
    let result = windows_key_material::parse_hivexsh_lsval_output(
        r#"
"EDIV"=dword:nothex
"#,
    );

    assert!(result.is_err());
}

#[test]
fn rejects_short_windows_address() {
    let result = windows_key_material::parse_hivexsh_lsval_output(
        r#"
"Address"=hex(11):80,fb,f1
"#,
    );

    assert!(result.is_err());
}

#[test]
fn debug_redacts_key_material_bytes() {
    let material = windows_key_material::parse_hivexsh_lsval_output(
        r#"
"LTK"=hex(3):00,11,22,33,44,55,66,77,88,99,aa,bb,cc,dd,ee,ff
"IRK"=hex(3):ff,ee,dd,cc,bb,aa,99,88,77,66,55,44,33,22,11,00
"CSRK"=hex(3):12,34,56,78,90,ab,cd,ef,fe,dc,ba,09,87,65,43,21
"Address"=hex(11):80,fb,f1,fd,c0,c6,00,00
"#,
    )
    .unwrap();

    let debug = format!("{material:?}");

    assert!(debug.contains("<redacted>"));
    assert!(!debug.contains("00, 11, 22"));
    assert!(!debug.contains("ff, ee, dd"));
    assert!(!debug.contains("12, 34, 56"));
}
