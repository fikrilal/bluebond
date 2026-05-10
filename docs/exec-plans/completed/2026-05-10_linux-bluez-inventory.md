# Plan: Linux BlueZ Inventory Scan

## Objective

Implement the first read-only `bluebond scan` slice by detecting Linux BlueZ adapters and paired device records from the BlueZ store.

## Context

BlueBond currently has the Rust crate scaffold, architecture guardrails, and `bluebond doctor`.

This slice starts Milestone 1 from `_WIP/end-to-end-backlog.md`. It must stay Linux-only, read-only, and small enough to verify with fixtures.

## Acceptance Criteria

- [x] `bluebond scan` exists.
- [x] Scan reads BlueZ adapters from `/var/lib/bluetooth` by default.
- [x] Scan supports an override path for fixture/manual testing.
- [x] Scan parses minimal BlueZ device `info` metadata.
- [x] Scan output lists adapters and devices.
- [x] Tests cover BlueZ inventory parsing.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: low

Impact area:

- cli
- app
- domain
- infra
- tests

No system mutation is allowed in this slice.

## Checklist

- [x] Add domain inventory models.
- [x] Implement read-only BlueZ store inventory parsing.
- [x] Wire `bluebond scan`.
- [x] Add fixture-based tests.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-10`: Keep this slice Linux-only. Windows hive detection and registry key inspection will be separate exec plans.
- `2026-05-10`: Add `--bluez-dir` for fixture/manual diagnostics while keeping `/var/lib/bluetooth` as the default.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && ./tool/verify.sh
. "$HOME/.cargo/env" && cargo run -- scan --bluez-dir tests/fixtures/bluez
. "$HOME/.cargo/env" && cargo run -- scan
```

Result:

```text
OK

Fixture scan found adapter F8:89:D2:83:92:C0 and device C6:C0:FD:F1:FB:80.

Live default scan failed as current user because /var/lib/bluetooth is mode 700 root:root on this machine.
This is expected for raw BlueZ store parsing and is tracked as follow-up product debt.
```

## Follow-Up Debt

- Decide whether unprivileged `scan` should use D-Bus/bluetoothctl for public inventory and reserve raw BlueZ key inspection for privileged `plan` or `apply`.
