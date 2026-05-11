# Plan: Harness Hardening

## Objective

Strengthen BlueBond's verification harness now that the codebase has domain matching, Windows registry parsing, and Bluetooth key fixtures.

## Context

The current harness runs docs structure checks, architecture checks, formatting, clippy, and tests. As BlueBond moves toward planning/apply behavior, the harness needs stronger boundary and fixture-safety checks.

This slice should improve mechanical feedback without changing product behavior.

## Acceptance Criteria

- [x] Architecture checks cover domain/app/infra/cli boundaries.
- [x] Architecture checks reject filesystem/process access in the domain layer.
- [x] Fixture checks reject unapproved Bluetooth key material.
- [x] Fixture checks reject direct machine-dependent paths in tests.
- [x] `tool/verify.sh` runs the new fixture checks.
- [x] Harness documentation describes the expanded checks.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: low

Impact area:

- harness
- docs
- tests

No product behavior changes are allowed in this slice.

## Checklist

- [x] Strengthen `tool/check_architecture.sh`.
- [x] Add `tool/check_fixtures.sh`.
- [x] Wire fixture checks into `tool/verify.sh`.
- [x] Update `docs/harness/agent-harness.md`.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-10`: Keep fixture checks simple and grep-based. They are guardrails for obvious mistakes, not a full secret scanner.
- `2026-05-10`: Allow deterministic fake fixture keys already used in tests.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && ./tool/verify.sh
```

Result:

```text
OK

The new fixture check caught and removed a machine-local `/mnt/windows` path from a test fixture setup.
```

## Follow-Up Debt

- None.
