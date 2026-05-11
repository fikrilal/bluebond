# Plan: Apply Preview Input Collection

## Objective

Collect real read-only inputs for BlueZ apply previews from the filesystem and Windows registry.

## Context

The app layer already has a pure preview model that accepts existing BlueZ `info` content and decoded Windows key material. The missing bridge is read-only orchestration that gathers those inputs from infra for sync plan actions.

## Acceptance Criteria

- [x] Target BlueZ `info` content is read when present.
- [x] Missing target BlueZ `info` content is represented as create input, not an error.
- [x] Matched Windows device registry `lsval` output is read through infra.
- [x] Windows registry output is decoded into `WindowsBluetoothKeyMaterial`.
- [x] The collected inputs feed `preview_bluez_info_content`.
- [x] No filesystem writes or service mutations are introduced.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: medium

Impact area:

- app
- infra
- tests

## Checklist

- [x] Add BlueZ info content read helper in infra.
- [x] Add Windows device key material read helper in infra.
- [x] Add app-layer input collection from `ScanReport` and `SyncPlan`.
- [x] Add focused tests for existing content, missing content, and missing Windows source material.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Keep this task read-only. Actual backup and BlueZ writes remain separate tasks.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && cargo test --test bluez_info_content_preview_tests
. "$HOME/.cargo/env" && ./tool/verify.sh
```

Result:

```text
cargo test --test bluez_info_content_preview_tests: 7 passed
./tool/verify.sh: OK
```

## Follow-Up Debt

- Add command-output injection if registry collection needs broader unit coverage without shelling out.
