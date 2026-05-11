# Plan: List BlueBond Backups

## Objective

List BlueBond-created backup snapshots from backup metadata.

## Context

Milestone 4 writes `bluebond-backup.json` metadata beside backup files. Rollback must use this metadata rather than guessing paths from directory names.

## Acceptance Criteria

- [x] Backup metadata can be read from disk.
- [x] Backup root can be scanned for BlueBond metadata files.
- [x] Backups are listed with snapshot ID, operation, target paths, and backup paths.
- [x] Invalid/non-BlueBond directories are ignored.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: medium

Impact area:

- app
- infra
- rollback
- tests

## Checklist

- [x] Add metadata read/list helpers.
- [x] Add app-level rollback backup listing model.
- [x] Add CLI list output.
- [x] Add focused tests.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Rollback listing only trusts `bluebond-backup.json`.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && cargo test --test rollback_tests
. "$HOME/.cargo/env" && cargo run --quiet -- rollback list --backup-dir /tmp/nonexistent-bluebond-backups
```

Result:

```text
cargo test --test rollback_tests: 7 passed
rollback list: printed no backups found
```

## Follow-Up Debt

- Add filtering by target device after basic listing is stable.
