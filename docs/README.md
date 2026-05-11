# BlueBond Documentation

This directory is the source of truth for BlueBond project planning, architecture, and engineering rules.

## Start Here

- [Product engineering proposal](product/engineering-proposal.md): product direction, V1 scope, CLI contract, and milestones.
- [Release engineering proposal](product/release-engineering-proposal.md): end-to-end public release and tag workflow.
- [Architecture](engineering/architecture.md): Rust crate layout, layer boundaries, dependency direction, and command flow.
- [Coding rules](engineering/coding-rules.md): implementation contract, safety rules, verification, and review checklist.
- [Safety model](engineering/safety.md): mutation boundaries, backups, rollback, and secret-handling expectations.
- [Troubleshooting](troubleshooting.md): common discovery, matching, apply, rollback, and reconnect failures.
- [Release process](release.md): local packaging and tag-based GitHub release workflow.
- [Agent harness](harness/agent-harness.md): agent-first workflow, verification harness, execution plans, and feedback loops.
- [Execution plans](exec-plans/README.md): planning workflow for non-trivial agent work.
- [Bluetooth dual-boot problem](case-studies/bluetooth-dual-boot-problem.md): real M600 case study and the manual fix that motivated the project.

## Directory Layout

```text
docs/
  README.md
  release.md
  troubleshooting.md
  release-notes/
    v0.1.0.md
  product/
    engineering-proposal.md
  engineering/
    architecture.md
    coding-rules.md
    safety.md
    key-format.md
  harness/
    agent-harness.md
  case-studies/
    bluetooth-dual-boot-problem.md
  exec-plans/
    README.md
    _template.md
    active/
    completed/
    tech-debt-tracker.md
  adr/
```

## Documentation Rules

- Product intent belongs in `docs/product/`.
- Architecture and implementation policy belong in `docs/engineering/`.
- Agent workflow and harness design belong in `docs/harness/`.
- Active implementation plans belong in `docs/exec-plans/`.
- Real-world debugging stories and validation cases belong in `docs/case-studies/`.
- User support guides that cut across features can live directly under `docs/`.
- Durable decisions belong in `docs/adr/`.
- Root-level Markdown should stay focused on public entry points and repository policy. Detailed docs belong here under `docs/`.

## Planned Docs

These files are referenced by the architecture and proposal but not written yet:

- `docs/engineering/key-format.md`
- `docs/adr/0001-record-architecture-decisions.md`
- `docs/adr/0002-rust-cli-first.md`
