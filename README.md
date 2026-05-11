# BlueBond

BlueBond is a Rust CLI for repairing Bluetooth pairing drift on Linux/Windows dual-boot machines.

Some Bluetooth devices store different bond material per operating system. If a mouse, keyboard, or headset is paired and used in Windows, Linux BlueZ may keep an older bond record and fail to reconnect. BlueBond reads the offline Windows Bluetooth bond data, converts compatible key material, and writes a matching BlueZ `info` record on Linux.

BlueBond is intentionally conservative:

- `doctor`, `scan`, `plan`, and `apply --dry-run` are read-only.
- `apply --execute` and `rollback restore` require root.
- BlueBond never writes to Windows.
- BlueBond backs up Linux BlueZ records before writing.
- BlueBond does not print raw Bluetooth keys.

Read-only means BlueBond does not change system state. It does not guarantee the command can run without privileges: many Linux distributions protect `/var/lib/bluetooth` as `0700 root:root`, so `scan`, `plan`, and `apply --dry-run` may need `sudo` to inspect real BlueZ bond records.

## Status

BlueBond is early public software. It has been built around a real Legion M600 dual-boot case, fixture tests, and an explicit safety workflow, but it is not a general Bluetooth repair tool.

Use it only if the failure pattern matches this project:

1. The device works after pairing in Linux.
2. The device also works after pairing or using it in Windows.
3. Returning to Linux makes auto-connect fail or makes manual connect unreliable.
4. Re-pairing temporarily fixes Linux until Windows uses the device again.

## Supported Platform

Current V1 target:

- Linux host using BlueZ.
- Offline Windows installation mounted read-only or read/write by the OS.
- Bluetooth bond data stored in the Windows `SYSTEM` registry hive.
- `hivexsh` or compatible hivex tooling available on Linux.
- `systemctl` available for restarting `bluetooth.service` during apply and rollback.

Known practical target:

- Linux plus Windows dual boot on the same machine.
- Bluetooth LE HID devices such as mice and keyboards.

Not supported in V1:

- Writing Windows registry data.
- Automatic Windows-side repair.
- Kernel, firmware, or WiFi/Bluetooth coexistence tuning.
- Broad Bluetooth troubleshooting unrelated to dual-boot bond drift.
- GUI or TUI flows.

## Install From GitHub Releases

Prerequisites:

- BlueZ.
- `hivexsh`.
- `systemctl`.
- Root access for mutating commands.

Download and verify the Linux x86_64 archive:

```bash
curl -LO https://github.com/fikrilal/bluebond/releases/download/v0.1.1/bluebond-0.1.1-x86_64-unknown-linux-gnu.tar.gz
curl -LO https://github.com/fikrilal/bluebond/releases/download/v0.1.1/bluebond-0.1.1-x86_64-unknown-linux-gnu.tar.gz.sha256
sha256sum -c bluebond-0.1.1-x86_64-unknown-linux-gnu.tar.gz.sha256
```

Extract and install:

```bash
tar -xzf bluebond-0.1.1-x86_64-unknown-linux-gnu.tar.gz
sudo install -m 0755 bluebond-0.1.1-x86_64-unknown-linux-gnu/bluebond /usr/local/bin/bluebond
bluebond --help
```

## Install From Source

Source builds require the Rust stable toolchain.

```bash
git clone https://github.com/fikrilal/bluebond.git
cd bluebond
cargo build --release
./target/release/bluebond --help
```

Package a local release archive:

```bash
./tool/package-release.sh
```

The package and checksum are written under `dist/`.

## Safe Workflow

Check host readiness:

```bash
bluebond doctor
```

Inspect Linux BlueZ and Windows Bluetooth inventory:

```bash
bluebond scan --windows-root /mnt/windows
```

If `/var/lib/bluetooth` is not readable by your user, run read-only inspection with `sudo`:

```bash
sudo bluebond scan --windows-root /mnt/windows
```

Build a read-only plan:

```bash
bluebond plan --windows-root /mnt/windows
```

Use `sudo` here too if the BlueZ store is protected:

```bash
sudo bluebond plan --windows-root /mnt/windows
```

Preview exact apply inputs and target files:

```bash
bluebond apply --dry-run --windows-root /mnt/windows
```

Use `sudo` for dry-run when BlueBond needs to inspect protected BlueZ bond records:

```bash
sudo bluebond apply --dry-run --windows-root /mnt/windows
```

If matching is ambiguous, choose the Linux target device and Windows source device explicitly:

```bash
bluebond apply --dry-run \
  --windows-root /mnt/windows \
  --target-device AA:BB:CC:DD:EE:FF \
  --windows-source-device AA:BB:CC:DD:EE:00
```

Execute only after the dry-run shows the expected target:

```bash
sudo bluebond apply --execute \
  --windows-root /mnt/windows \
  --target-device AA:BB:CC:DD:EE:FF \
  --windows-source-device AA:BB:CC:DD:EE:00
```

After apply, reconnect the device from Bluetooth settings or `bluetoothctl`. BlueBond verifies that the expected BlueZ record exists and contains key material, but the final radio reconnect is still a manual check in V1.

## Rollback

List BlueBond backups:

```bash
bluebond rollback list
```

Restore one backup using its metadata file:

```bash
sudo bluebond rollback restore \
  --metadata /var/lib/bluebond/backups/<snapshot>/bluebond-backup.json
```

Rollback only uses BlueBond-created metadata. It does not guess backup contents from directory names.

## Documentation

- [Documentation index](docs/README.md)
- [Safety model](docs/engineering/safety.md)
- [Troubleshooting](docs/troubleshooting.md)
- [Architecture](docs/engineering/architecture.md)
- [Coding rules](docs/engineering/coding-rules.md)
- [Dual-boot Bluetooth case study](docs/case-studies/bluetooth-dual-boot-problem.md)

## Development

Run the full local harness:

```bash
./tool/verify.sh
```

The harness checks documentation structure, architecture boundaries, fixture safety, formatting, clippy, and tests.

## Security

Bluetooth bond keys are credentials. Do not paste raw `LTK`, `IRK`, `CSRK`, or full registry exports into public issues.

See [SECURITY.md](SECURITY.md) for reporting guidance.

## License

MIT. See [LICENSE](LICENSE).
