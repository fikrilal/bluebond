# Changelog

All notable changes to BlueBond will be documented in this file.

The format follows a simple human-maintained changelog. BlueBond is pre-1.0, so breaking CLI changes may still happen before the first stable release.

## Unreleased

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
