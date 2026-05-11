# Plan: Public Safety And Troubleshooting Docs

## Objective

Document the risk model and common failure modes for users before they run mutating commands.

## Context

BlueBond writes Linux BlueZ bond records and restarts Bluetooth. Public users need clear safety boundaries, recovery guidance, and issue-reporting data.

## Acceptance Criteria

- [x] Safety doc explains what BlueBond reads and writes.
- [x] Safety doc explains backup and rollback expectations.
- [x] Troubleshooting doc covers common discovery, matching, apply, service, and reconnect failures.
- [x] Docs avoid exposing raw Bluetooth key material.
- [x] Verification evidence is recorded before completion.

## Risk

Risk class: low

Impact area:

- docs
- support
- safety

## Checklist

- [x] Add safety documentation.
- [x] Add troubleshooting documentation.
- [x] Link docs from README and docs index.
- [x] Run relevant verification.
- [x] Record follow-up debt.

## Decision Log

- `2026-05-11`: Troubleshooting examples use redacted addresses and avoid raw key dumps.

## Verification Evidence

Commands run:

```bash
./tool/verify.sh
```

Result:

```text
OK
```

## Follow-Up Debt

- Add distro-specific service recovery notes after more real machines are tested.
