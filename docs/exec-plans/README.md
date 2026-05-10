# Execution Plans

Execution plans are the system of record for non-trivial agent work.

Use an execution plan when work spans multiple steps, touches architecture, mutates system behavior, or is likely to continue across sessions.

## Lifecycle

1. Copy `docs/exec-plans/_template.md`.
2. Create a file in `docs/exec-plans/active/`.
3. Update status, decisions, and verification evidence as work progresses.
4. Move the plan to `docs/exec-plans/completed/` when finished.
5. Put unresolved follow-up debt in `docs/exec-plans/tech-debt-tracker.md`.

## Naming

Use:

```text
YYYY-MM-DD_short-topic.md
```

Examples:

```text
2026-05-10_rust-crate-scaffold.md
2026-05-10_bluez-info-parser.md
```

## What Belongs In A Plan

- Objective.
- Constraints.
- Acceptance criteria.
- Risk class.
- Implementation checklist.
- Decision log.
- Verification evidence.
- Follow-up debt.

## What Does Not Belong Here

- Tiny docs edits.
- One-line fixes.
- Speculative ideas without active work.
