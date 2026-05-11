# Plan: Apply Safety Metadata

## Objective

Record safety metadata for apply operations so rollback can avoid guessing.

## Context

Backup snapshots need metadata that describes the operation, source Windows registry paths, target BlueZ paths, BlueBond version, and timestamp/snapshot ID. This metadata becomes the contract for Milestone 5 rollback.

## Acceptance Criteria

- [x] Metadata includes BlueBond version.
- [x] Metadata includes snapshot ID and backup root.
- [x] Metadata includes target BlueZ paths.
- [x] Metadata includes Windows source addresses and registry paths when known.
- [x] Metadata is written with backup snapshots.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: medium

Impact area:

- app
- backup
- tests

## Checklist

- [x] Add metadata model.
- [x] Include metadata in backup write flow.
- [x] Add focused tests.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Store metadata as JSON for Milestone 5 rollback readability and tooling compatibility.

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

- Add schema version migration rules if rollback format changes after V1.
