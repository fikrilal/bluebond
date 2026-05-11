# Security Policy

BlueBond handles Bluetooth bond credentials. Treat all Bluetooth key material as sensitive.

## Supported Versions

BlueBond is pre-1.0. Security fixes target the current `development` branch until tagged releases begin.

## Sensitive Data

Do not post raw values for:

- `LTK`
- `IRK`
- `CSRK`
- Windows registry binary Bluetooth key material
- full BlueZ `info` files from a real machine
- full Windows `SYSTEM` hive exports

Safe issue data usually includes:

- BlueBond version or commit.
- Linux distribution and BlueZ version.
- Windows version.
- Redacted adapter/device addresses.
- Redacted `bluebond scan`, `plan`, or `apply --dry-run` output.
- Exact command used, with personal mount paths redacted if needed.

## Reporting Security Issues

If a bug can expose key material, overwrite unrelated BlueZ records, write to Windows, bypass privilege checks, or restore an unrelated backup path, report it privately first.

Use the repository owner's preferred private contact path until a dedicated security advisory workflow is configured.

## Project Safety Boundaries

BlueBond must:

- Never write to the Windows registry.
- Require root for `apply --execute` and `rollback restore`.
- Back up BlueZ files before writing.
- Redact key material in normal output and debug formatting.
- Keep mutating operations explicit.
