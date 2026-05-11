use bluebond::convert::bluez_info;
use bluebond::convert::windows_key_material;
use bluebond::convert::windows_to_bluez::BluezInfoKeySections;

#[test]
fn replaces_existing_managed_key_sections_and_preserves_unmanaged_sections() {
    let existing_info = "\
[General]
Name=Legion M600 Mouse
Trusted=true

[IdentityResolvingKey]
Key=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA

[LinkKey]
Key=00112233445566778899AABBCCDDEEFF
Type=4

[LocalSignatureKey]
Key=BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB
Counter=7
Authenticated=false

[LongTermKey]
Key=CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC
Authenticated=false
EncSize=16
EDiv=1
Rand=2
";
    let key_sections = rendered_key_sections();

    let merged = bluez_info::merge_key_sections(existing_info, &key_sections);

    assert_eq!(
        merged,
        "\
[General]
Name=Legion M600 Mouse
Trusted=true

[LinkKey]
Key=00112233445566778899AABBCCDDEEFF
Type=4

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
fn appends_rendered_key_sections_when_missing() {
    let existing_info = "\
[General]
Name=Legion M600 Mouse
Trusted=true
";
    let key_sections = rendered_key_sections();

    let merged = bluez_info::merge_key_sections(existing_info, &key_sections);

    assert!(merged.starts_with("[General]\nName=Legion M600 Mouse\nTrusted=true\n\n"));
    assert!(merged.contains("[IdentityResolvingKey]\n"));
    assert!(merged.contains("[LocalSignatureKey]\n"));
    assert!(merged.contains("[LongTermKey]\n"));
}

#[test]
fn removes_stale_managed_key_sections_when_no_keys_are_rendered() {
    let existing_info = "\
[General]
Name=Legion M600 Mouse

[IdentityResolvingKey]
Key=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA

[LocalSignatureKey]
Key=BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB
Counter=7
Authenticated=false

[LongTermKey]
Key=CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC
Authenticated=false
EncSize=16
EDiv=1
Rand=2
";
    let empty_material = windows_key_material::parse_hivexsh_lsval_output("").unwrap();
    let key_sections = BluezInfoKeySections::from_windows_key_material(&empty_material).unwrap();

    let merged = bluez_info::merge_key_sections(existing_info, &key_sections);

    assert_eq!(merged, "[General]\nName=Legion M600 Mouse\n");
}

#[test]
fn returns_only_key_sections_when_existing_info_is_empty() {
    let key_sections = rendered_key_sections();

    let merged = bluez_info::merge_key_sections("", &key_sections);

    assert!(merged.starts_with("[IdentityResolvingKey]\n"));
    assert!(merged.ends_with("Rand=9255003132036915216\n"));
}

fn rendered_key_sections() -> BluezInfoKeySections {
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

    BluezInfoKeySections::from_windows_key_material(&material).unwrap()
}
