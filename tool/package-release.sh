#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

package_name="bluebond"
version="$(cargo metadata --no-deps --format-version 1 | sed -n 's/.*"version":"\([^"]*\)".*/\1/p' | head -n 1)"

if [[ -z "$version" ]]; then
  printf 'Unable to determine package version from Cargo metadata.\n' >&2
  exit 1
fi

if [[ "${GITHUB_REF_TYPE:-}" == "tag" && "${GITHUB_REF_NAME:-}" != "v${version}" ]]; then
  printf 'Tag/version mismatch: tag is %s but Cargo version is %s.\n' "${GITHUB_REF_NAME}" "${version}" >&2
  exit 1
fi

target_triple="${TARGET:-$(rustc -vV | sed -n 's/^host: //p')}"
archive_name="${package_name}-${version}-${target_triple}"
dist_dir="dist"
package_dir="${dist_dir}/${archive_name}"
binary_path="target/release/${package_name}"

if [[ -n "${TARGET:-}" ]]; then
  binary_path="target/${TARGET}/release/${package_name}"
fi

rm -rf "$dist_dir"
mkdir -p "$package_dir"

if [[ -n "${TARGET:-}" ]]; then
  cargo build --release --target "$TARGET"
else
  cargo build --release
fi

cp "$binary_path" "$package_dir/"
cp README.md CHANGELOG.md LICENSE "$package_dir/"

tarball="${dist_dir}/${archive_name}.tar.gz"
rm -f "$tarball" "${tarball}.sha256"
tar -C "$dist_dir" -czf "$tarball" "$archive_name"
(cd "$dist_dir" && sha256sum "${archive_name}.tar.gz" > "${archive_name}.tar.gz.sha256")

printf 'Created %s\n' "$tarball"
printf 'Created %s\n' "${tarball}.sha256"
