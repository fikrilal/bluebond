# Changelog

All notable changes to BlueBond will be documented in this file.

The format follows a simple human-maintained changelog. BlueBond is pre-1.0, so breaking CLI changes may still happen before the first stable release.

## Unreleased

### Fixed

- Remove stale BlueZ LE compatibility key sections when syncing Windows key material.

## v0.1.1 - 2026-05-11

### Fixed

- Report unreadable `/var/lib/bluetooth` as an actionable BlueZ permission issue instead of a generic I/O failure.
- Make `doctor` check whether the BlueZ store is readable, not only whether it exists.

### Documentation

- Clarify that read-only commands do not mutate state but may still require root on distros where BlueZ stores bond records as `0700 root:root`.

## v0.1.0 - 2026-05-11

### Added

- Rust CLI scaffold with `doctor`, `scan`, `plan`, `apply`, and `rollback` commands.
- Read-only Linux BlueZ inventory discovery.
- Read-only Windows Bluetooth registry discovery through offline `SYSTEM` hive inspection.
- Domain matching for exact and one-byte drift candidates.
- Dry-run planning and JSON plan output.
- BlueZ `info` preview rendering and Windows key-material conversion.
- Mutating apply path with privilege checks, BlueZ backups, atomic writes, Bluetooth service restart, and post-apply verification.
- Rollback listing and restore from BlueBond backup metadata.
- Public README, safety docs, troubleshooting docs, CI, and release packaging.

### Security

- Bluetooth key material is treated as secret and redacted from normal output/debug formatting.
- Fixture safety checks block accidental committed real key material patterns.
