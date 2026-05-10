# Repository Guidelines

## Project Posture

BlueBond is a serious public Rust CLI that edits sensitive Bluetooth bond state. Favor correctness, explicit safety, and maintainable boundaries over speed.

## Source Of Truth

Start here:

- Docs index: `docs/README.md`
- Architecture: `docs/engineering/architecture.md`
- Coding rules: `docs/engineering/coding-rules.md`
- Agent harness: `docs/harness/agent-harness.md`
- Execution plans: `docs/exec-plans/README.md`
- Problem case study: `docs/case-studies/bluetooth-dual-boot-problem.md`
- Product/engineering proposal: `docs/product/engineering-proposal.md`

## Non-Negotiables

- Preserve dependency direction:

```text
cli -> app -> domain
app -> infra
app -> convert
infra -> domain
convert -> domain
```

- Keep `domain` pure: no filesystem, commands, terminal output, root checks, `systemctl`, `bluetoothctl`, or `hivex`.
- Treat Bluetooth bond keys as secrets.
- `scan` and `plan` must be read-only.
- `apply` and `rollback` must require root.
- Back up BlueZ state before writing.
- Never write to the Windows registry.
- Prefer `pub(crate)` over `pub`.
- Do not add speculative abstractions.

## Verification

Baseline checks for code changes:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

Once configured, prefer:

```bash
./tool/verify.sh
```

Use execution plans under `docs/exec-plans/active/` for non-trivial implementation work.

Docs-only changes do not need Rust checks unless they affect examples that should compile.

## Documentation Discipline

If the same mistake or workflow gap appears twice, promote it into:

- `docs/engineering/coding-rules.md`
- `docs/engineering/architecture.md`
- `docs/harness/agent-harness.md`
- a test fixture
- a verification script
- an ADR

Do not rely on memory for stable project policy.
