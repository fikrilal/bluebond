#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

fail=0

report_violation() {
  local label="$1"
  printf '\nFixture violation: %s\n' "$label" >&2
  fail=1
}

if [[ -d tests/fixtures ]]; then
  key_matches="$(rg -n '^\s*Key=[0-9A-Fa-f]{32}\s*$' tests/fixtures || true)"
  if [[ -n "$key_matches" ]]; then
    disallowed_keys="$(
      printf '%s\n' "$key_matches" \
        | rg -v 'Key=00112233445566778899AABBCCDDEEFF|Key=FFEEDDCCBBAA99887766554433221100' \
        || true
    )"

    if [[ -n "$disallowed_keys" ]]; then
      printf '%s\n' "$disallowed_keys" >&2
      report_violation "Bluetooth fixture keys must use approved deterministic fake values"
    fi
  fi

  windows_hex_values="$(rg -n 'hex\([0-9]+\):([0-9A-Fa-f]{2},){15}[0-9A-Fa-f]{2}' tests/fixtures || true)"
  if [[ -n "$windows_hex_values" ]]; then
    printf '%s\n' "$windows_hex_values" >&2
    report_violation "Windows registry binary key material must not be committed as fixtures without an explicit fake allowlist"
  fi
fi

machine_paths="$(rg -n '/mnt/windows|/var/lib/bluetooth' tests --glob '*.rs' --glob '!tests/**/fixtures/**' || true)"
if [[ -n "$machine_paths" ]]; then
  printf '%s\n' "$machine_paths" >&2
  report_violation "tests must not depend on machine-local Windows or BlueZ paths"
fi

exit "$fail"
