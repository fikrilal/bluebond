# Troubleshooting

This guide focuses on BlueBond failures, not general Bluetooth or WiFi diagnosis.

Do not paste raw Bluetooth key material into public issues.

## Before Debugging

Run:

```bash
bluebond doctor
```

Then collect:

```bash
bluebond scan --windows-root /mnt/windows
bluebond plan --windows-root /mnt/windows
bluebond apply --dry-run --windows-root /mnt/windows
```

Redact personal paths and addresses before posting output publicly.

## Windows Installation Is Not Found

Symptoms:

- `scan` finds Linux BlueZ but no Windows hive.
- `plan` has no Windows candidates.

Checks:

```bash
ls /mnt/windows/Windows/System32/config/SYSTEM
```

Fixes:

- Mount the Windows partition.
- Pass the mount explicitly with `--windows-root /mnt/windows`.
- Fully shut down Windows before mounting if Fast Startup or hibernation keeps the filesystem locked.

## Hivex Tooling Is Missing

Symptoms:

- `doctor` reports missing hivex tooling.
- Windows registry discovery fails before candidate matching.

Fix:

Install the package that provides `hivexsh` for your distribution.

Examples:

```bash
sudo apt install libguestfs-tools
sudo dnf install hivex
sudo pacman -S hivex
```

Package names vary by distribution.

## No Applyable Candidate

Symptoms:

- `plan` shows devices but no safe default apply action.
- `apply --dry-run` fails with ambiguous or missing selection.

Why it happens:

- Windows and Linux use different current Bluetooth addresses for the same physical device.
- Multiple devices have similar names.
- Adapter matching is incomplete.

Fix:

Use explicit manual selection:

```bash
bluebond apply --dry-run \
  --windows-root /mnt/windows \
  --target-device AA:BB:CC:DD:EE:FF \
  --windows-source-device AA:BB:CC:DD:EE:00
```

Only execute after the dry-run names the expected target device.

## Permission Denied During Apply Or Rollback

Symptoms:

- `apply --execute` refuses to run.
- `rollback restore` refuses to run.
- BlueZ files cannot be written.

Fix:

Use root for mutation:

```bash
sudo bluebond apply --execute --windows-root /mnt/windows
sudo bluebond rollback restore --metadata /var/lib/bluebond/backups/<snapshot>/bluebond-backup.json
```

Keep dry-run commands unprivileged when possible.

## Bluetooth Service Fails To Restart

Symptoms:

- Apply or rollback reports that `bluetooth.service` did not start.
- Bluetooth UI disappears or devices do not show.

Check:

```bash
systemctl status bluetooth.service
journalctl -u bluetooth.service -b
```

Recovery:

```bash
sudo systemctl start bluetooth.service
bluebond rollback list
sudo bluebond rollback restore --metadata /var/lib/bluebond/backups/<snapshot>/bluebond-backup.json
```

If rollback also cannot start the service, keep the service logs and open an issue with redacted BlueBond output.

## Device Still Does Not Reconnect After Apply

Symptoms:

- Apply succeeds.
- Verification says the BlueZ record exists.
- The mouse/keyboard/headset still does not connect.

Checks:

```bash
bluetoothctl devices
bluetoothctl info AA:BB:CC:DD:EE:FF
```

Try:

- Turn the device off and on.
- Restart Bluetooth from the desktop UI.
- Run `sudo systemctl restart bluetooth.service`.
- Confirm Windows has not used the device again after the BlueBond apply.

If it still fails, collect redacted `scan`, `plan`, and `apply --dry-run` output. Do not include raw key values.

## WiFi Looks Worse After Bluetooth Work

BlueBond writes BlueZ bond files and restarts Bluetooth. It does not configure WiFi, network routing, radio coexistence, power management, firmware, or NetworkManager.

If WiFi connects but has no internet:

```bash
ip route
resolvectl status
nmcli device status
```

Treat that as a separate network issue unless it only occurs immediately after `bluetooth.service` restart.

## Rollback Backup Is Not Listed

Symptoms:

- `rollback list` shows no backups.
- You expected a backup from an earlier apply.

Checks:

```bash
sudo find /var/lib/bluebond/backups -name bluebond-backup.json -print
```

Fix:

Pass the exact metadata file to restore:

```bash
sudo bluebond rollback restore --metadata /path/to/bluebond-backup.json
```

BlueBond intentionally does not restore from arbitrary directories without metadata.
