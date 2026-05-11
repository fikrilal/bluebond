use std::path::PathBuf;

use bluebond::app::apply::{
    self, BackupSnapshotEntry, BluezInfoContentChange, BluezInfoContentPreview,
};

#[test]
fn builds_backup_entry_for_existing_bluez_info_content() {
    let preview = BluezInfoContentPreview {
        changes: vec![update_change()],
    };

    let snapshot = apply::build_backup_snapshot(&preview, "tests/fixtures/backups/snapshot-1");

    assert_eq!(
        snapshot.root_dir,
        PathBuf::from("tests/fixtures/backups/snapshot-1")
    );
    assert_eq!(snapshot.entries.len(), 1);
    assert_eq!(
        snapshot.entries[0],
        BackupSnapshotEntry {
            source_path: PathBuf::from(
                "tests/fixtures/bluez/F8:89:D2:83:92:C0/C6:C0:FD:F1:FB:80/info"
            ),
            backup_path: PathBuf::from(
                "tests/fixtures/backups/snapshot-1/F8:89:D2:83:92:C0/C6:C0:FD:F1:FB:80/info"
            ),
            content: "[General]\nName=Legion M600 Mouse\n".to_string(),
        }
    );
}

#[test]
fn skips_backup_entry_for_new_bluez_info_content() {
    let preview = BluezInfoContentPreview {
        changes: vec![create_change()],
    };

    let snapshot = apply::build_backup_snapshot(&preview, "tests/fixtures/backups/snapshot-1");

    assert!(snapshot.entries.is_empty());
}

#[test]
fn backup_snapshot_paths_are_deterministic_for_multiple_entries() {
    let mut second = update_change();
    second.linux_target_device_address = "C6:C0:FD:F1:FB:81".parse().unwrap();
    second.target_info_path =
        PathBuf::from("tests/fixtures/bluez/F8:89:D2:83:92:C0/C6:C0:FD:F1:FB:81/info");
    let preview = BluezInfoContentPreview {
        changes: vec![update_change(), second],
    };

    let snapshot = apply::build_backup_snapshot(&preview, "tests/fixtures/backups/snapshot-1");

    assert_eq!(
        snapshot
            .entries
            .iter()
            .map(|entry| entry.backup_path.clone())
            .collect::<Vec<_>>(),
        vec![
            PathBuf::from(
                "tests/fixtures/backups/snapshot-1/F8:89:D2:83:92:C0/C6:C0:FD:F1:FB:80/info"
            ),
            PathBuf::from(
                "tests/fixtures/backups/snapshot-1/F8:89:D2:83:92:C0/C6:C0:FD:F1:FB:81/info"
            ),
        ]
    );
}

#[test]
fn debug_redacts_backup_snapshot_content() {
    let preview = BluezInfoContentPreview {
        changes: vec![update_change()],
    };

    let snapshot = apply::build_backup_snapshot(&preview, "tests/fixtures/backups/snapshot-1");
    let debug = format!("{snapshot:?}");

    assert!(debug.contains("<redacted>"));
    assert!(!debug.contains("Legion M600 Mouse\\n"));
}

fn update_change() -> BluezInfoContentChange {
    BluezInfoContentChange {
        display_name: "Legion M600 Mouse".to_string(),
        linux_adapter_address: "F8:89:D2:83:92:C0".parse().unwrap(),
        linux_target_device_address: "C6:C0:FD:F1:FB:80".parse().unwrap(),
        windows_source_device_address: "C6:C0:FD:F1:FB:80".parse().unwrap(),
        target_info_path: PathBuf::from(
            "tests/fixtures/bluez/F8:89:D2:83:92:C0/C6:C0:FD:F1:FB:80/info",
        ),
        existing_info_content: Some("[General]\nName=Legion M600 Mouse\n".to_string()),
        next_info_content: "[General]\nName=Legion M600 Mouse\n\n[LongTermKey]\nKey=NEW\n"
            .to_string(),
        content_changed: true,
    }
}

fn create_change() -> BluezInfoContentChange {
    BluezInfoContentChange {
        existing_info_content: None,
        ..update_change()
    }
}
