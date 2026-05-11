# Plan: Apply Dry-Run Command

## Objective

Expose a read-only `bluebond apply --dry-run` command that previews exact BlueZ changes and backup candidates.

## Context

BlueBond has a dry-run plan command, a pure content preview model, and read-only preview input collection. Users need a CLI entry point that shows the richer apply preview without writing to `/var/lib/bluetooth`.

## Acceptance Criteria

- [x] CLI accepts `bluebond apply --dry-run`.
- [x] Dry-run supports `--bluez-dir` and `--windows-root`.
- [x] Output shows target `info` path, changed/unchanged status, and backup candidate path.
- [x] Output states that no changes were made.
- [x] Dry-run does not require root.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: medium

Impact area:

- cli
- app
- tests

## Checklist

- [x] Add CLI args for apply dry-run.
- [x] Wire scan, plan, preview collection, and backup snapshot model.
- [x] Add human-readable output.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Do not add an apply mutation command yet. The only accepted apply mode in this slice is `--dry-run`.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && cargo run --quiet -- apply --help
. "$HOME/.cargo/env" && cargo run --quiet -- apply --dry-run --bluez-dir tests/fixtures/bluez --windows-root tests/fixtures/windows
. "$HOME/.cargo/env" && ./tool/verify.sh
```

Result:

```text
apply --help: printed apply dry-run options
apply --dry-run with fixtures: exited 0 and printed no changes made
./tool/verify.sh: OK
```

## Follow-Up Debt

- Add JSON output for dry-run previews after the human-readable flow is stable.
