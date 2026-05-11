# Plan: BlueZ Sync Plan Generation

## Objective

Generate a dry-run BlueZ sync plan from domain match results.

## Context

Milestone 2 matching can identify exact usable matches and one-byte address drift candidates. Milestone 3 should now convert those match results into explicit no-write plan items.

This slice must stay pure and read-only. It should not expose a CLI command, convert key values, or write BlueZ files yet.

## Acceptance Criteria

- [x] Domain has a sync plan model.
- [x] Exact usable matches become plan actions.
- [x] Single address-drift candidates become plan actions.
- [x] Ambiguous, missing, and no-key matches are skipped with reasons.
- [x] Plan actions include Linux adapter, Linux target device, and Windows source device addresses.
- [x] Tests cover plan and skip behavior.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: low

Impact area:

- domain
- tests

No system mutation is allowed in this slice.

## Checklist

- [x] Add sync plan domain models.
- [x] Implement plan generation from `BondMatchReport`.
- [x] Add focused tests.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Keep plan generation independent from filesystem paths and key conversion. Exact path rendering and BlueZ info conversion are later slices.

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

- Exact filesystem path rendering and BlueZ info conversion remain later slices.
