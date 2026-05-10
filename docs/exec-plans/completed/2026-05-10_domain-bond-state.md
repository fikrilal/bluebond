# Plan: Domain Bluetooth Bond State

## Objective

Add domain-level Bluetooth bond state models that normalize Linux BlueZ and Windows BTHPORT discovery into matching-ready structures.

## Context

Milestone 1 discovery is complete. `scan` can read Linux BlueZ inventory, locate Windows `SYSTEM` hives, discover Windows adapter keys, and list Windows device keys with key-material presence.

Milestone 2 should not match directly against infra-shaped structs. This slice creates the domain vocabulary that future matching logic will use.

## Acceptance Criteria

- [x] Domain has a top-level discovered bond state model.
- [x] Domain represents Linux adapters and devices.
- [x] Domain represents Windows adapters and devices.
- [x] Domain represents key-material presence explicitly.
- [x] App layer can map a `ScanReport` into domain bond state.
- [x] Tests cover normalized domain construction from scan-shaped data.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: low

Impact area:

- domain
- app
- tests

No system mutation is allowed in this slice.

## Checklist

- [x] Add domain bond-state models.
- [x] Add scan-to-domain mapping.
- [x] Add focused tests.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-10`: Do not implement matching in this slice. Matching belongs to the next execution plan.
- `2026-05-10`: Keep filesystem paths and command behavior out of the domain model. Registry paths are allowed as logical source identifiers.

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

- Matching is intentionally deferred to the next slice.
