# BlueBond Documentation

This directory is the source of truth for BlueBond project planning, architecture, and engineering rules.

## Start Here

- [Product engineering proposal](product/engineering-proposal.md): product direction, V1 scope, CLI contract, and milestones.
- [Architecture](engineering/architecture.md): Rust crate layout, layer boundaries, dependency direction, and command flow.
- [Coding rules](engineering/coding-rules.md): implementation contract, safety rules, verification, and review checklist.
- [Agent harness](harness/agent-harness.md): agent-first workflow, verification harness, execution plans, and feedback loops.
- [Execution plans](exec-plans/README.md): planning workflow for non-trivial agent work.
- [Bluetooth dual-boot problem](case-studies/bluetooth-dual-boot-problem.md): real M600 case study and the manual fix that motivated the project.

## Directory Layout

```text
docs/
  README.md
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
- Durable decisions belong in `docs/adr/`.
- Root-level Markdown should stay minimal. `AGENTS.md` is the operating contract for agents; broad docs belong here.

## Planned Docs

These files are referenced by the architecture and proposal but not written yet:

- `docs/engineering/safety.md`
- `docs/engineering/key-format.md`
- `docs/adr/0001-record-architecture-decisions.md`
- `docs/adr/0002-rust-cli-first.md`
