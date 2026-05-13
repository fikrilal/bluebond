use crate::convert::windows_to_bluez::BluezInfoKeySections;

const MANAGED_KEY_SECTIONS: &[&str] = &[
    "IdentityResolvingKey",
    "LocalSignatureKey",
    "LongTermKey",
    "PeripheralLongTermKey",
    "RemoteSignatureKey",
    "SlaveLongTermKey",
];

pub fn merge_key_sections(existing_info: &str, key_sections: &BluezInfoKeySections) -> String {
    let preserved = preserve_unmanaged_sections(existing_info);
    let rendered_keys = key_sections.render();

    join_info_parts(&preserved, &rendered_keys)
}

fn preserve_unmanaged_sections(existing_info: &str) -> String {
    let mut preserved = Vec::new();
    let mut current_section_is_managed = false;

    for line in existing_info.lines() {
        if let Some(section) = parse_section_header(line) {
            current_section_is_managed = is_managed_key_section(section);
        }

        if !current_section_is_managed {
            preserved.push(line);
        }
    }

    preserved.join("\n")
}

fn join_info_parts(preserved: &str, rendered_keys: &str) -> String {
    let preserved = preserved.trim_end();
    let rendered_keys = rendered_keys.trim();

    match (preserved.is_empty(), rendered_keys.is_empty()) {
        (true, true) => String::new(),
        (true, false) => format!("{rendered_keys}\n"),
        (false, true) => format!("{preserved}\n"),
        (false, false) => format!("{preserved}\n\n{rendered_keys}\n"),
    }
}

fn parse_section_header(line: &str) -> Option<&str> {
    line.trim()
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .map(str::trim)
}

fn is_managed_key_section(section: &str) -> bool {
    MANAGED_KEY_SECTIONS.contains(&section)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::convert::windows_key_material;
    use crate::convert::windows_to_bluez::BluezInfoKeySections;

    #[test]
    fn preserves_existing_info_when_no_key_sections_are_rendered() {
        let key_sections = BluezInfoKeySections::from_windows_key_material(
            &windows_key_material::parse_hivexsh_lsval_output("").unwrap(),
        )
        .unwrap();

        assert_eq!(
            merge_key_sections("[General]\nName=Device\n", &key_sections),
            "[General]\nName=Device\n"
        );
    }
}
