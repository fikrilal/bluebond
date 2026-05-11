# BlueBond Safety Model

BlueBond repairs a narrow dual-boot Bluetooth failure by copying compatible bond material from an offline Windows installation into Linux BlueZ records.

That is powerful enough to require strict boundaries.

## Safety Goals

- Make read-only inspection useful before any mutation.
- Make every BlueZ write explicit and reviewable.
- Back up existing BlueZ state before writing.
- Keep rollback tied to BlueBond metadata, not path guessing.
- Never write to Windows.
- Never print raw Bluetooth key material in normal output.

## Command Safety Classes

Read-only commands:

- `bluebond doctor`
- `bluebond scan`
- `bluebond plan`
- `bluebond apply --dry-run`
- `bluebond rollback list`

Read-only means these commands do not mutate Linux or Windows state. It does not guarantee they can run without privileges. On many Linux systems, `/var/lib/bluetooth` is `0700 root:root`, so commands that inspect real BlueZ bond records may still need `sudo`.

Mutating commands:

- `bluebond apply --execute`
- `bluebond rollback restore --metadata <path>`

Mutating commands require root because they write under BlueZ-owned system paths and restart `bluetooth.service`.

## What BlueBond Reads

Linux:

- BlueZ adapter directories.
- BlueZ device directories.
- BlueZ `info` files.
- Bluetooth service state through system tooling.

Windows:

- Offline `Windows/System32/config/SYSTEM` registry hive.
- Bluetooth adapter and device key paths under the hive.
- Bluetooth bond key material required to generate BlueZ-compatible records.

BlueBond reads Windows data only. Windows registry writes are not allowed in V1.

## What BlueBond Writes

BlueBond writes Linux-side files only:

- BlueBond backup snapshots under the configured backup root.
- BlueZ `info` files for the selected target adapter/device.
- Backup metadata files describing the operation.

BlueBond does not delete unrelated BlueZ records in V1.

## Backup Contract

Before apply writes a BlueZ `info` record, BlueBond creates a backup snapshot containing:

- snapshot timestamp and ID
- BlueBond version
- operation summary
- target BlueZ paths
- source Windows registry paths
- copied backup files

Rollback consumes `bluebond-backup.json` metadata. It refuses to infer restore contents from directory names.

## Matching Safety

Automatic matching is intentionally conservative:

- Exact adapter/device matches are safest.
- One-byte drift candidates are treated as candidates, not blind defaults.
- Ambiguous matches require explicit user selection.
- Manual selection requires both Linux target device and Windows source device.

## Service Safety

For mutation, BlueBond stops `bluetooth.service`, writes files, and starts the service again.

If service start fails, do not keep retrying random Bluetooth commands. Check:

```bash
systemctl status bluetooth.service
journalctl -u bluetooth.service -b
```

Then use rollback if the BlueZ state should be restored.

## Secret Handling

Bluetooth bond keys are credentials. BlueBond should expose presence, type, source, and path information without printing raw key bytes.

Public bug reports must not include raw `LTK`, `IRK`, `CSRK`, full BlueZ `info` files from real devices, or full Windows hive exports.

## Known Residual Risks

- A valid write can still fail to reconnect if the device has changed identity again.
- Radio state, firmware bugs, device power state, or OS Bluetooth stack bugs can prevent reconnect after a correct file update.
- Distro-specific BlueZ behavior may require manual service recovery.
- V1 verification confirms records and key presence; it does not prove a real radio reconnect.
