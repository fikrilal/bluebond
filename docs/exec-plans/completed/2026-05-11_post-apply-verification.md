# Plan: Post-Apply Verification

## Objective

Verify that applied BlueZ records are visible after mutation.

## Context

After writing BlueZ `info` records and restarting Bluetooth, BlueBond should re-read the BlueZ store and confirm target records exist with expected key material presence. Device reconnect remains a manual check.

## Acceptance Criteria

- [x] Re-read BlueZ inventory after apply.
- [x] Confirm each target device exists.
- [x] Confirm expected long-term key material is present.
- [x] Return a manual reconnect check message.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: medium

Impact area:

- app
- infra
- tests

## Checklist

- [x] Add app-level verification report.
- [x] Add tests for success and missing target.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Keep verification conservative: prove BlueZ store visibility, not hardware reconnect.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && cargo test --test apply_verification_tests
. "$HOME/.cargo/env" && ./tool/verify.sh
```

Result:

```text
cargo test --test apply_verification_tests: 2 passed
./tool/verify.sh: OK
```

## Follow-Up Debt

- Add optional Bluetooth controller/device runtime verification after V1 apply is stable.
