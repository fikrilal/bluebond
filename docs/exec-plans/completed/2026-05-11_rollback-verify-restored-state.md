# Plan: Verify Restored BlueZ State

## Objective

Verify restored BlueZ records after rollback.

## Context

After restoring backup files and restarting Bluetooth, BlueBond should re-read the BlueZ store and confirm restored target records are visible.

## Acceptance Criteria

- [x] Rollback verification re-reads BlueZ inventory.
- [x] Verification reports restored target visibility.
- [x] CLI output includes verification result and manual reconnect guidance.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: medium

Impact area:

- app
- cli
- tests

## Checklist

- [x] Add rollback verification report.
- [x] Wire verification into restore command.
- [x] Add focused tests.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Rollback verification checks BlueZ store visibility; physical reconnect remains manual.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && cargo test --test rollback_tests
```

Result:

```text
cargo test --test rollback_tests: 7 passed
```

## Follow-Up Debt

- Add optional Bluetooth runtime reconnect verification later.
