# Plan: BlueZ Info Content Preview

## Objective

Render previewable before/after BlueZ `info` content for sync plan actions.

## Context

The project can now render Windows key material into BlueZ key sections and merge those sections into BlueZ `info` text. The next step is an app-layer preview model that combines a `SyncPlan`, existing BlueZ info content, and decoded Windows key material.

This remains pure. It does not read or write `/var/lib/bluetooth`; callers supply content and decoded material explicitly.

## Acceptance Criteria

- [x] Each sync action can render a target `info` path and next file content.
- [x] Existing `info` content is merged when present.
- [x] Missing existing `info` content creates a full key-section preview from empty content.
- [x] Missing Windows key material for an action is reported as an error.
- [x] Preview debug output does not leak full file content or key material.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: medium

Impact area:

- app
- convert
- tests

## Checklist

- [x] Add app-layer content preview types and builder.
- [x] Add tests for update, create, missing material, unchanged content, and debug redaction.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Keep filesystem reads outside this layer. The preview builder accepts supplied content and key material so it stays deterministic and testable.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && cargo test --test bluez_info_content_preview_tests
. "$HOME/.cargo/env" && ./tool/verify.sh
```

Result:

```text
cargo test --test bluez_info_content_preview_tests: 5 passed
./tool/verify.sh: OK
```

## Follow-Up Debt

- Wire preview inputs to real BlueZ and Windows reads after the pure model is stable.
