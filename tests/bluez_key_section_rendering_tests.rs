use bluebond::convert::windows_key_material;
use bluebond::convert::windows_to_bluez::BluezInfoKeySections;

#[test]
fn renders_windows_key_material_as_bluez_info_sections() {
    let material = windows_key_material::parse_hivexsh_lsval_output(
        r#"
"LTK"=hex(3):00,11,22,33,44,55,66,77,88,99,aa,bb,cc,dd,ee,ff
"KeyLength"=dword:00000010
"ERand"=hex(11):10,20,30,40,50,60,70,80
"EDIV"=dword:000071b9
"IRK"=hex(3):ff,ee,dd,cc,bb,aa,99,88,77,66,55,44,33,22,11,00
"CSRK"=hex(3):12,34,56,78,90,ab,cd,ef,fe,dc,ba,09,87,65,43,21
"AuthReq"=dword:0000002d
"#,
    )
    .unwrap();

    let sections = BluezInfoKeySections::from_windows_key_material(&material).unwrap();

    assert_eq!(
        sections.render(),
        "\
[IdentityResolvingKey]
Key=FFEEDDCCBBAA99887766554433221100

[LocalSignatureKey]
Key=1234567890ABCDEFFEDCBA0987654321
Counter=0
Authenticated=true

[LongTermKey]
Key=00112233445566778899AABBCCDDEEFF
Authenticated=true
EncSize=16
EDiv=29113
Rand=9255003132036915216
"
    );
}

#[test]
fn renders_available_sections_without_ltk_metadata() {
    let material = windows_key_material::parse_hivexsh_lsval_output(
        r#"
"IRK"=hex(3):ff,ee,dd,cc,bb,aa,99,88,77,66,55,44,33,22,11,00
"#,
    )
    .unwrap();

    let sections = BluezInfoKeySections::from_windows_key_material(&material).unwrap();

    assert_eq!(
        sections.render(),
        "\
[IdentityResolvingKey]
Key=FFEEDDCCBBAA99887766554433221100
"
    );
}

#[test]
fn rejects_ltk_with_malformed_erand_length() {
    let material = windows_key_material::parse_hivexsh_lsval_output(
        r#"
"LTK"=hex(3):00,11,22,33,44,55,66,77,88,99,aa,bb,cc,dd,ee,ff
"ERand"=hex(11):10,20,30
"EDIV"=dword:000071b9
"#,
    )
    .unwrap();

    let result = BluezInfoKeySections::from_windows_key_material(&material);

    assert!(result.is_err());
}

#[test]
fn debug_redacts_rendered_key_material() {
    let material = windows_key_material::parse_hivexsh_lsval_output(
        r#"
"LTK"=hex(3):00,11,22,33,44,55,66,77,88,99,aa,bb,cc,dd,ee,ff
"KeyLength"=dword:00000010
"ERand"=hex(11):10,20,30,40,50,60,70,80
"EDIV"=dword:000071b9
"IRK"=hex(3):ff,ee,dd,cc,bb,aa,99,88,77,66,55,44,33,22,11,00
"CSRK"=hex(3):12,34,56,78,90,ab,cd,ef,fe,dc,ba,09,87,65,43,21
"#,
    )
    .unwrap();

    let sections = BluezInfoKeySections::from_windows_key_material(&material).unwrap();
    let debug = format!("{sections:?}");

    assert!(debug.contains("<redacted>"));
    assert!(!debug.contains("00112233445566778899AABBCCDDEEFF"));
    assert!(!debug.contains("FFEEDDCCBBAA99887766554433221100"));
    assert!(!debug.contains("1234567890ABCDEFFEDCBA0987654321"));
}
