# Agent Harness

## Purpose

BlueBond is expected to be built mostly through agent execution with a human acting as commander, reviewer, and product owner.

That changes what the repository needs first. Before major product code exists, the repo needs a harness that makes agent work:

- legible
- bounded
- mechanically verifiable
- recoverable
- easy to continue across sessions

The harness is the development environment, docs map, verification scripts, execution-plan workflow, and architecture guardrails that let agents safely make progress.

## Principles

### `AGENTS.md` Is A Map, Not A Manual

Root `AGENTS.md` should stay short. It points agents to the source-of-truth docs.

Detailed rules belong in:

- `docs/engineering/architecture.md`
- `docs/engineering/coding-rules.md`
- `docs/harness/agent-harness.md`
- `docs/exec-plans/`
- ADRs under `docs/adr/`

### Repository Knowledge Is The System Of Record

If an agent needs to know something repeatedly, encode it in the repository.

Examples:

- architecture rules
- command flows
- safety policy
- key conversion rules
- verification commands
- execution plans
- known debt

Do not rely on chat history as durable memory.

### Enforcement Beats Reminder Text

When a rule matters, prefer a script, lint, test, or fixture over prose alone.

Examples:

- architecture boundaries -> `tool/check_architecture.sh`
- standard verification -> `tool/verify.sh`
- key conversion rules -> fixture tests
- recurring docs drift -> docs index and path checks

### Agent Legibility Is A Product Requirement

Code should be readable by future agent runs, not only humans.

That means:

- stable paths
- explicit boundaries
- small modules
- predictable naming
- source-local docs where useful
- minimal hidden state
- boring dependencies

### Promote Repeated Feedback Into The Harness

If the same issue appears twice, it should become one of:

- a coding rule
- a verification script
- a test fixture
- a scaffold/template update
- an ADR
- an execution-plan checklist item

## Harness Components

### Docs Map

Current docs map:

```text
docs/
  README.md
  product/
  engineering/
  harness/
  case-studies/
  exec-plans/
  adr/
```

The docs index should be enough for a fresh agent to find the right context without reading every file.

### Execution Plans

Non-trivial tasks should create a plan under:

```text
docs/exec-plans/active/
```

Plans are used when work includes:

- multiple implementation steps
- system state mutation
- architecture decisions
- risky behavior
- long-running work
- expected follow-ups

Tiny docs edits do not need plans.

### Verification Scripts

Canonical command:

```bash
./tool/verify.sh
```

This should become the one command agents run before claiming completion.

Current verification stages:

- docs structure checks
- architecture checks
- fixture safety checks
- Rust formatting once `Cargo.toml` exists
- Rust clippy once `Cargo.toml` exists
- Rust tests once `Cargo.toml` exists

### Architecture Checks

Architecture checks should start small and grow with the codebase.

Initial checks:

- `domain` must not import `infra`, `app`, or `cli`
- `domain` must not use filesystem, path, process, or environment APIs
- `app` must not import `cli`
- `infra` must not import `app` or `cli`
- `cli` must not import `infra` directly
- direct `std::process::Command` must only appear in `infra::command`
- direct BlueZ writes must only appear in `infra::bluez::store`

These checks are intentionally simple. They are meant to catch easy drift early, not replace Rust's type system or code review.

### Fixtures

Fixtures should become the main way agents validate tricky behavior without relying on local machine state.

Planned fixture areas:

```text
tests/fixtures/bluez/
tests/fixtures/windows/
tests/fixtures/command-output/
```

The real Legion M600 case should be converted into anonymized fixtures with fake deterministic keys.

Fixture safety checks live in:

```bash
./tool/check_fixtures.sh
```

The fixture checker currently rejects:

- unapproved 16-byte BlueZ key fixture values
- Windows registry binary key material committed as fixture text without an explicit fake allowlist
- test code that depends on machine-local paths such as `/mnt/windows` or `/var/lib/bluetooth`

Smoke tests against real machine state are allowed during development, but they must stay out of automated tests and committed fixtures.

## Initial Harness Build Order

1. Keep `AGENTS.md` short and map-like.
2. Maintain `docs/README.md` as the docs index.
3. Add `docs/exec-plans/` workflow.
4. Add `tool/verify.sh`.
5. Add `tool/check_architecture.sh`.
6. Add Rust crate only after the harness can verify itself.
7. Add fixture-based tests before mutating commands.

## Human Commander Workflow

For non-trivial tasks, the human should be able to say:

```text
Create an execution plan for X, then implement it and run the harness.
```

The agent should then:

1. Read `AGENTS.md`.
2. Read relevant docs only.
3. Create or update an execution plan.
4. Implement the change.
5. Run `./tool/verify.sh`.
6. Update the plan with verification evidence.
7. Summarize outcome and remaining risk.

## Harness Debt

Harness debt is real debt.

Examples:

- verification scripts that are too slow
- docs that point to moved files
- architecture checks with noisy false positives
- missing fixtures for bugs we already saw
- commands that agents cannot run locally

Track unresolved harness debt in:

```text
docs/exec-plans/tech-debt-tracker.md
```
