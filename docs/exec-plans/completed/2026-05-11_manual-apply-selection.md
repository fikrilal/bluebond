# Plan: Manual Apply Selection

## Objective

Allow a user to explicitly map one Linux target device to one Windows source device when automatic matching refuses an ambiguous candidate.

## Context

The live mouse test showed correct safety behavior: BlueBond skipped the Legion M600 mouse because multiple Windows records were plausible one-byte drift candidates. The user needs an explicit override that keeps automatic matching conservative while letting a human select the intended source.

## Acceptance Criteria

- [x] CLI accepts `--target-device` and `--windows-source-device` for `apply`.
- [x] Optional `--adapter` scopes the selection when multiple Linux adapters exist.
- [x] Manual selection builds a one-action sync plan.
- [x] Manual selection requires the Linux target device to exist.
- [x] Manual selection requires the Windows source device to exist with key material.
- [x] Manual selection works for both `--dry-run` and `--execute`.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: high

Impact area:

- app
- cli
- apply
- system mutation

## Checklist

- [x] Add manual selection app model and plan builder.
- [x] Add CLI flags and validation.
- [x] Wire manual selection into dry-run and execute paths.
- [x] Add focused tests.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Require both Linux target and Windows source addresses. BlueBond should not infer the source when automatic matching is ambiguous.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && cargo test --test apply_manual_selection_tests
. "$HOME/.cargo/env" && cargo run --quiet -- apply --help
. "$HOME/.cargo/env" && cargo run --quiet -- apply --dry-run --target-device C6:C0:FE:F1:FB:80
. "$HOME/.cargo/env" && ./tool/verify.sh
```

Result:

```text
cargo test --test apply_manual_selection_tests: 5 passed
apply --help: printed --adapter, --target-device, and --windows-source-device
partial manual selection: exited 2 and required both target/source flags
./tool/verify.sh: OK
```

## Follow-Up Debt

- Add interactive candidate selection later; explicit flags are safer for the first live fix.
