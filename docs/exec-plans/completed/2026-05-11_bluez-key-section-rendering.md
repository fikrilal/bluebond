# Plan: BlueZ Key Section Rendering

## Objective

Render decoded Windows Bluetooth key material into deterministic BlueZ `info` key sections.

## Context

The conversion layer already parses Windows `hivexsh lsval` output into `WindowsBluetoothKeyMaterial`. The next step is a pure renderer that converts that material into BlueZ-compatible INI sections without touching the filesystem.

BlueZ settings storage documents `[LongTermKey]`, `[PeripheralLongTermKey]`, `[IdentityResolvingKey]`, and `[LocalSignatureKey]` / `[RemoteSignatureKey]` sections. BlueBond will render the core sections first and leave compatibility duplication policy to the higher-level planner.

## Acceptance Criteria

- [x] Windows `LTK`, `IRK`, and `CSRK` bytes render as uppercase BlueZ hex.
- [x] Windows `ERand` renders as BlueZ decimal after little-endian conversion.
- [x] Windows `EDIV`, key length, and authentication metadata render into `[LongTermKey]`.
- [x] Rendered key structures do not leak raw key bytes through `Debug`.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: low

Impact area:

- convert
- tests

## Checklist

- [x] Add conversion structs and text rendering in `convert/windows_to_bluez.rs`.
- [x] Add focused tests for full rendering, partial material, malformed ERand, and redacted debug output.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Keep compatibility section duplication out of the converter for now; the planner should decide whether to write both current and legacy BlueZ section names.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && cargo test --test bluez_key_section_rendering_tests
. "$HOME/.cargo/env" && ./tool/verify.sh
```

Result:

```text
cargo test --test bluez_key_section_rendering_tests: 4 passed
./tool/verify.sh: OK
```

## Follow-Up Debt

- None.
