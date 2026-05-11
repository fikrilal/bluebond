#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

step() {
  printf '\n==> %s\n' "$1"
}

step "Verify documentation structure"
required_docs=(
  "AGENTS.md"
  "docs/README.md"
  "docs/product/engineering-proposal.md"
  "docs/engineering/architecture.md"
  "docs/engineering/coding-rules.md"
  "docs/harness/agent-harness.md"
  "docs/case-studies/bluetooth-dual-boot-problem.md"
  "docs/exec-plans/README.md"
  "docs/exec-plans/_template.md"
  "docs/exec-plans/tech-debt-tracker.md"
)

for path in "${required_docs[@]}"; do
  if [[ ! -f "$path" ]]; then
    printf 'Missing required doc: %s\n' "$path" >&2
    exit 1
  fi
done

step "Verify architecture guardrails"
./tool/check_architecture.sh

step "Verify fixture safety"
./tool/check_fixtures.sh

if [[ -f Cargo.toml ]]; then
  step "Cargo fmt"
  cargo fmt --check

  step "Cargo clippy"
  cargo clippy --all-targets --all-features -- -D warnings

  step "Cargo test"
  cargo test --all-features
else
  step "Cargo checks"
  printf 'Skipping Rust checks: Cargo.toml does not exist yet.\n'
fi

printf '\nOK\n'
