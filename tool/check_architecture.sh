#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

if [[ ! -d src ]]; then
  printf 'Skipping architecture checks: src/ does not exist yet.\n'
  exit 0
fi

fail=0

check_forbidden() {
  local label="$1"
  local path="$2"
  local pattern="$3"

  if [[ ! -d "$path" ]]; then
    return 0
  fi

  if rg -n "$pattern" "$path"; then
    printf '\nArchitecture violation: %s\n' "$label" >&2
    fail=1
  fi
}

check_forbidden \
  "domain must not import infra" \
  "src/domain" \
  'crate::infra|super::super::infra'

check_forbidden \
  "domain must not import cli" \
  "src/domain" \
  'crate::cli|super::super::cli'

check_forbidden \
  "infra must not import cli" \
  "src/infra" \
  'crate::cli|super::super::cli'

if rg -n 'std::process::Command|process::Command' src --glob '*.rs' \
  --glob '!src/infra/command.rs' \
  --glob '!src/infra/command/*.rs'
then
  printf '\nArchitecture violation: command execution must go through infra::command\n' >&2
  fail=1
fi

if rg -n '/var/lib/bluetooth|var/lib/bluetooth' src --glob '*.rs' \
  --glob '!src/infra/bluez/store.rs' \
  --glob '!src/infra/bluez/store/*.rs' \
  --glob '!src/infra/backup/store.rs' \
  --glob '!src/infra/backup/store/*.rs'
then
  printf '\nArchitecture violation: BlueZ store paths must stay in infra::bluez::store\n' >&2
  fail=1
fi

exit "$fail"
