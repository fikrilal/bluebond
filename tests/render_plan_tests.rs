use std::path::PathBuf;

use bluebond::app::plan::{self, RenderPlanRequest, RenderedSyncChangeType};
use bluebond::domain::{BluetoothAddress, SyncPlan, SyncPlanAction, SyncPlanActionType};

#[test]
fn renders_update_action_to_bluez_info_path() {
    let plan = SyncPlan {
        actions: vec![SyncPlanAction {
            action_type: SyncPlanActionType::UpdateExistingBluezRecord,
            linux_adapter_address: address("F8:89:D2:83:92:C0"),
            linux_target_device_address: address("C6:C0:FD:F1:FB:80"),
            bluez_template_device_address: None,
            windows_source_device_address: address("C6:C0:FD:F1:FB:80"),
            display_name: "Legion M600 Mouse".to_string(),
        }],
        skipped: Vec::new(),
    };

    let rendered = plan::render(
        &plan,
        &RenderPlanRequest::new("tests/fixtures/rendered-bluez-root"),
    );

    assert_eq!(rendered.changes.len(), 1);
    assert_eq!(
        rendered.changes[0].change_type,
        RenderedSyncChangeType::UpdateBluezRecord
    );
    assert_eq!(
        rendered.changes[0].target_device_dir,
        PathBuf::from("tests/fixtures/rendered-bluez-root/F8:89:D2:83:92:C0/C6:C0:FD:F1:FB:80")
    );
    assert_eq!(
        rendered.changes[0].target_info_path,
        PathBuf::from(
            "tests/fixtures/rendered-bluez-root/F8:89:D2:83:92:C0/C6:C0:FD:F1:FB:80/info"
        )
    );
    assert_eq!(
        rendered.changes[0].windows_source_device_address,
        "C6:C0:FD:F1:FB:80"
    );
}

#[test]
fn renders_create_action_for_address_drift_target() {
    let plan = SyncPlan {
        actions: vec![SyncPlanAction {
            action_type: SyncPlanActionType::CreateBluezRecord,
            linux_adapter_address: address("F8:89:D2:83:92:C0"),
            linux_target_device_address: address("C6:C0:FC:F1:FB:80"),
            bluez_template_device_address: None,
            windows_source_device_address: address("C6:C0:FD:F1:FB:80"),
            display_name: "Legion M600 Mouse".to_string(),
        }],
        skipped: Vec::new(),
    };

    let rendered = plan::render(
        &plan,
        &RenderPlanRequest::new("tests/fixtures/rendered-bluez-root"),
    );

    assert_eq!(
        rendered.changes[0].change_type,
        RenderedSyncChangeType::CreateBluezRecord
    );
    assert_eq!(
        rendered.changes[0].linux_target_device_address,
        "C6:C0:FC:F1:FB:80"
    );
    assert_eq!(
        rendered.changes[0].windows_source_device_address,
        "C6:C0:FD:F1:FB:80"
    );
}

#[test]
fn serializes_rendered_plan_report_as_json() {
    let plan = SyncPlan {
        actions: vec![SyncPlanAction {
            action_type: SyncPlanActionType::CreateBluezRecord,
            linux_adapter_address: address("F8:89:D2:83:92:C0"),
            linux_target_device_address: address("C6:C0:FC:F1:FB:80"),
            bluez_template_device_address: None,
            windows_source_device_address: address("C6:C0:FD:F1:FB:80"),
            display_name: "Legion M600 Mouse".to_string(),
        }],
        skipped: Vec::new(),
    };
    let rendered = plan::render(
        &plan,
        &RenderPlanRequest::new("tests/fixtures/rendered-bluez-root"),
    );
    let report = bluebond::app::plan::PlanReport {
        rendered_plan: rendered,
        skipped: Vec::new(),
        no_changes_made: true,
    };

    let json = serde_json::to_value(&report).unwrap();

    assert_eq!(json["no_changes_made"], true);
    assert_eq!(
        json["rendered_plan"]["changes"][0]["change_type"],
        "create_bluez_record"
    );
    assert_eq!(
        json["rendered_plan"]["changes"][0]["windows_source_device_address"],
        "C6:C0:FD:F1:FB:80"
    );
}

fn address(value: &str) -> BluetoothAddress {
    value.parse().unwrap()
}
