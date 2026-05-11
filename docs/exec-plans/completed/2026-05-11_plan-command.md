# Plan: BlueBond Plan Command

## Objective

Expose the existing dry-run planning pipeline through `bluebond plan`.

## Context

Milestone 3 has domain sync plan generation and app-level filesystem rendering. The next slice should give users an end-to-end read-only dry run from discovery to planned BlueZ target paths.

This slice must not decode key values, write files, restart services, or require root.

## Acceptance Criteria

- [x] `bluebond plan` exists.
- [x] `bluebond plan` supports `--bluez-dir PATH`.
- [x] `bluebond plan` supports `--windows-root PATH`.
- [x] Command runs scan -> domain state -> match report -> sync plan -> rendered plan.
- [x] Output lists planned create/update actions.
- [x] Output lists skipped candidates with reasons.
- [x] Output clearly states that no changes were made.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: low

Impact area:

- cli
- app
- output

No system mutation is allowed in this slice.

## Checklist

- [x] Add plan command args.
- [x] Wire planning pipeline.
- [x] Add plan output rendering.
- [x] Run CLI smoke tests.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Keep `bluebond plan` read-only and human-readable only. JSON output remains a separate slice.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && ./tool/verify.sh
. "$HOME/.cargo/env" && cargo run -- plan --bluez-dir tests/fixtures/bluez --windows-root tests/fixtures/windows
. "$HOME/.cargo/env" && cargo run -- plan --bluez-dir tests/fixtures/bluez --windows-root /mnt/windows
```

Result:

```text
OK

Fixture placeholder Windows hive produced no planned changes and skipped candidates with missing Windows adapter.

Real /mnt/windows smoke test produced one update action and one one-byte drift create action against fixture BlueZ data. No changes were made.
```

## Follow-Up Debt

- JSON output and candidate selection remain separate slices.
