# Plan: Backup Snapshot Model

## Objective

Model backup snapshots for planned BlueZ `info` file changes before filesystem mutation exists.

## Context

Once BlueBond can preview exact file content, the next safety primitive is a backup snapshot model. This should describe what existing files need to be preserved and where those backups should live, without writing the backups yet.

The existing backup infra only defines the default backup directory. This task should add the app-level snapshot model and keep actual filesystem writes for a later explicit apply task.

## Acceptance Criteria

- [x] Backup snapshots are generated from previewed changes.
- [x] Existing files produce backup entries with source path, backup path, and content.
- [x] New files without existing content do not produce backup entries.
- [x] Backup paths are deterministic and grouped under a supplied snapshot root.
- [x] Snapshot debug output does not leak file content or key material.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: medium

Impact area:

- app
- backup
- tests

## Checklist

- [x] Add backup snapshot model and builder.
- [x] Add tests for update backup, create-no-backup, deterministic paths, and debug redaction.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Model backups before writing them so the safety contract is reviewable before privileged mutation is introduced.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && cargo test --test bluez_info_content_preview_tests --test backup_snapshot_tests
. "$HOME/.cargo/env" && ./tool/verify.sh
```

Result:

```text
cargo test --test bluez_info_content_preview_tests --test backup_snapshot_tests: 9 passed
./tool/verify.sh: OK
```

## Follow-Up Debt

- Implement actual backup file writes after preview and snapshot modeling are committed.
