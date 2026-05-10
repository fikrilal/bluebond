# Plan: Windows Bluetooth Device Key Extraction

## Objective

Extend `bluebond scan` to inspect Windows Bluetooth device key directories under each discovered BTHPORT adapter key.

## Context

BlueBond can locate Windows `SYSTEM` hives and discover BTHPORT adapter key directories. The next read-only discovery slice is finding device-level key directories below those adapters.

This slice should identify device addresses and whether expected bond material values are present. Full value decoding and BlueZ conversion remain follow-up work.

## Acceptance Criteria

- [x] Scan lists Windows Bluetooth device keys below each Windows adapter key.
- [x] Device key names are normalized into `BluetoothAddress`.
- [x] Scan reports whether common bond material values are present.
- [x] Tests cover device key listing parsing.
- [x] Tests cover value-name parsing for key material presence.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: low

Impact area:

- infra
- app
- cli
- tests

No system mutation is allowed in this slice.

## Checklist

- [x] Add Windows Bluetooth device key model.
- [x] Parse device key directory listings.
- [x] Inspect device values enough to detect key material presence.
- [x] Wire scan output.
- [x] Add parser tests.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-10`: Keep this slice to device discovery and key-material presence. Full LTK/IRK/CSRK/ERand/EDIV decoding belongs to a dedicated conversion slice.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && ./tool/verify.sh
. "$HOME/.cargo/env" && cargo run -- scan --bluez-dir tests/fixtures/bluez --windows-root /mnt/windows
```

Result:

```text
OK

Real /mnt/windows smoke test found five Windows Bluetooth device keys under:
ControlSet001\Services\BTHPORT\Parameters\Keys\f889d28392c0

All five reported key material present.
```

## Follow-Up Debt

- Full key value decoding remains a separate conversion slice.
