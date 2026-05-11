# Plan: Atomic BlueZ Info Writes

## Objective

Write planned BlueZ `info` content atomically after backups exist.

## Context

Apply previews contain exact target `info` paths and next content. Mutation must create target directories, write through a temporary file, rename into place, and set conservative file permissions where supported.

## Acceptance Criteria

- [x] Target adapter/device directories are created when missing.
- [x] `info` content is written through temp file then rename.
- [x] Final file mode is set to `0600` on Unix.
- [x] Existing unrelated BlueZ files are preserved.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: high

Impact area:

- infra
- app
- system mutation
- tests

## Checklist

- [x] Add atomic BlueZ info writer.
- [x] Add app-level write orchestration from content preview.
- [x] Add focused tests with temporary BlueZ stores.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Use temp file plus rename instead of direct writes.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && cargo test --test bluez_info_write_tests
. "$HOME/.cargo/env" && ./tool/verify.sh
```

Result:

```text
cargo test --test bluez_info_write_tests: 4 passed
./tool/verify.sh: OK
```

## Follow-Up Debt

- Consider fsync for stronger crash safety after the initial V1 flow is working.
