# Release Process

BlueBond uses plain Cargo release builds for early public releases.

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

BlueBond is pre-1.0. Update the package version in `Cargo.toml` and move changelog entries from `Unreleased` into a versioned section before tagging.

Suggested tag format:

```text
v0.1.0
```

## GitHub Release

Push a tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The release workflow builds the package, uploads the package/checksum as workflow artifacts, and creates or updates the GitHub release for tag pushes.

## Verification Before Tagging

Run:

```bash
./tool/verify.sh
./tool/package-release.sh
```

Do not tag from a dirty working tree.
