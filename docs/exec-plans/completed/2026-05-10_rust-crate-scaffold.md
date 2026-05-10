# Plan: Rust Crate Scaffold

## Objective

Create the initial Rust CLI scaffold for BlueBond with the documented layered architecture, a low-risk `doctor` command, and verification through the existing harness.

## Context

Relevant docs:

- `AGENTS.md`
- `docs/README.md`
- `docs/engineering/architecture.md`
- `docs/engineering/coding-rules.md`
- `docs/harness/agent-harness.md`

Current state:

- Repository has docs and harness scripts.
- No Rust crate exists yet.
- `./tool/verify.sh` currently skips Rust checks because `Cargo.toml` is absent.

## Acceptance Criteria

- [x] `Cargo.toml` exists with a binary/library crate.
- [x] `src/main.rs` is thin and delegates to the library.
- [x] `src/lib.rs` exposes the planned top-level modules.
- [x] Layer folders exist for `cli`, `app`, `domain`, `infra`, and `convert`.
- [x] `bluebond doctor` runs without requiring root.
- [x] `doctor` reports availability of required host tools and paths.
- [x] `infra::command` is the only production module using `std::process::Command`.
- [x] At least one pure domain type has tests.
- [x] `./tool/verify.sh` passes.

## Risk

Risk class: low

Impact area:

- harness
- domain
- CLI scaffold

No system Bluetooth state should be mutated in this plan.

## Checklist

- [x] Add `Cargo.toml`.
- [x] Add source module skeleton.
- [x] Implement CLI parsing with `clap`.
- [x] Implement typed error surface.
- [x] Implement `infra::command`.
- [x] Implement `doctor` use case.
- [x] Add `BluetoothAddress` domain type and tests.
- [x] Run `cargo fmt`.
- [x] Run `./tool/verify.sh`.
- [x] Record verification evidence.

## Decision Log

- `2026-05-10`: Start with `doctor` instead of `scan` or `apply` because it exercises CLI/app/infra boundaries without mutating system state.
- `2026-05-10`: Installed a user-local Rust toolchain with `rustup` because the machine did not have `rustc` or `cargo`.
- `2026-05-10`: Tightened `tool/check_architecture.sh` after the first run showed the path allowlist was too broad for Rust source paths.

## Verification Evidence

Commands run:

```bash
./tool/verify.sh
cargo run -- doctor
```

Result:

```text
./tool/verify.sh

==> Verify documentation structure
==> Verify architecture guardrails
==> Cargo fmt
==> Cargo clippy
==> Cargo test
OK

cargo run -- doctor

BlueBond doctor

     ok  hivexget             required to read Windows SYSTEM registry hives
     ok  hivexsh              required to traverse Windows SYSTEM registry hives
     ok  bluetoothctl         required to verify BlueZ device state
     ok  systemctl            required for apply and rollback flows
     ok  BlueZ store          required to read Linux BlueZ bond records
     ok  findmnt              used to discover mounted Windows partitions
```

## Follow-Up Debt

- Implement `scan` after the scaffold proves the architecture and harness.
