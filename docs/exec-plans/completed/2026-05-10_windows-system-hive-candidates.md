# Plan: Windows SYSTEM Hive Candidate Detection

## Objective

Extend `bluebond scan` to detect offline Windows roots and validate candidate `SYSTEM` registry hive paths without parsing registry contents.

## Context

Linux BlueZ inventory scan is implemented. The next read-only discovery slice is locating the Windows installation that contains Bluetooth registry data.

This slice must not use `hivex` yet. Registry key inspection belongs to a later execution plan.

## Acceptance Criteria

- [x] `bluebond scan` supports `--windows-root PATH`.
- [x] Scan detects common Windows root candidates when no root is provided.
- [x] Scan validates `Windows/System32/config/SYSTEM`.
- [x] Scan reports missing, non-file, unreadable, and ready hive states.
- [x] Scan output includes a Windows installations section.
- [x] Tests cover explicit root validation and fixture detection.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: low

Impact area:

- cli
- app
- infra
- tests

No system mutation is allowed in this slice.

## Checklist

- [x] Add Windows root scan request field.
- [x] Add Windows hive candidate model.
- [x] Implement read-only candidate validation.
- [x] Wire scan output.
- [x] Add fixture-based tests.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-10`: Keep this slice to filesystem-level hive detection only. `hivex` registry inspection is intentionally deferred.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && ./tool/verify.sh
. "$HOME/.cargo/env" && cargo run -- scan --bluez-dir tests/fixtures/bluez --windows-root tests/fixtures/windows
. "$HOME/.cargo/env" && cargo run -- scan --bluez-dir tests/fixtures/bluez --windows-root tests/fixtures/windows-missing
```

Result:

```text
OK

Fixture scan reports tests/fixtures/windows as ready.
Fixture scan reports tests/fixtures/windows-missing as missing SYSTEM hive.
```

## Follow-Up Debt

- None.
