# BlueBond Coding Rules

## Purpose

BlueBond edits Bluetooth bond credentials and BlueZ state. The codebase must stay boring, explicit, and mechanically verifiable.

These rules are the project contract for Rust implementation. They are inspired by the stricter boundary and verification model used in the core kits, adapted for a Rust system CLI.

## Non-Negotiables

- Architecture boundaries are enforced by code structure and verification scripts, not review memory.
- `domain` code is pure and side-effect free.
- `app` code orchestrates use cases and safety policy.
- `infra` code owns operating-system interaction.
- `cli` code only parses input and renders output.
- Bluetooth key material is treated as secret.
- `scan` and `plan` are read-only.
- `apply` and `rollback` require explicit root execution.
- Every write to `/var/lib/bluetooth` requires a backup first.
- No Windows registry writes, ever.

## Simplicity First

Prefer the smallest code that correctly solves the current problem.

Rules:

- No speculative plugin systems.
- No GUI abstractions in V1.
- No async runtime unless a real concurrent workflow requires it.
- No generic framework around shell commands.
- No global service locator.
- No domain abstractions with only one implementation unless they sit at a real system boundary.
- No workspace split until there is a real second crate.

Acceptable abstractions:

- Traits for system boundaries that must be faked in tests.
- Newtypes for domain correctness and secret redaction.
- Small modules that protect architectural direction.

## Layer Responsibilities

### `domain`

Owns pure concepts and rules.

Allowed:

- Bluetooth addresses.
- Device identity.
- Adapter identity.
- Bond key value objects.
- Match candidates and confidence.
- Sync plan data structures.
- Validation of key lengths and address formats.

Forbidden:

- Filesystem access.
- Shell commands.
- Terminal output.
- `systemctl`.
- `bluetoothctl`.
- `hivex`.
- Root privilege checks.
- Linux or Windows path discovery.

### `app`

Owns use-case orchestration.

Allowed:

- Scan workflow.
- Plan workflow.
- Apply workflow.
- Rollback workflow.
- Doctor workflow.
- Safety sequencing.
- Calling infrastructure through narrow ports or concrete adapters.

Forbidden:

- Parsing BlueZ `info` syntax directly.
- Parsing Windows registry output directly.
- Formatting CLI tables.
- Embedding raw command strings when an infrastructure adapter should own them.

### `infra`

Owns side effects and external systems.

Allowed:

- Reading `/var/lib/bluetooth`.
- Writing BlueZ `info` files.
- Running `systemctl`.
- Running `bluetoothctl`.
- Running `hivexget` or `hivexsh`.
- Discovering mounted Windows partitions.
- Creating and restoring backups.
- Checking root privileges.

Forbidden:

- Owning match policy.
- Deciding which candidate is the best match.
- Printing user-facing CLI output directly.
- Leaking raw key material in logs or errors.

### `convert`

Owns deterministic key conversion.

Allowed:

- Windows key representation to BlueZ key representation.
- Endian conversion.
- Hex formatting.
- Compatibility section generation.

Forbidden:

- Filesystem access.
- Shell commands.
- Terminal output.
- Match scoring.

### `cli`

Owns command-line presentation.

Allowed:

- `clap` command definitions.
- Human-readable output.
- Exit code mapping.
- Optional JSON rendering later.

Forbidden:

- Business rules.
- Filesystem mutation.
- Direct shell command execution.
- Key conversion logic.

## Dependency Direction

Allowed:

```text
cli -> app -> domain
app -> infra
app -> convert
infra -> domain
convert -> domain
```

Forbidden:

```text
domain -> app
domain -> infra
domain -> cli
infra -> cli
convert -> cli
```

Implementation rule:

- Prefer `pub(crate)` over `pub`.
- Keep modules private unless another layer genuinely needs them.
- Re-export intentionally from `mod.rs`; do not expose internals by default.

## Rust Style

Use Rust's type system to make invalid states hard to express.

Rules:

- Use newtypes for Bluetooth addresses, key material, EDIV, ERand, and Windows FILETIME.
- Avoid passing raw `String` for domain concepts.
- Avoid nullable-style modeling with `Option` when a specific enum communicates intent better.
- Prefer `Result<T, BluebondError>` for domain/app errors.
- Use `thiserror` for typed errors.
- Use `anyhow` only at top-level command boundaries if needed.
- Avoid `unwrap` and `expect` in production code.
- Avoid panics for user/environment errors.
- Avoid broad `pub` visibility.
- Keep functions small and named after the rule or action they perform.

Allowed `expect` cases:

- Tests.
- Static invariants that cannot fail unless source code is wrong.

## Secret Handling

Bluetooth keys are credentials.

Rules:

- Raw `LTK`, `IRK`, and `CSRK` values must not appear in normal output.
- `Debug` implementations for key types must redact.
- Error messages must not include raw key material.
- Test fixtures must use fake deterministic keys.
- Add `--show-secrets` only if a strong debugging need appears.

Pattern:

```rust
pub struct SecretKey16([u8; 16]);

impl std::fmt::Debug for SecretKey16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<redacted>")
    }
}
```

## Command Execution

All command execution must go through `infra::command`.

Rules:

- Do not call `std::process::Command` directly outside `infra::command`.
- Capture stdout, stderr, and exit status.
- Include command context in errors.
- Do not log secrets.
- Commands that mutate system state must be used only from `apply` or `rollback` flows.

This makes command behavior mockable and auditable.

## File Writes

Rules:

- `scan` and `plan` must not write files.
- `apply` must create a backup before writing.
- Write to a temporary file first when practical.
- Set final permissions explicitly.
- BlueZ `info` files should be owned by `root:root` and mode `0600`.
- Never write into the mounted Windows partition.

## Matching Policy

Matching must be explicit and reviewable.

Rules:

- Match score must include reasons.
- High-confidence automatic selection requires multiple signals.
- Name-only matching is not enough for apply.
- Ambiguous matches must stop and ask the user to select.
- Historical BLE identities should be preserved unless cleanup is explicit.

Recommended signals:

- Device name.
- VID/PID.
- BLE appearance.
- HID service UUID.
- Windows `LastConnected`.
- Existing Linux BlueZ record.
- Address exact match or address-family similarity.

## Error Handling

Errors should be actionable.

Good:

```text
No Windows SYSTEM hive found.
Mount your Windows partition, then run bluebond scan again.
```

Bad:

```text
IO error
```

Rules:

- Add context at layer boundaries.
- Keep domain errors typed.
- Map technical errors into helpful CLI messages.
- Do not leak secrets in errors.
- Use stable exit codes later when the CLI contract matures.

## Testing Rules

Most tests must run without root and without Bluetooth hardware.

Required test coverage:

- Bluetooth address parsing and normalization.
- BlueZ `info` parsing.
- BlueZ `info` writing.
- Windows `hivexget` output parsing.
- Windows FILETIME conversion.
- ERand little-endian conversion.
- Key redaction behavior.
- Candidate match scoring.
- Dry-run plan generation.
- Backup path generation.
- Rollback restore behavior using temporary directories.

Testing style:

- Use fixture files for real-world shapes.
- Use fake deterministic key material.
- Use fake command runners for app flow tests.
- Avoid tests that depend on the developer's actual Bluetooth adapter.

## Verification Commands

Baseline local verification:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

Recommended once configured:

```bash
cargo deny check
cargo machete
./tool/check_architecture.sh
./tool/verify.sh
```

`tool/verify.sh` should become the canonical local and CI gate.

## Architecture Enforcement

Rust does not have a built-in equivalent of dependency-cruiser for module-layer rules, so BlueBond should combine three mechanisms:

1. Module privacy:
   - keep internals private
   - expose only intentional APIs
   - prefer `pub(crate)` over `pub`

2. Static checks:
   - add a lightweight `tool/check_architecture.sh`
   - reject forbidden import patterns such as `crate::infra` from `domain`
   - reject `std::process::Command` outside `infra::command`

3. Reviewable docs:
   - keep `docs/engineering/architecture.md` as the source of truth
   - update docs when boundaries intentionally change

The goal is not perfect static analysis on day one. The goal is to prevent easy drift and make boundary violations cheap to detect.

## Documentation Rules

When a recurring mistake appears twice, promote it into one of:

- `docs/engineering/coding-rules.md`
- `docs/engineering/architecture.md`
- `docs/safety.md`
- a verify script
- a test fixture

Do not rely on memory for stable project policy.

## ADR Policy

Use ADRs for decisions that change project direction.

Examples:

- Choosing Rust as the implementation language.
- Choosing `hivex` shell-out for V1.
- Changing from single-crate to workspace.
- Adding GUI support.
- Supporting Windows-side export.

Recommended path:

```text
docs/adr/0001-record-architecture-decisions.md
docs/adr/0002-rust-cli-first.md
```

Do not rewrite historical ADRs. Add a new ADR when a decision changes.

## V1 Code Review Checklist

Before merging a code change, confirm:

- Does this preserve dependency direction?
- Does this keep domain pure?
- Does this avoid printing secrets?
- Does this keep `scan` and `plan` read-only?
- Does this back up before writing?
- Is the behavior covered by tests or fixtures?
- Did `cargo fmt`, `cargo clippy`, and `cargo test` run?
- If a rule was missing, was it added to docs or verification?
