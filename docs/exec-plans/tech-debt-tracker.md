# Tech Debt Tracker

Track known follow-up work that should not be forgotten.

## Harness

- Add docs link validation once documentation grows.
- Add fixture redaction checks before committing real Bluetooth key examples.

## Product

- Create anonymized fixtures from the Legion M600 case.
- Document Windows Fast Startup and BitLocker limitations.
- Decide whether unprivileged `scan` should use D-Bus/bluetoothctl for public inventory and reserve raw BlueZ key inspection for privileged `plan` or `apply`.

## Engineering

- Decide whether V1 writes only the newest Windows identity or preserves historical identities by default.
