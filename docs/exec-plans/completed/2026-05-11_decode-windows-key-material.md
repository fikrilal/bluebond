# Plan: Decode Windows Bluetooth Key Material

## Objective

Parse Windows BTHPORT device key values from `hivexsh lsval` output into typed Rust structures.

## Context

BlueBond can detect Windows Bluetooth device key directories and whether key material is present. The next step toward BlueZ conversion is decoding the Windows values from registry text output.

This slice should remain read-only and fixture-driven. It should not write BlueZ files or expose decoded key material in CLI output.

## Acceptance Criteria

- [x] Parser reads `LTK`, `IRK`, `CSRK`, `ERand`, `EDIV`, `Address`, `AddressType`, `KeyLength`, and `AuthReq`.
- [x] Hex byte lists are parsed into bytes.
- [x] DWORD values are parsed into `u32`.
- [x] QWORD-style hex byte lists are parsed into bytes.
- [x] Parsed address bytes convert into `BluetoothAddress`.
- [x] Tests use fake deterministic values.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: medium

Impact area:

- convert
- infra parser
- tests

No system mutation is allowed in this slice.

## Checklist

- [x] Add Windows key material structs.
- [x] Add `hivexsh lsval` parser.
- [x] Add focused parser tests.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Parse from `hivexsh` text output for V1 speed. Native hive parsing remains a later improvement.
- `2026-05-11`: Keep decoded key material out of CLI output until redaction and safety policy are explicit.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && ./tool/verify.sh
```

Result:

```text
OK
```

## Follow-Up Debt

- Integrating decoded key material into scan/plan remains a separate slice.
