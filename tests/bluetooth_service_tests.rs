use bluebond::app::apply;

#[test]
fn reports_successful_bluetooth_service_restart() {
    let report = apply::bluetooth_service_report_from_outcomes(true, true).unwrap();

    assert!(report.stopped);
    assert!(report.started);
    assert_eq!(report.recovery_instructions, None);
}

#[test]
fn reports_recovery_instructions_when_bluetooth_service_start_fails() {
    let error = apply::bluetooth_service_report_from_outcomes(true, false).unwrap_err();
    let message = error.to_string();

    assert!(message.contains("sudo systemctl start bluetooth.service"));
    assert!(message.contains("restore from the BlueBond backup"));
}

#[test]
fn stop_failure_prevents_start_sequence() {
    let error = apply::bluetooth_service_report_from_outcomes(false, true).unwrap_err();

    assert!(error.to_string().contains("systemctl"));
}
