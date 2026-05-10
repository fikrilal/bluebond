# BlueBond End-to-End Backlog

## Purpose

This backlog is the high-level build map for BlueBond.

It is not an execution plan. Execution plans live in `docs/exec-plans/` and should be created from one focused slice of this backlog.

Use this file to answer:

- What must exist before BlueBond is a complete public project?
- What should be built first?
- What should stay out of scope until the core workflow is reliable?
- Which work requires stronger safety review?

## Product Target

BlueBond should become a safe Rust CLI that repairs dual-boot Bluetooth bond-key drift by reading Windows Bluetooth bond material and writing compatible Linux BlueZ records.

The primary V1 workflow:

1. Inspect the local Linux Bluetooth state.
2. Inspect the offline Windows installation.
3. Identify candidate adapter and device matches.
4. Build a dry-run sync plan.
5. Back up the existing BlueZ state.
6. Apply a selected sync plan.
7. Restart or reload Bluetooth safely.
8. Verify the device can reconnect.
9. Roll back from a BlueBond backup if needed.

## Guiding Rules

- Prefer read-only discovery before mutation.
- Keep `scan` and `plan` usable without root.
- Require explicit privilege for `apply` and `rollback`.
- Never write to Windows.
- Never delete existing Linux Bluetooth records in V1.
- Back up before every Linux BlueZ write.
- Fail closed when adapter or device matching is ambiguous.
- Keep shell command execution behind `infra::command`.
- Keep business rules in `domain` and orchestration in `app`.
- Keep every non-trivial slice covered by an execution plan.

## Milestone 0: Foundation

Status: done.

Goal: establish the repo, docs, architecture, harness, and Rust crate.

Completed:

- Project description and problem statement.
- Engineering proposal.
- Architecture documentation.
- Coding rules.
- Agent harness documentation.
- Execution plan workflow.
- Rust crate scaffold.
- `bluebond doctor`.
- Architecture guardrail script.
- Full verification script.

Remaining:

- Add CI once the repository is published remotely.
- Add release automation after the first useful command set exists.

## Milestone 1: Read-Only Discovery

Goal: make BlueBond able to explain the local Bluetooth situation without changing anything.

Candidate exec plans:

- `feat(scan): detect linux bluez inventory`
- `feat(scan): detect windows system hive candidates`
- `feat(scan): extract windows bluetooth key paths`
- `feat(scan): render candidate summary`

Required capabilities:

- Read Linux BlueZ adapters from `/var/lib/bluetooth`.
- Read Linux BlueZ device directories under each adapter.
- Parse BlueZ `info` files enough to extract name, alias, paired, trusted, address type, and key material presence.
- Detect mounted Windows partitions.
- Locate `Windows/System32/config/SYSTEM`.
- Use `hivex` tooling to inspect Bluetooth registry paths.
- Normalize adapter and device addresses.
- Render a human-readable scan report.
- Return clear errors for missing dependencies, missing Windows mount, and missing BlueZ state.

Acceptance:

- `bluebond scan` is read-only.
- `bluebond scan --windows-root /mnt/windows` works.
- Scan output is stable enough for users to paste into an issue.
- Fixture tests cover Linux BlueZ inventory parsing.
- Fixture tests cover Windows hive command-output parsing once extraction exists.

## Milestone 2: Domain Model And Matching

Goal: represent Bluetooth bond state explicitly and identify safe sync candidates.

Candidate exec plans:

- `feat(domain): model bluetooth adapters devices and bond keys`
- `feat(match): match windows and linux devices`
- `feat(match): classify ambiguous candidates`

Required capabilities:

- Model Bluetooth adapter identity.
- Model Bluetooth device identity.
- Model Linux BlueZ bond material.
- Model Windows BTHPORT bond material.
- Model sync candidates.
- Detect exact adapter matches.
- Detect exact device address matches.
- Detect likely identity drift for resolvable private address style changes.
- Detect same-name candidates.
- Detect ambiguous candidates.
- Explain why a candidate is safe, risky, or rejected.

Acceptance:

- Matching logic is deterministic and fixture-tested.
- Ambiguous matches never produce an applyable default plan.
- The CLI shows enough detail for a user to choose a device intentionally.

## Milestone 3: Dry-Run Planning

Goal: convert a selected candidate into an explicit no-write plan.

Candidate exec plans:

- `feat(plan): generate bluez sync plan`
- `feat(plan): render exact filesystem changes`
- `feat(plan): add json output`

Required capabilities:

- Select candidate by device name, address, or interactive index.
- Convert Windows key material into BlueZ `info` format.
- Show target adapter directory.
- Show target device directory.
- Show whether a new BlueZ record would be created or an existing one would be updated.
- Show backup location that would be used by apply.
- Show service actions that would be required.
- Support `--json` for issue reports and future GUI reuse.

Acceptance:

- `bluebond plan` makes no filesystem changes.
- The plan includes exact paths and addresses.
- Key conversion has dedicated tests.
- Planner behavior has fixture tests.

## Milestone 4: Safe Apply

Goal: perform the selected sync while protecting the user's existing BlueZ state.

Candidate exec plans:

- `feat(apply): require privileged execution`
- `feat(apply): backup bluez adapter state`
- `feat(apply): write bluez info records`
- `feat(apply): restart bluetooth service safely`
- `feat(apply): verify post-apply state`

Required capabilities:

- Refuse apply unless running with sufficient privileges.
- Stop `bluetooth.service` before writing.
- Create timestamped backup before modification.
- Write BlueZ `info` atomically where possible.
- Preserve existing records unless explicitly updating a selected target.
- Start `bluetooth.service` after writing.
- Run a basic verification pass.
- Print manual recovery instructions if service restart fails.

Acceptance:

- Apply cannot run without a dry-run plan.
- Backup is created before any write.
- Failed writes leave either the old state intact or a usable backup.
- Verification explains what was checked and what remains manual.

## Milestone 5: Rollback

Goal: make every BlueBond write reversible.

Candidate exec plans:

- `feat(rollback): list bluebond backups`
- `feat(rollback): restore selected backup`
- `feat(rollback): verify restored bluez state`

Required capabilities:

- List BlueBond-created backups.
- Show backup metadata.
- Restore a selected backup.
- Stop Bluetooth before restore.
- Start Bluetooth after restore.
- Refuse to restore unrelated paths.

Acceptance:

- Rollback uses only BlueBond backup metadata.
- Rollback never guesses backup contents.
- Restore behavior is fixture-tested where possible.

## Milestone 6: Public Project Readiness

Goal: make the project usable, understandable, and maintainable for external users.

Candidate exec plans:

- `docs(readme): write public usage guide`
- `ci(github): add rust verification workflow`
- `chore(release): add release packaging`
- `docs(safety): document risk model`
- `docs(troubleshooting): add common failure modes`

Required capabilities:

- Public README with install, usage, safety, and examples.
- Supported OS and dependency matrix.
- Known limitations.
- Troubleshooting guide.
- GitHub issue templates.
- GitHub Actions for fmt, clippy, tests, and architecture checks.
- Release build instructions.
- Versioned changelog.
- License and contribution guidance.

Acceptance:

- A new user can understand what BlueBond changes before running it.
- CI blocks architecture drift.
- Release artifacts are reproducible enough for early public use.

## Milestone 7: Post-V1 Enhancements

Goal: improve ergonomics after the safe CLI workflow exists.

Candidates:

- Interactive TUI.
- GUI wrapper.
- Native registry parsing instead of shelling out to `hivex`.
- Better Bluetooth device metadata.
- Distro-specific packaging.
- Windows helper command for exporting Bluetooth metadata.
- Multi-adapter workflows.
- Additional Bluetooth device classes.
- Automated reconnect verification.

Non-goals until V1 is solid:

- Windows-side modification.
- Automatic re-pairing.
- Kernel or firmware changes.
- Broad WiFi/Bluetooth coexistence tuning.
- General Bluetooth troubleshooting unrelated to dual-boot bond drift.

## First Pull Candidates

The next small task should come from Milestone 1.

Recommended order:

1. `feat(scan): detect linux bluez inventory`
2. `feat(scan): detect windows system hive candidates`
3. `feat(scan): extract windows bluetooth key paths`
4. `feat(domain): model bluetooth bond state`
5. `feat(match): match windows and linux devices`
6. `feat(plan): generate bluez sync plan`

The first task is intentionally Linux-only and read-only. It gives us real local state, fixture tests, and CLI output without entering privileged write behavior.

## Backlog Hygiene

- Keep this file high-level.
- Move implementation detail into exec plans.
- Move stable design decisions into `docs/engineering/`.
- Move user-facing explanations into `docs/product/` or the public README.
- Move completed implementation evidence into completed exec plans.
- Revisit this backlog after each milestone.
