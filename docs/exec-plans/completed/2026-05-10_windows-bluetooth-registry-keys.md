# Plan: Windows Bluetooth Registry Key Inspection

## Objective

Extend `bluebond scan` to inspect ready Windows `SYSTEM` hive candidates and report Bluetooth adapter key directories under BTHPORT.

## Context

BlueBond can already read Linux BlueZ inventory and locate candidate Windows `SYSTEM` hives. The next read-only discovery slice is proving that BlueBond can find Windows Bluetooth key paths before device-level extraction and matching.

This slice must stay read-only. It should use `hivexsh` through `infra::command` and avoid direct registry parsing.

## Acceptance Criteria

- [x] Scan reports whether Windows Bluetooth key inspection ran.
- [x] Scan reports missing `hivexsh` without crashing.
- [x] Scan checks common `ControlSet00N\Services\BTHPORT\Parameters\Keys` paths.
- [x] Scan reports Windows Bluetooth adapter key directories as normalized Bluetooth addresses.
- [x] Tests cover registry key output parsing.
- [x] Tests cover compact Bluetooth address parsing.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: low

Impact area:

- app
- domain
- infra
- tests

No system mutation is allowed in this slice.

## Checklist

- [x] Add compact Bluetooth address parsing.
- [x] Add command stdin support for `hivexsh`.
- [x] Add Windows Bluetooth key inspection model.
- [x] Implement BTHPORT adapter key discovery.
- [x] Wire scan output.
- [x] Add fixture/parser tests.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-10`: Inspect adapter key directories first. Device key extraction and bond material decoding remain separate follow-up slices.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && ./tool/verify.sh
. "$HOME/.cargo/env" && cargo run -- scan --bluez-dir tests/fixtures/bluez --windows-root tests/fixtures/windows
. "$HOME/.cargo/env" && cargo run -- scan --bluez-dir tests/fixtures/bluez --windows-root /mnt/windows
```

Result:

```text
OK

Fixture placeholder hive reports registry inspection failed, as expected because it is not a real registry hive.

Real /mnt/windows smoke test found:
F8:89:D2:83:92:C0
ControlSet001\Services\BTHPORT\Parameters\Keys\f889d28392c0
```

## Follow-Up Debt

- Device-level key extraction remains a separate slice.
