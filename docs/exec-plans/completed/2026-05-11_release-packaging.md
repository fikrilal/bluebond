# Plan: Release Packaging

## Objective

Add a reproducible early release packaging path for the Rust CLI.

## Context

BlueBond is not ready for distro packages yet, but public users need predictable release artifacts once tags are cut.

## Acceptance Criteria

- [x] Local release packaging command exists.
- [x] Release package includes binary, README, changelog, and license.
- [x] Release package has a SHA-256 checksum.
- [x] GitHub Actions can build and upload release artifacts.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: low

Impact area:

- release
- harness

## Checklist

- [x] Add `tool/package-release.sh`.
- [x] Add release workflow.
- [x] Add release documentation.
- [x] Run package script locally.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Use plain Cargo release builds before adopting heavier release tooling.

## Verification Evidence

Commands run:

```bash
./tool/package-release.sh
./tool/verify.sh
```

Result:

```text
release package and checksum created under dist/
OK
```

## Follow-Up Debt

- Add macOS and Windows build jobs after BlueBond has tested platform-specific behavior there.
