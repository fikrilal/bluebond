# Plan: Bluetooth Service Restart

## Objective

Stop and start `bluetooth.service` safely around BlueZ store mutation.

## Context

BlueZ reads its store while the Bluetooth service is running. Mutating apply should stop the service before writes and start it afterward. Failures must produce recovery instructions.

## Acceptance Criteria

- [x] Infra can stop `bluetooth.service`.
- [x] Infra can start `bluetooth.service`.
- [x] App exposes a restart sequence.
- [x] Start failure includes recovery instructions.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: high

Impact area:

- infra
- app
- system mutation
- tests

## Checklist

- [x] Add service stop/start helpers.
- [x] Add app restart orchestration.
- [x] Add tests with injected service outcomes.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Keep service commands in infra and test app sequencing with injected outcomes.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && cargo test --test bluetooth_service_tests
. "$HOME/.cargo/env" && ./tool/verify.sh
```

Result:

```text
cargo test --test bluetooth_service_tests: 3 passed
./tool/verify.sh: OK
```

## Follow-Up Debt

- Add distro-specific recovery guidance if early users report different service names.
