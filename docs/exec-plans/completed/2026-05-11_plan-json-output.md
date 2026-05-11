# Plan: Plan JSON Output

## Objective

Add machine-readable JSON output for `bluebond plan`.

## Context

`bluebond plan` currently produces human-readable dry-run output. JSON output is useful for issue reports, future GUI/TUI work, and stable automation.

This slice should serialize app-facing plan report types, not raw domain internals.

## Acceptance Criteria

- [x] `bluebond plan --json` exists.
- [x] Human output remains the default.
- [x] JSON output serializes app-facing plan report data.
- [x] JSON output includes `no_changes_made: true`.
- [x] JSON output includes BlueZ root, planned changes, and skipped candidates.
- [x] CLI does not import domain types.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: low

Impact area:

- cli
- app
- serialization

No system mutation is allowed in this slice.

## Checklist

- [x] Add `--json` argument to `bluebond plan`.
- [x] Add serializable app-facing JSON models.
- [x] Wire JSON output.
- [x] Run CLI smoke tests.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Serialize app plan report types only. Domain internals stay independent from CLI output format.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && ./tool/verify.sh
. "$HOME/.cargo/env" && cargo run --quiet -- plan --json --bluez-dir tests/fixtures/bluez --windows-root /mnt/windows > /tmp/bluebond-plan.json
jq '.no_changes_made, .rendered_plan.changes[0].change_type, (.skipped | length)' /tmp/bluebond-plan.json
```

Result:

```text
OK

true
"update_bluez_record"
0
```

## Follow-Up Debt

- None.
