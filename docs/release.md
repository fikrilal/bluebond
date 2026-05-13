# Release Process

BlueBond uses plain Cargo release builds and GitHub Releases for early public distribution.

## User Install Path

Public users should install from GitHub Releases rather than building from source:

```bash
curl -LO https://github.com/fikrilal/bluebond/releases/download/v0.1.2/bluebond-0.1.2-x86_64-unknown-linux-gnu.tar.gz
curl -LO https://github.com/fikrilal/bluebond/releases/download/v0.1.2/bluebond-0.1.2-x86_64-unknown-linux-gnu.tar.gz.sha256
sha256sum -c bluebond-0.1.2-x86_64-unknown-linux-gnu.tar.gz.sha256
tar -xzf bluebond-0.1.2-x86_64-unknown-linux-gnu.tar.gz
sudo install -m 0755 bluebond-0.1.2-x86_64-unknown-linux-gnu/bluebond /usr/local/bin/bluebond
bluebond --help
```

## Local Package

Build and package the current checkout:

```bash
./tool/package-release.sh
```

Outputs are written to `dist/`:

```text
bluebond-<version>-<target>.tar.gz
bluebond-<version>-<target>.tar.gz.sha256
```

The archive contains:

- `bluebond` binary
- `README.md`
- `CHANGELOG.md`
- `LICENSE`

## Versioning

Version source of truth:

```text
Cargo.toml package.version
```

Tag format:

```text
v<version>
```

For example:

```text
v0.1.2
```

The package script fails in GitHub Actions if the pushed tag does not match the Cargo version.

## Maintainer Checklist

Before tagging:

1. Update `Cargo.toml` version if needed.
2. Move changelog entries from `Unreleased` to the version section.
3. Add or update `docs/release-notes/v<version>.md`.
4. Ensure `README.md` install commands match the artifact name.
5. Run local verification:

```bash
./tool/verify.sh
./tool/package-release.sh
```

6. Commit the release prep:

```bash
git commit -m "chore(release): prepare v0.1.2"
```

7. Merge the release commit to `main`.
8. Tag `main`:

```bash
git tag -a v0.1.2 -m "v0.1.2"
git push origin v0.1.2
```

## GitHub Release Workflow

Pushing a `v*` tag triggers `.github/workflows/release.yml`.

The workflow:

1. checks out the tagged commit
2. installs Rust stable
3. runs `./tool/verify.sh`
4. runs `./tool/package-release.sh`
5. uploads the archive/checksum as workflow artifacts
6. creates or updates the GitHub Release
7. attaches the archive/checksum to the release

Release notes are loaded from:

```text
docs/release-notes/<tag>.md
```

If no tag-specific notes file exists, the workflow falls back to a short changelog pointer.

## Post-Release Validation

After GitHub publishes the release, validate the real user path:

```bash
tmpdir="$(mktemp -d)"
cd "$tmpdir"
curl -LO https://github.com/fikrilal/bluebond/releases/download/v0.1.2/bluebond-0.1.2-x86_64-unknown-linux-gnu.tar.gz
curl -LO https://github.com/fikrilal/bluebond/releases/download/v0.1.2/bluebond-0.1.2-x86_64-unknown-linux-gnu.tar.gz.sha256
sha256sum -c bluebond-0.1.2-x86_64-unknown-linux-gnu.tar.gz.sha256
tar -xzf bluebond-0.1.2-x86_64-unknown-linux-gnu.tar.gz
./bluebond-0.1.2-x86_64-unknown-linux-gnu/bluebond --version
./bluebond-0.1.2-x86_64-unknown-linux-gnu/bluebond --help
```

Do not announce the release until this passes.
