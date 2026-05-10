# BlueBond Engineering Proposal

## Purpose

BlueBond is a Rust CLI for fixing dual-boot Bluetooth pairing problems between Windows and Linux.

The first supported workflow is:

1. Run BlueBond on Linux.
2. Read the offline Windows `SYSTEM` registry hive.
3. Extract Windows Bluetooth bond keys.
4. Match Windows Bluetooth devices to Linux BlueZ records.
5. Create a dry-run plan.
6. Back up Linux BlueZ state.
7. Write compatible BlueZ `info` records.
8. Restart Bluetooth.
9. Verify the device can reconnect.

BlueBond should be safe enough for public use because it modifies sensitive system Bluetooth state.

## Non-Goals For V1

- No GUI.
- No Windows-side writing.
- No macOS support.
- No automatic deletion of existing Linux Bluetooth records.
- No direct Windows registry modification.
- No automatic pairing flow.
- No kernel driver or firmware changes.
- No broad Bluetooth troubleshooting beyond bond-key synchronization.

## Target Users

BlueBond targets users who:

- Dual-boot Windows and Linux on the same machine.
- Use the same physical Bluetooth adapter in both operating systems.
- Have Bluetooth keyboards, mice, headphones, controllers, or other peripherals that work in one OS but fail in the other after switching.
- Are comfortable running a terminal command but do not want to manually decode registry keys and BlueZ files.

## Core Problem

Windows and Linux store Bluetooth bond keys separately.

Windows stores keys in the offline registry hive:

```text
SYSTEM\ControlSet00N\Services\BTHPORT\Parameters\Keys\<adapter-address>
```

Linux BlueZ stores keys in:

```text
/var/lib/bluetooth/<adapter-address>/<device-address>/info
```

If Windows updates or replaces the bond, Linux may still show the device as paired and trusted, but the cryptographic keys no longer match. The result is unreliable reconnects, failed manual connects, or repeated re-pairing loops.

BlueBond solves this by copying the Windows-current bond into BlueZ format.

## Product Shape

BlueBond should be a CLI-first tool.

Primary commands:

```bash
bluebond scan
bluebond plan
sudo bluebond apply --device "Legion M600 Mouse"
sudo bluebond rollback
```

Suggested output style:

```text
$ bluebond scan

Windows installations:
  [1] /mnt/windows

Linux adapters:
  [1] F8:89:D2:83:92:C0

Candidate devices:
  [1] Legion M600 Mouse
      Linux:   C6:C0:FC:F1:FB:80
      Windows: C6:C0:FD:F1:FB:80
      Status:  Windows bond appears newer
```

```text
$ bluebond plan --device "Legion M600 Mouse"

Plan:
  Read Windows SYSTEM hive:
    /mnt/windows/Windows/System32/config/SYSTEM

  Match adapter:
    Windows: f889d28392c0
    Linux:   F8:89:D2:83:92:C0

  Add BlueZ record:
    /var/lib/bluetooth/F8:89:D2:83:92:C0/C6:C0:FD:F1:FB:80/info

  Backup:
    /var/lib/bluetooth-backups/bluebond-20260510-...

No changes made.
```

```text
$ sudo bluebond apply --device "Legion M600 Mouse"

Backup created.
Bluetooth stopped.
BlueZ record written.
Bluetooth started.

Verification:
  Legion M600 Mouse: paired yes, trusted yes
```

## Safety Model

BlueBond must be conservative by default.

Required safety behavior:

- `scan` and `plan` never require root.
- `apply` and `rollback` require root.
- `plan` is dry-run by default.
- Windows registry hives are read-only.
- BlueBond never writes to Windows.
- BlueBond backs up the full Linux adapter directory before modification.
- BlueBond stops `bluetooth.service` before writing BlueZ records.
- BlueBond preserves existing Linux records unless the user explicitly requests cleanup.
- BlueBond prints exact paths and addresses before applying.
- BlueBond supports rollback.
- BlueBond fails closed when matching is ambiguous.

Ambiguous matching should produce a user-facing selection rather than a guess.

## Rust Stack

Recommended Rust stack:

```text
CLI: clap
Errors: anyhow for application flow, thiserror for domain errors
Serialization: serde, serde_json
Terminal output: owo-colors or anstream/anstyle
Tables: comfy-table or custom minimal formatting
INI parsing/writing: custom BlueZ parser/writer or configparser with strict tests
Temp files: tempfile
Filesystem walking: walkdir
Time parsing: windows-core FILETIME conversion implemented internally
Testing: cargo test with fixtures
```

Avoid large dependencies in V1. A small system utility is easier to audit.

## External Tools

V1 can shell out to existing system tools:

```text
hivexget
hivexsh
findmnt
lsblk
systemctl
bluetoothctl
```

Reasoning:

- Registry hive parsing is subtle.
- `hivex` is battle-tested for offline Windows registry reading.
- Shelling out lets V1 ship faster while keeping the core model and transformation logic in Rust.

Later versions can bind to `hivex` directly or use a native Rust registry parser if one is reliable enough.

## Repository Layout

BlueBond should use the layered single-crate architecture described in [architecture.md](../engineering/architecture.md).

Recommended layout:

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
    case-studies/
      bluetooth-dual-boot-problem.md
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
    windows_registry_tests.rs
    key_conversion_tests.rs
    bluez_info_tests.rs
    rollback_tests.rs
```

The dependency direction should stay:

```text
cli -> app -> domain
app -> infra
app -> convert
infra -> domain
convert -> domain
```

The domain layer must not know about terminal output, shell commands, root privileges, filesystem paths, `systemctl`, `bluetoothctl`, or `hivex`.

Implementation rules are defined in [coding-rules.md](../engineering/coding-rules.md). The proposal describes product and system direction; the coding rules are the day-to-day engineering contract.

## Core Data Model

Suggested domain types:

```rust
struct BluetoothAddress(String);

struct LinuxAdapter {
    address: BluetoothAddress,
    bluez_path: PathBuf,
}

struct LinuxDevice {
    adapter: BluetoothAddress,
    address: BluetoothAddress,
    name: Option<String>,
    trusted: bool,
    blocked: bool,
    bonded: bool,
    services: Vec<UuidString>,
    keys: Option<BluezKeys>,
}

struct WindowsAdapter {
    address: BluetoothAddress,
    registry_key_path: String,
}

struct WindowsDevice {
    adapter: BluetoothAddress,
    address: BluetoothAddress,
    name: Option<String>,
    vid: Option<u16>,
    pid: Option<u16>,
    appearance: Option<u16>,
    last_connected: Option<WindowsFileTime>,
    services: Vec<UuidString>,
    keys: WindowsKeys,
}

struct WindowsKeys {
    ltk: [u8; 16],
    irk: Option<[u8; 16]>,
    csrk: Option<[u8; 16]>,
    ediv: u16,
    erand: u64,
    address_type: AddressType,
}

struct BluezKeys {
    long_term_key: LongTermKey,
    identity_resolving_key: Option<[u8; 16]>,
    local_signature_key: Option<[u8; 16]>,
}
```

## Key Conversion Rules

Windows values:

```text
LTK   -> 16-byte hex
IRK   -> 16-byte hex
CSRK  -> 16-byte hex
EDIV  -> DWORD
ERand -> 8-byte little-endian value
```

BlueZ values:

```ini
[IdentityResolvingKey]
Key=<IRK uppercase hex>

[LocalSignatureKey]
Key=<CSRK uppercase hex>
Counter=0
Authenticated=false

[LongTermKey]
Key=<LTK uppercase hex>
Authenticated=0
EncSize=16
EDiv=<EDIV decimal>
Rand=<ERand as unsigned decimal after little-endian conversion>
```

BlueBond should also optionally write compatibility sections:

```ini
[PeripheralLongTermKey]
[SlaveLongTermKey]
```

These sections help with older BlueZ naming and observed real-world compatibility.

## Matching Strategy

BlueBond should score Windows and Linux device candidates using several signals:

- Exact Bluetooth address match.
- Similar Bluetooth address family or historical variants.
- Device name match.
- VID/PID match.
- BLE appearance match.
- Shared HID service UUID `00001812-0000-1000-8000-00805f9b34fb`.
- Windows `LastConnected` recency.
- Existing Linux name and services.

Example match result:

```text
Candidate: Legion M600 Mouse
Confidence: high
Reasons:
  name match
  VID/PID match
  HID service match
  Windows bond newer than Linux
  address variant differs by one byte
```

If confidence is medium or low, BlueBond should ask the user to pick a candidate explicitly.

## Apply Flow

`apply` should perform these steps:

1. Validate it is running as root.
2. Re-run or load a saved plan.
3. Verify the Windows hive still exists.
4. Verify target Linux adapter path exists.
5. Create a timestamped backup:

```text
/var/lib/bluetooth-backups/bluebond-<timestamp>-<adapter>
```

6. Stop Bluetooth:

```bash
systemctl stop bluetooth
```

7. Write BlueZ `info` records.
8. Set ownership and permissions:

```text
owner: root:root
mode: 0600 for info files
```

9. Start Bluetooth:

```bash
systemctl start bluetooth
```

10. Verify using `bluetoothctl info`.

## Rollback Flow

`rollback` should:

1. List known BlueBond backups.
2. Ask the user to select one if not specified.
3. Stop Bluetooth.
4. Restore the backed-up adapter directory.
5. Start Bluetooth.
6. Print restored paths.

Rollback should never delete backups automatically.

## Error Handling

Errors should be explicit and actionable.

Examples:

```text
No Windows SYSTEM hive found.
Try mounting your Windows partition first.
```

```text
Windows hive is unavailable because the NTFS volume appears unsafe.
Disable Windows Fast Startup and fully shut down Windows.
```

```text
Multiple matching devices found.
Run: bluebond plan --interactive
```

```text
Refusing to apply without backup.
```

## Security And Privacy

Bluetooth bond keys are credentials.

BlueBond must:

- Avoid printing raw key material by default.
- Redact keys in logs unless `--show-secrets` is explicitly passed.
- Avoid uploading telemetry.
- Avoid network access.
- Make debug bundles opt-in and redact secrets by default.

## Test Strategy

Tests should use fixtures from anonymized real data.

Required tests:

- Parse BlueZ `info` files.
- Write BlueZ `info` files.
- Parse Windows `hivexget` output.
- Convert Windows `ERand` little-endian bytes to BlueZ decimal.
- Select newest Windows device by `LastConnected`.
- Match Windows and Linux candidates by name, VID/PID, and services.
- Generate a dry-run plan.
- Refuse ambiguous plans.
- Preserve existing Linux records.
- Generate rollback metadata.

The M600 case should become a fixture, with sensitive keys replaced by deterministic fake keys.

## V1 Milestones

### Milestone 1: Read-Only Scanner

- Create Rust project.
- Implement `bluebond scan`.
- Detect Linux BlueZ adapters/devices.
- Detect mounted Windows installations.
- Read active Windows control set.
- Read Windows Bluetooth adapter/device records.

### Milestone 2: Planner

- Implement candidate matching.
- Implement key conversion.
- Print a dry-run plan.
- Add fixture-based tests.

### Milestone 3: Apply And Rollback

- Implement root checks.
- Implement backups.
- Stop/start Bluetooth.
- Write BlueZ records.
- Implement rollback.

### Milestone 4: Verification

- Run `bluetoothctl info`.
- Optional connect attempt.
- Improve user-facing diagnostics.

### Milestone 5: Packaging

- Add README install docs.
- Add GitHub Actions CI.
- Add release builds with `cargo-dist`.
- Package `.deb` later if demand exists.

## CLI Contract

Initial command set:

```bash
bluebond scan
bluebond plan [--device <name-or-address>] [--windows <path>] [--adapter <address>]
bluebond apply --device <name-or-address> [--windows <path>] [--adapter <address>]
bluebond rollback [--backup <path>]
bluebond doctor
```

`doctor` should check:

- `hivexget` available.
- `bluetoothctl` available.
- `systemctl` available.
- BlueZ data path exists.
- Windows partition visible.
- Current user privileges.
- Windows Fast Startup warning when detectable.

## Open Questions

- Should BlueBond keep fallback historical identities by default, or only write the newest Windows identity?
- Should BlueBond support BR/EDR devices in V1, or focus on BLE HID first?
- Should BlueBond use `hivexget` output or `hivexsh` scripted traversal as the stable interface?
- Should `apply` attempt a live connection, or only write and verify metadata?
- How much interactive prompting is acceptable for a serious CLI?

## Recommended V1 Policy

For V1, BlueBond should focus on BLE HID devices such as mice and keyboards.

Default behavior:

- Write the newest high-confidence Windows identity.
- Preserve old Linux identities.
- Do not remove historical identities.
- Do not attempt to pair.
- Do not print secrets.
- Require explicit confirmation before apply.

This scope is narrow enough to ship safely and directly solves the demonstrated real-world problem.
