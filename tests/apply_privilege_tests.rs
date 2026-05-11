use bluebond::app::apply;

#[test]
fn accepts_privileged_apply_when_running_as_root() {
    assert!(apply::require_privileged_apply_with(true).is_ok());
}

#[test]
fn rejects_mutating_apply_when_not_running_as_root() {
    let result = apply::require_privileged_apply_with(false);

    assert!(result.is_err());
}

#[test]
fn dry_run_request_does_not_require_privilege_gate() {
    let request = apply::default_dry_run_request();

    assert!(request.backup_root.ends_with("dry-run-preview"));
}
