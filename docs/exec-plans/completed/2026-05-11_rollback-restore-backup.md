# Plan: Restore Selected Backup

## Objective

Restore a selected BlueBond backup snapshot safely.

## Context

Rollback should restore only files described by BlueBond metadata and backed by actual backup files. It must stop Bluetooth before writing and start it afterward.

## Acceptance Criteria

- [x] Rollback requires privileged execution.
- [x] Rollback restores only metadata-declared backup entries.
- [x] Missing backup files fail closed.
- [x] Bluetooth is stopped before restore and started afterward.
- [x] Restore writes through the existing atomic BlueZ writer.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: high

Impact area:

- app
- infra
- system mutation
- tests

## Checklist

- [x] Add rollback restore request/report.
- [x] Add metadata-driven restore orchestration.
- [x] Add CLI restore command.
- [x] Add focused tests.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Do not restore paths unless metadata contains both target and backup path.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && cargo test --test rollback_tests
. "$HOME/.cargo/env" && cargo run --quiet -- rollback --help
```

Result:

```text
cargo test --test rollback_tests: 7 passed
rollback --help: printed list and restore commands
```

## Follow-Up Debt

- Add dry-run rollback preview after first restore flow exists.
