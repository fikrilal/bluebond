# Plan: Exact Bond Matching

## Objective

Add exact adapter and device address matching over normalized domain bond state.

## Context

BlueBond now has `DiscoveredBondState`, which separates Linux BlueZ inventory from Windows BTHPORT discovery. The next Milestone 2 slice is deterministic exact matching before introducing address-drift heuristics.

This slice must not classify fuzzy candidates. It should only match equal adapter addresses and equal device addresses.

## Acceptance Criteria

- [x] Domain has a match report model.
- [x] Matching finds exact Linux adapter to Windows adapter matches.
- [x] Matching finds exact Linux device to Windows device matches inside matched adapters.
- [x] Matching marks Windows device matches without key material as not usable.
- [x] Tests cover exact adapter and device matches.
- [x] Tests cover missing adapter and missing device cases.
- [x] Tests cover Windows device without key material.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: low

Impact area:

- domain
- tests

No system mutation is allowed in this slice.

## Checklist

- [x] Add match report domain models.
- [x] Implement exact matching logic.
- [x] Add focused tests.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-10`: Exact matching only. Address drift matching for the Legion M600 style case remains a separate slice.

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

- Address drift matching for devices like the Legion M600 remains a separate slice.
