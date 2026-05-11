# Plan: Project Contribution Metadata

## Objective

Add public contribution, changelog, security, and issue-template files for external collaboration.

## Context

Milestone 6 requires license and contribution guidance plus GitHub issue templates. `LICENSE` already exists, but the project does not yet have contribution or support files.

## Acceptance Criteria

- [x] Contribution guide documents local verification.
- [x] Changelog exists and starts at current unreleased work.
- [x] Security policy explains sensitive Bluetooth data handling.
- [x] Issue templates collect diagnostics without raw key material.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: low

Impact area:

- docs
- support

## Checklist

- [x] Add `CONTRIBUTING.md`.
- [x] Add `CHANGELOG.md`.
- [x] Add `SECURITY.md`.
- [x] Add GitHub issue templates.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Issue templates explicitly forbid posting raw Bluetooth key material.

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

- Add `CODE_OF_CONDUCT.md` before opening broad community contribution channels.
