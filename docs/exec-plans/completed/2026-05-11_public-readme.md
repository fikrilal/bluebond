# Plan: Public README

## Objective

Write a root README that lets a new user understand what BlueBond does, what it changes, and how to run the safe workflow.

## Context

Milestone 6 requires public project readiness. The repository currently has internal docs but no public root README.

## Acceptance Criteria

- [x] README explains the dual-boot Bluetooth problem.
- [x] README documents install/build prerequisites.
- [x] README documents `doctor`, `scan`, `plan`, `apply`, and `rollback`.
- [x] README calls out safety constraints and known limitations.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: low

Impact area:

- docs
- release

## Checklist

- [x] Add public root README.
- [x] Link detailed docs.
- [x] Keep command examples aligned with current CLI flags.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Keep the README focused on the safe CLI path; deeper internals stay under `docs/`.

## Verification Evidence

Commands run:

```bash
./tool/verify.sh
```

Result:

```text
OK
```

## Follow-Up Debt

- Add screenshots or terminal recordings after the first tagged release.
