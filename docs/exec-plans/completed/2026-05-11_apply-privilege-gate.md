# Plan: Apply Privilege Gate

## Objective

Add an app-layer privilege gate for future mutating apply operations while keeping dry-run rootless.

## Context

Milestone 4 will eventually mutate `/var/lib/bluetooth` and restart Bluetooth. Those paths must require explicit privileged execution. Dry-run preview must continue to work without root.

## Acceptance Criteria

- [x] App exposes a privilege check for mutating apply operations.
- [x] Root execution is accepted.
- [x] Non-root execution is rejected for mutation.
- [x] Dry-run preview path does not call the privilege gate.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: low

Impact area:

- app
- infra
- tests

## Checklist

- [x] Add a privilege check result/API in `app::apply`.
- [x] Use infra root detection behind the app boundary.
- [x] Add tests with injected privilege state.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Add the gate before mutation exists so future write tasks cannot skip the safety contract.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && cargo test --test apply_privilege_tests
. "$HOME/.cargo/env" && cargo run --quiet -- apply
. "$HOME/.cargo/env" && ./tool/verify.sh
```

Result:

```text
cargo test --test apply_privilege_tests: 3 passed
apply without --dry-run: exited 2 and refused mutating apply
./tool/verify.sh: OK
```

## Follow-Up Debt

- Call this gate from the first real mutating apply command.
