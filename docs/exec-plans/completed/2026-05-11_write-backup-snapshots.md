# Plan: Write Backup Snapshots

## Objective

Write planned BlueZ backup snapshots to disk before any BlueZ mutation.

## Context

BlueBond already models backup snapshots from previewed changes. The next safety step is to create a timestamped backup directory, write existing `info` files into their backup paths, and write metadata for future rollback.

## Acceptance Criteria

- [x] Backup snapshot directories are created under a supplied backup root.
- [x] Backup entries are written to disk with parent directories.
- [x] Backup metadata is written alongside entries.
- [x] New BlueZ records without existing content do not create backup files.
- [x] No BlueZ store mutation is introduced by this task.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: medium

Impact area:

- app
- infra
- backup
- tests

## Checklist

- [x] Add backup snapshot directory and file write helpers.
- [x] Add metadata writing.
- [x] Add app-level backup write orchestration.
- [x] Add focused tests.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Use deterministic snapshot IDs in tests and timestamp-derived IDs for runtime.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && cargo test --test backup_snapshot_tests
. "$HOME/.cargo/env" && ./tool/verify.sh
```

Result:

```text
cargo test --test backup_snapshot_tests: 6 passed
./tool/verify.sh: OK
```

## Follow-Up Debt

- Milestone 5 rollback should validate and consume the metadata format.
