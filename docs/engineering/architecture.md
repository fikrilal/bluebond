# BlueBond Architecture

## Position

The initial flat repository layout was useful as a sketch, but it is too module-by-technical-topic for a serious public Rust CLI.

BlueBond should use a layered architecture:

```text
CLI presentation -> application use cases -> domain model -> infrastructure adapters
```

The rule is simple:

- The domain should not know about `bluetoothctl`, `systemctl`, `hivex`, filesystems, terminals, or root privileges.
- The application layer coordinates use cases and safety policy.
- Infrastructure modules perform operating-system-specific work.
- The CLI layer only parses user input and renders output.

This keeps the risky parts testable and prevents the project from becoming a collection of command wrappers.

## Recommended Layout

Recommended V1 layout:

```text
bluebond/
  Cargo.toml
  README.md
  LICENSE
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
  src/
    main.rs
    lib.rs
    cli/
      mod.rs
      args.rs
      output.rs
    app/
      mod.rs
      scan.rs
      plan.rs
      apply.rs
      rollback.rs
      doctor.rs
    domain/
      mod.rs
      address.rs
      bond.rs
      device.rs
      adapter.rs
      match_candidate.rs
      plan.rs
      key_material.rs
    infra/
      mod.rs
      command.rs
      bluez/
        mod.rs
        info_file.rs
        store.rs
        service.rs
      windows/
        mod.rs
        system_hive.rs
        bthport.rs
        filetime.rs
      linux/
        mod.rs
        mounts.rs
        privileges.rs
      backup/
        mod.rs
        store.rs
    convert/
      mod.rs
      windows_to_bluez.rs
      endian.rs
    error.rs
  tests/
    fixtures/
      bluez/
      windows/
      command-output/
    scan_tests.rs
    planner_tests.rs
    key_conversion_tests.rs
    bluez_info_tests.rs
    rollback_tests.rs
```

## Why This Layout Is Better

The first layout grouped modules by immediate implementation concern:

```text
windows_registry.rs
linux_bluez.rs
planner.rs
apply.rs
backup.rs
commands.rs
```

That is fine for a prototype, but it blurs responsibilities as the project grows. For example, `apply.rs` can easily become responsible for validation, backup, file writing, service control, command output, and verification.

The recommended layout separates those responsibilities:

- `domain/`: data and rules that are true regardless of OS command details.
- `app/`: use-case orchestration.
- `infra/`: concrete adapters to Linux, BlueZ, Windows hives, shell commands, and backups.
- `convert/`: deterministic key conversion.
- `cli/`: user interface only.

That structure is common in serious Rust CLIs because it keeps the binary thin and makes most behavior testable through `lib.rs`.

## Crate Boundary

BlueBond should have both `main.rs` and `lib.rs`.

`main.rs` should be small:

```rust
fn main() -> std::process::ExitCode {
    bluebond::cli::run()
}
```

`lib.rs` exposes internal modules:

```rust
pub mod app;
pub mod cli;
pub mod convert;
pub mod domain;
pub mod error;
pub mod infra;
```

Reasoning:

- Integration tests can call library code directly.
- CLI parsing does not become the only entry point.
- Future GUI or daemon wrappers can reuse the same application layer.

## Dependency Direction

Allowed dependency direction:

```text
cli -> app -> domain
app -> infra
app -> convert
infra -> domain
convert -> domain
```

Forbidden dependency direction:

```text
domain -> app
domain -> infra
domain -> cli
infra -> cli
convert -> cli
```

The domain can define types such as `BluetoothAddress`, `BondKey`, and `SyncPlan`, but it must not call `systemctl`, parse terminal args, or read `/var/lib/bluetooth`.

Detailed coding and enforcement rules live in [coding-rules.md](coding-rules.md). The architecture document defines the shape; the coding rules define how to keep that shape intact while implementing features.

## Boundary Enforcement

BlueBond should follow the same principle as the core kits: architecture boundaries should be mechanically checked, not left to memory.

Rust gives us useful visibility controls, but module privacy alone is not enough. V1 should combine:

- narrow `pub(crate)` visibility
- a small `tool/check_architecture.sh`
- `cargo clippy` with warnings denied
- fixture-based tests for conversion and planner behavior

Initial architecture checks should reject:

- `crate::infra` imports from `domain`
- `crate::cli` imports outside `main.rs` and `cli`
- direct `std::process::Command` usage outside `infra::command`
- direct writes to `/var/lib/bluetooth` outside `infra::bluez::store`

These checks do not need to be perfect static analysis on day one. They need to catch the easy drift that makes boundaries decay.

## Command Flow

### `bluebond scan`

Flow:

```text
cli::args
  -> app::scan
    -> infra::linux::mounts
    -> infra::bluez::store
    -> infra::windows::system_hive
    -> infra::windows::bthport
  -> cli::output
```

Responsibilities:

- Discover mounted Windows installations.
- Discover Linux BlueZ adapters and devices.
- Discover Windows Bluetooth adapters and devices.
- Print candidates.
- Make no changes.

### `bluebond plan`

Flow:

```text
cli::args
  -> app::plan
    -> app::scan
    -> domain::match_candidate
    -> convert::windows_to_bluez
    -> domain::plan
  -> cli::output
```

Responsibilities:

- Select a candidate device.
- Score Windows/Linux matches.
- Convert Windows keys to proposed BlueZ records.
- Print exact intended writes.
- Make no changes.

### `sudo bluebond apply`

Flow:

```text
cli::args
  -> app::apply
    -> infra::linux::privileges
    -> app::plan
    -> infra::backup::store
    -> infra::bluez::service
    -> infra::bluez::store
    -> infra::bluez::service
    -> infra::bluez::store
  -> cli::output
```

Responsibilities:

- Require root.
- Recompute or load the plan.
- Create backup before writing.
- Stop Bluetooth.
- Write BlueZ records.
- Start Bluetooth.
- Verify BlueZ sees the target device.

### `sudo bluebond rollback`

Flow:

```text
cli::args
  -> app::rollback
    -> infra::linux::privileges
    -> infra::backup::store
    -> infra::bluez::service
    -> infra::backup::store
    -> infra::bluez::service
  -> cli::output
```

Responsibilities:

- Require root.
- List or select backup.
- Stop Bluetooth.
- Restore adapter directory.
- Start Bluetooth.
- Print restored paths.

## Domain Layer

The domain layer contains concepts, not operating-system operations.

Core domain types:

```rust
pub struct BluetoothAddress {
    bytes: [u8; 6],
}

pub enum AddressType {
    Public,
    Random,
}

pub struct Adapter {
    pub address: BluetoothAddress,
}

pub struct DeviceIdentity {
    pub address: BluetoothAddress,
    pub address_type: AddressType,
    pub name: Option<String>,
}

pub struct BleBondKeys {
    pub ltk: LongTermKey,
    pub irk: Option<IdentityResolvingKey>,
    pub csrk: Option<SignatureKey>,
}

pub struct SyncPlan {
    pub adapter: BluetoothAddress,
    pub target_device: DeviceIdentity,
    pub writes: Vec<BluezRecordWrite>,
    pub backup_required: bool,
}
```

Important domain behavior:

- Normalize Bluetooth addresses.
- Compare addresses.
- Represent key material without printing secrets by default.
- Score match candidates.
- Validate key lengths and required fields.

The domain layer should have no filesystem paths except when a value is part of a plan generated by the application layer.

## Application Layer

The application layer implements use cases:

```text
scan
plan
apply
rollback
doctor
```

It owns sequencing and safety policy.

Example responsibilities:

- `scan`: call readers and assemble inventory.
- `plan`: choose candidate and build `SyncPlan`.
- `apply`: enforce root, backup, stop service, write, restart.
- `rollback`: restore previous backup.
- `doctor`: verify dependencies and environment.

The app layer should depend on traits where useful:

```rust
trait BluezStore {
    fn read_adapters(&self) -> Result<Vec<LinuxAdapter>>;
    fn write_device_record(&self, record: &BluezRecordWrite) -> Result<()>;
}

trait WindowsBluetoothStore {
    fn read_adapters(&self) -> Result<Vec<WindowsAdapter>>;
    fn read_devices(&self, adapter: &BluetoothAddress) -> Result<Vec<WindowsDevice>>;
}

trait BluetoothService {
    fn stop(&self) -> Result<()>;
    fn start(&self) -> Result<()>;
}
```

Traits are useful for testing app flows without touching the real machine. Do not over-abstract every helper; use traits at system boundaries.

## Infrastructure Layer

The infrastructure layer is where side effects live.

### `infra::bluez`

Responsibilities:

- Read `/var/lib/bluetooth`.
- Parse BlueZ `info` files.
- Write BlueZ `info` files.
- Set permissions.
- Interact with `bluetooth.service`.
- Optionally verify with `bluetoothctl`.

Files:

```text
infra/bluez/info_file.rs
infra/bluez/store.rs
infra/bluez/service.rs
```

### `infra::windows`

Responsibilities:

- Locate `Windows/System32/config/SYSTEM`.
- Read active `ControlSet`.
- Traverse `BTHPORT\Parameters`.
- Parse `Devices` and `Keys`.
- Convert Windows `FILETIME` values.

Files:

```text
infra/windows/system_hive.rs
infra/windows/bthport.rs
infra/windows/filetime.rs
```

V1 can shell out to `hivexget` and `hivexsh`. The rest of the program should not care whether registry values came from external commands or a native parser.

### `infra::linux`

Responsibilities:

- Discover mounts with `findmnt` or `/proc/self/mountinfo`.
- Detect privilege state.
- Detect unsafe Windows mount conditions when possible.

Files:

```text
infra/linux/mounts.rs
infra/linux/privileges.rs
```

### `infra::backup`

Responsibilities:

- Create timestamped backups.
- List backups.
- Restore backups.
- Refuse destructive restore when input is invalid.

Files:

```text
infra/backup/store.rs
```

## Conversion Layer

The conversion layer is deterministic and heavily tested.

Responsibilities:

- Convert Windows `LTK`, `IRK`, `CSRK` byte arrays to BlueZ uppercase hex.
- Convert Windows `EDIV` to BlueZ decimal.
- Convert Windows `ERand` little-endian bytes to BlueZ unsigned decimal.
- Build BlueZ compatibility sections.

Files:

```text
convert/windows_to_bluez.rs
convert/endian.rs
```

This layer should not read files, call shell commands, or print output.

## CLI Layer

The CLI layer should stay thin.

Responsibilities:

- Define commands and options.
- Validate basic argument shape.
- Call app use cases.
- Render tables, warnings, and plans.
- Map errors to exit codes.

Recommended crate:

```text
clap
```

Output should be human-readable by default. JSON output can be added later:

```bash
bluebond scan --json
bluebond plan --json
```

Do not let CLI structs leak into the domain layer.

## Error Strategy

Use two levels of errors:

- `thiserror` for typed library/domain errors.
- `anyhow` for top-level command context if desired.

Recommended pattern:

```rust
#[derive(thiserror::Error, Debug)]
pub enum BluebondError {
    #[error("no Windows SYSTEM hive found")]
    NoWindowsHive,

    #[error("ambiguous device match: {0}")]
    AmbiguousMatch(String),

    #[error("invalid key length for {field}: expected {expected}, got {actual}")]
    InvalidKeyLength {
        field: &'static str,
        expected: usize,
        actual: usize,
    },
}
```

The CLI should print actionable messages:

```text
No Windows SYSTEM hive found.
Mount your Windows partition, then run bluebond scan again.
```

## Secret Handling

Bluetooth bond keys are credentials.

Rules:

- Key material should use wrapper types with redacted `Debug`.
- Do not print raw keys unless `--show-secrets` is explicitly passed.
- Test fixtures should use fake deterministic keys.
- Logs should redact secrets.

Example:

```rust
pub struct SecretKey16([u8; 16]);

impl std::fmt::Debug for SecretKey16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<redacted>")
    }
}
```

## Testing Architecture

Most tests should not require root or Bluetooth hardware.

Fixture-first tests:

```text
tests/fixtures/bluez/
tests/fixtures/windows/
tests/fixtures/command-output/
```

Test categories:

- Unit tests for address parsing.
- Unit tests for key conversion.
- Unit tests for Windows `FILETIME`.
- Parser tests for BlueZ `info`.
- Parser tests for `hivexget` output.
- Planner tests for match confidence.
- Apply tests with fake stores and fake service.
- Rollback tests with temporary directories.

Only a small number of manual integration tests should touch real Bluetooth.

## Dependency Policy

Keep dependencies boring and auditable.

Good V1 dependencies:

```toml
clap = { version = "...", features = ["derive"] }
thiserror = "..."
anyhow = "..."
serde = { version = "...", features = ["derive"] }
serde_json = "..."
tempfile = "..."
walkdir = "..."
```

Optional:

```toml
owo-colors = "..."
comfy-table = "..."
```

Avoid in V1 unless needed:

- Async runtime.
- D-Bus client.
- GUI toolkit.
- Native Windows registry parser.
- Long-running daemon.

## Why Not A Workspace Yet

Do not start with a multi-crate workspace.

BlueBond should begin as one binary crate with a library target. A workspace can be introduced when there is a real split, for example:

```text
bluebond-core
bluebond-cli
bluebond-gui
```

Starting with one crate keeps refactors cheap while the model stabilizes.

## Migration From Proposal Layout

The proposal layout:

```text
windows_registry.rs
linux_bluez.rs
planner.rs
apply.rs
backup.rs
commands.rs
```

maps to the architecture layout as:

```text
windows_registry.rs -> infra/windows/*
linux_bluez.rs      -> infra/bluez/*
planner.rs          -> app/plan.rs + domain/match_candidate.rs
apply.rs            -> app/apply.rs
backup.rs           -> infra/backup/*
commands.rs         -> infra/command.rs
```

This keeps implementation details out of the core model.

## Recommended V1 Build Order

1. Create crate, CLI shell, and error types.
2. Implement domain address and key types.
3. Implement BlueZ `info` parser/writer with tests.
4. Implement Windows `hivexget` output parser with tests.
5. Implement key conversion with tests.
6. Implement scanner.
7. Implement planner.
8. Implement backup store.
9. Implement apply flow behind explicit confirmation.
10. Implement rollback.
11. Add `doctor`.
12. Add release packaging.

## Architecture Decision

BlueBond should use a layered single-crate Rust architecture for V1:

```text
cli -> app -> domain
app -> infra
app -> convert
infra -> domain
convert -> domain
```

This gives the project a clean foundation without over-engineering it into a workspace or framework-heavy application. It fits the product: a serious, safety-sensitive system CLI with a narrow but tricky job.
