# Contributing To BlueBond

BlueBond edits sensitive Bluetooth state. Contributions should favor small, reviewable changes with explicit safety behavior.

## Local Setup

```bash
git clone https://github.com/fikrilal/bluebond.git
cd bluebond
cargo build
```

Install runtime tools used by real scans:

- BlueZ.
- `hivexsh`.
- `systemctl`.

Tests and most development work use fixtures and do not require a mounted Windows partition.

## Before Opening A Pull Request

Run:

```bash
./tool/verify.sh
```

This runs:

- documentation structure checks
- architecture guardrails
- fixture safety checks
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-features`

Do not claim verification passed unless it was actually run.

## Architecture Rules

Respect the dependency direction:

```text
cli -> app -> domain
app -> infra
app -> convert
infra -> domain
convert -> domain
```

High-level rules:

- `domain` is pure and side-effect free.
- `app` orchestrates use cases and safety sequencing.
- `infra` owns filesystem, command, BlueZ, registry, and service interactions.
- `cli` parses arguments and renders output.
- Raw Bluetooth key material must not appear in logs, errors, or normal output.

See [docs/engineering/coding-rules.md](docs/engineering/coding-rules.md).

## Execution Plans

Non-trivial work should start with an execution plan under `docs/exec-plans/active/` and move it to `docs/exec-plans/completed/` after verification.

Use the template:

```bash
cp docs/exec-plans/_template.md docs/exec-plans/active/YYYY-MM-DD_short-title.md
```

## Commit Messages

Use scoped Conventional Commit style:

```text
feat(scope): short message
fix(scope): short message
docs(scope): short message
ci(scope): short message
chore(scope): short message
```

Examples:

```text
feat(scan): detect linux bluez inventory
docs(safety): document rollback recovery
ci(github): run verification harness
```

## Safety Expectations

Mutating behavior needs extra scrutiny:

- `apply` and `rollback` must require root.
- Every BlueZ write must have a backup path.
- Windows registry writes are out of scope.
- Ambiguous matching must fail closed unless the user selects exact devices.
- Tests should use fake deterministic key material only.
