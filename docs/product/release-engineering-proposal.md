# BlueBond Release Engineering Proposal

## Purpose

BlueBond should be usable by people who do not have Rust installed and do not want to build from source.

The release system should turn a verified Git commit into a downloadable GitHub Release with:

- a Linux binary archive
- a checksum
- clear install instructions
- release notes
- a reproducible local packaging path

This proposal defines the end-to-end release workflow needed before BlueBond can be comfortably shared with external users.

## Problem

The current public repository is visible and documented, but there is no published release.

That means a user must currently:

1. install Rust
2. clone the repository
3. build with Cargo
4. find the binary under `target/release`
5. manually decide how to install it

That is acceptable for contributors, but it is too much friction for users who only want to repair a dual-boot Bluetooth pairing issue.

## Target User Experience

A user should be able to install BlueBond from GitHub Releases:

```bash
curl -LO https://github.com/fikrilal/bluebond/releases/download/v0.1.0/bluebond-0.1.0-x86_64-unknown-linux-gnu.tar.gz
curl -LO https://github.com/fikrilal/bluebond/releases/download/v0.1.0/bluebond-0.1.0-x86_64-unknown-linux-gnu.tar.gz.sha256
sha256sum -c bluebond-0.1.0-x86_64-unknown-linux-gnu.tar.gz.sha256
tar -xzf bluebond-0.1.0-x86_64-unknown-linux-gnu.tar.gz
cd bluebond-0.1.0-x86_64-unknown-linux-gnu
sudo install -m 0755 bluebond /usr/local/bin/bluebond
bluebond --help
```

Then the actual safe workflow is:

```bash
bluebond doctor
bluebond scan --windows-root /mnt/windows
bluebond plan --windows-root /mnt/windows
bluebond apply --dry-run --windows-root /mnt/windows
```

Mutation remains explicit:

```bash
sudo bluebond apply --execute --windows-root /mnt/windows
```

## Release Principles

- Release from a clean, verified commit.
- Release from `main`, not from a local development branch.
- Use immutable version tags such as `v0.1.0`.
- Build artifacts in GitHub Actions, not only on a developer machine.
- Publish checksums beside binary archives.
- Keep release notes human-readable and safety-oriented.
- Keep the source-build path documented for contributors.
- Do not introduce heavy release tooling until the plain workflow becomes painful.

## Scope For First Public Release

The first release should support:

- Linux `x86_64-unknown-linux-gnu`
- one `.tar.gz` archive
- one `.sha256` checksum file
- GitHub Release notes
- local package generation through `tool/package-release.sh`
- CI verification before packaging

The first release does not need:

- `.deb` packaging
- AUR packaging
- Homebrew formula
- Windows binary
- macOS binary
- installer script
- automatic dependency installation
- signed artifacts

Those can come after real external usage validates the CLI workflow.

## Versioning

BlueBond is pre-1.0. The first public tag should be:

```text
v0.1.0
```

Version source of truth:

```text
Cargo.toml package.version
```

Rules:

- Tag names must match `v<version>` from `Cargo.toml`.
- `CHANGELOG.md` must move relevant entries from `Unreleased` to the tagged version before release.
- Release archive names must include version and target triple.
- Breaking CLI changes are allowed before `1.0`, but they must be documented in the changelog.

## Artifact Contract

For `v0.1.0`, publish:

```text
bluebond-0.1.0-x86_64-unknown-linux-gnu.tar.gz
bluebond-0.1.0-x86_64-unknown-linux-gnu.tar.gz.sha256
```

Archive contents:

```text
bluebond-0.1.0-x86_64-unknown-linux-gnu/
  bluebond
  README.md
  CHANGELOG.md
  LICENSE
```

The binary should be built with:

```bash
cargo build --release
```

## Release Workflow

### 1. Prepare The Release Commit

Update:

- `Cargo.toml` version if needed
- `CHANGELOG.md`
- `README.md` release/install instructions if the artifact name changes
- `docs/release.md` if the process changes

Run:

```bash
./tool/verify.sh
./tool/package-release.sh
```

Expected result:

```text
OK
dist/bluebond-<version>-<target>.tar.gz
dist/bluebond-<version>-<target>.tar.gz.sha256
```

Commit:

```bash
git add Cargo.toml CHANGELOG.md README.md docs/release.md
git commit -m "chore(release): prepare v0.1.0"
```

### 2. Merge To Main

Fetch and inspect:

```bash
git fetch origin
git checkout main
git pull origin main
git log --oneline origin/main..origin/development
```

If `development` contains release commits not on `main`, merge:

```bash
git merge --no-ff origin/development
git push origin main
```

Release tags should point at `main`.

### 3. Create The Tag

Validate the version:

```bash
cargo metadata --no-deps --format-version 1
```

Create and push the tag:

```bash
git tag -a v0.1.0 -m "v0.1.0"
git push origin v0.1.0
```

### 4. GitHub Actions Builds The Release

The release workflow should:

1. check out the tagged commit
2. install Rust stable
3. run `./tool/verify.sh`
4. run `./tool/package-release.sh`
5. upload workflow artifacts
6. create or update the GitHub Release
7. attach the `.tar.gz` and `.sha256` files

If the workflow fails, fix forward with a new commit and new tag. Do not move an already-pushed public tag unless the release has not been consumed and the correction is explicitly documented.

### 5. Validate The Published Release

After GitHub publishes the release, test from a clean temporary directory:

```bash
tmpdir="$(mktemp -d)"
cd "$tmpdir"
curl -LO https://github.com/fikrilal/bluebond/releases/download/v0.1.0/bluebond-0.1.0-x86_64-unknown-linux-gnu.tar.gz
curl -LO https://github.com/fikrilal/bluebond/releases/download/v0.1.0/bluebond-0.1.0-x86_64-unknown-linux-gnu.tar.gz.sha256
sha256sum -c bluebond-0.1.0-x86_64-unknown-linux-gnu.tar.gz.sha256
tar -xzf bluebond-0.1.0-x86_64-unknown-linux-gnu.tar.gz
./bluebond-0.1.0-x86_64-unknown-linux-gnu/bluebond --version
./bluebond-0.1.0-x86_64-unknown-linux-gnu/bluebond --help
```

This validates the real user path, not just the CI path.

## GitHub Release Notes Template

````markdown
## BlueBond v0.1.0

First public preview release.

BlueBond is a Rust CLI for repairing Linux/Windows dual-boot Bluetooth bond-key drift by reading Windows Bluetooth bond material and writing compatible Linux BlueZ records.

### Install

```bash
curl -LO https://github.com/fikrilal/bluebond/releases/download/v0.1.0/bluebond-0.1.0-x86_64-unknown-linux-gnu.tar.gz
curl -LO https://github.com/fikrilal/bluebond/releases/download/v0.1.0/bluebond-0.1.0-x86_64-unknown-linux-gnu.tar.gz.sha256
sha256sum -c bluebond-0.1.0-x86_64-unknown-linux-gnu.tar.gz.sha256
tar -xzf bluebond-0.1.0-x86_64-unknown-linux-gnu.tar.gz
sudo install -m 0755 bluebond-0.1.0-x86_64-unknown-linux-gnu/bluebond /usr/local/bin/bluebond
```

### Safety

- `doctor`, `scan`, `plan`, and `apply --dry-run` are read-only.
- `apply --execute` and `rollback restore` require root.
- BlueBond never writes to Windows.
- BlueBond creates backup metadata before writing BlueZ records.
- Raw Bluetooth key material is not printed in normal output.

### First Command

```bash
bluebond doctor
```
````

## Repository Changes Needed

The current project is close, but a complete release pass should still add or verify:

- `CHANGELOG.md` has a `v0.1.0` section.
- `docs/release.md` includes the exact maintainer checklist.
- `README.md` points users to GitHub Releases as the preferred install path.
- Release workflow creates a GitHub Release on tag push.
- Release workflow attaches both archive and checksum.
- Release workflow uses the same package script tested locally.
- Local package script fails clearly if the binary is missing.

## Risks

### Bad Binary Published

Mitigation:

- CI runs full verification before packaging.
- Maintainer validates downloaded release artifact after publish.

### Users Run Mutating Commands Too Quickly

Mitigation:

- README and release notes lead with `doctor`, `scan`, `plan`, and `apply --dry-run`.
- `apply --execute` remains explicit and privileged.

### Tag Points At Wrong Branch

Mitigation:

- Release checklist requires tagging `main`.
- Maintainer checks `git branch --contains v0.1.0` after tagging.

### Linux Distro Compatibility

Mitigation:

- Start with `x86_64-unknown-linux-gnu`.
- Document runtime dependencies.
- Add distro packages only after external feedback.

## Acceptance Criteria

The release system is done when:

- `main` contains the release commit.
- `v0.1.0` tag exists on GitHub.
- GitHub Releases shows `v0.1.0`.
- The release has `.tar.gz` and `.sha256` assets.
- A clean machine can download, verify, extract, and run `bluebond --help`.
- README installation instructions match the published artifact.
- The old source-build path remains available for contributors.

## Recommended Next Execution Plans

1. `chore(release): prepare v0.1.0 metadata`
   - Move changelog entries under `v0.1.0`.
   - Update README install section to prefer GitHub Releases.
   - Tighten `docs/release.md` into a checklist.

2. `ci(release): validate tag artifact publishing`
   - Review and harden `.github/workflows/release.yml`.
   - Ensure artifact names match README.
   - Ensure release notes are useful.

3. `chore(release): tag and publish v0.1.0`
   - Merge to `main`.
   - Push annotated tag.
   - Validate downloaded release assets.

## Future Work

- Add `.deb` package.
- Add AUR package.
- Add install script with checksum verification.
- Add artifact signing.
- Add `aarch64-unknown-linux-gnu`.
- Add cargo-dist only if manual packaging becomes too costly.
