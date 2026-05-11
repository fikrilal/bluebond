# Plan: Render Sync Plan Filesystem Changes

## Objective

Render dry-run sync plan actions into exact BlueZ filesystem targets without writing files.

## Context

`SyncPlan` can generate domain-level actions and skip reasons. The next Milestone 3 slice should make those actions concrete enough for users to review: adapter address, Linux target device address, Windows source device address, and target BlueZ `info` path.

Path rendering belongs outside the domain layer because the domain must stay filesystem-free.

## Acceptance Criteria

- [x] App layer can render `SyncPlan` actions into filesystem changes.
- [x] Rendered changes include BlueZ target device directory and `info` path.
- [x] Rendered changes distinguish create vs update actions.
- [x] Rendered changes include Windows source device address.
- [x] Tests cover rendering with an injected BlueZ root.
- [x] Domain remains free of filesystem/path APIs.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: low

Impact area:

- app
- tests

No system mutation is allowed in this slice.

## Checklist

- [x] Add app-level rendered plan model.
- [x] Implement path rendering from `SyncPlan`.
- [x] Add focused tests.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Keep filesystem path rendering in `app`, not `domain`, to preserve domain purity.

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

- CLI `bluebond plan` rendering remains a separate slice.
