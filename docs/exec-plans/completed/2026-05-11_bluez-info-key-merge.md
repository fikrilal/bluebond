# Plan: BlueZ Info Key Merge

## Objective

Merge rendered BlueZ key sections into existing BlueZ `info` text while preserving unrelated content.

## Context

BlueBond can now decode Windows Bluetooth key material and render BlueZ key sections. The next pure step is a deterministic text merge that replaces managed key sections without touching the filesystem.

This belongs in `convert` because it is format conversion only. The app and infra layers can later use it when building previews and applying changes.

## Acceptance Criteria

- [x] Existing non-key sections such as `[General]` and `[LinkKey]` are preserved.
- [x] Managed key sections are replaced with the rendered Windows-derived sections.
- [x] Missing managed sections are appended deterministically.
- [x] Empty rendered key material removes stale managed key sections without adding blank output.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: low

Impact area:

- convert
- tests

## Checklist

- [x] Add a pure BlueZ `info` merge helper.
- [x] Add focused tests for replacement, insertion, and stale-section removal.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Manage only the sections BlueBond currently renders: `[IdentityResolvingKey]`, `[LocalSignatureKey]`, and `[LongTermKey]`.

## Verification Evidence

Commands run:

```bash
. "$HOME/.cargo/env" && cargo test --test bluez_info_merge_tests
. "$HOME/.cargo/env" && ./tool/verify.sh
```

Result:

```text
cargo test --test bluez_info_merge_tests: 4 passed
./tool/verify.sh: OK
```

## Follow-Up Debt

- None.
