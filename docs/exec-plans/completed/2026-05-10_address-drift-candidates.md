# Plan: Address Drift Candidate Classification

## Objective

Classify conservative Windows device address-drift candidates after exact adapter matching.

## Context

Exact adapter and device matching is implemented. The Legion M600 case needs a later matching mode because Linux and Windows can contain nearby but non-identical Bluetooth device identities.

This slice should classify likely address-drift candidates without marking them applyable. Exact matches must still win over drift classification.

## Acceptance Criteria

- [x] Exact device matches still win over drift candidates.
- [x] One-byte address drift can be classified inside an exact adapter match.
- [x] Drift candidates require Windows key material to be present.
- [x] Multiple drift candidates are marked ambiguous.
- [x] No drift candidate remains a missing Windows device.
- [x] Tests cover exact win, single drift, ambiguous drift, no drift, and missing key material.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: medium

Impact area:

- domain
- matching
- tests

No system mutation is allowed in this slice. Drift candidates are classification-only.

## Checklist

- [x] Add drift match statuses and candidate metadata.
- [x] Implement conservative one-byte distance classification.
- [x] Add focused tests.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-10`: Use one-byte address distance as the initial conservative classifier because it matches the known Legion M600 case while staying easy to explain.
- `2026-05-10`: Do not mark drift candidates as usable/applyable in this slice.

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

- Drift candidates are classification-only. Applyable planning remains a later slice.
