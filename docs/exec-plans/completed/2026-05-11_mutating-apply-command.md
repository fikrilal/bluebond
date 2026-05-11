# Plan: Mutating Apply Command

## Objective

Add an explicit mutating `bluebond apply --execute` command.

## Context

BlueBond now has preview collection, backup writing, atomic BlueZ writes, service restart, and post-apply verification components. The CLI needs to wire them behind an explicit flag and a privilege gate while keeping `--dry-run` as the safe review path.

## Acceptance Criteria

- [x] CLI accepts `bluebond apply --execute`.
- [x] `--execute` and `--dry-run` are mutually exclusive.
- [x] Execute requires privileged apply.
- [x] Execute writes backup before BlueZ mutation.
- [x] Execute stops service, writes records, starts service, and verifies state.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: high

Impact area:

- cli
- app
- system mutation
- tests

## Checklist

- [x] Add execute CLI flag.
- [x] Add app-level execute orchestration.
- [x] Add output summary.
- [x] Add tests for flag validation/root gate where practical.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Use `--execute` for mutation so the default apply behavior remains non-mutating unless the user is explicit.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && cargo run --quiet -- apply --help
. "$HOME/.cargo/env" && cargo run --quiet -- apply
. "$HOME/.cargo/env" && cargo run --quiet -- apply --dry-run --execute
. "$HOME/.cargo/env" && ./tool/verify.sh
```

Result:

```text
apply --help: printed --dry-run and --execute
apply: exited 2 and required exactly one mode
apply --dry-run --execute: exited 2 and rejected conflicting modes
./tool/verify.sh: OK
```

## Follow-Up Debt

- Add integration tests for execute with injected service and temp stores after command dependency injection exists.
