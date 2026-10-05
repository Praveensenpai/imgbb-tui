#!/usr/bin/env bash
#
# imgbb-tui installer
# Downloads the latest prebuilt Linux x86_64 release, verifies its checksum,
# and installs the binary to ~/.local/bin.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/Praveensenpai/imgbb-tui/main/scripts/install.sh | bash

set -euo pipefail
IFS=$'\n\t'

REPO="Praveensenpai/imgbb-tui"
BIN="imgbb-tui"
TARGET="x86_64-unknown-linux-gnu"
VERSION="latest"
PREFIX="${HOME}/.local/bin"
FROM_SOURCE=0

usage() {
  cat <<'EOF'
imgbb-tui installer

Usage:
  install.sh [options]

Options:
  --version <tag>   Release tag to install (default: latest)
  --prefix <dir>    Install directory (default: ~/.local/bin)
  --from-source     Build from source instead of downloading a release
  -h, --help        Show this help
EOF
}

parse_args() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --version)
        VERSION="${2:?--version requires a value}"
        shift 2
        ;;
      --prefix)
        PREFIX="${2:?--prefix requires a value}"
        shift 2
        ;;
      --from-source)
        FROM_SOURCE=1
        shift
        ;;
      -h|--help)
        usage
        exit 0
        ;;
      *)
        echo "Unknown option: $1" >&2
        usage >&2
        exit 1
        ;;
    esac
  done
}

detect_platform() {
  local os arch
  os="$(uname -s)"
  arch="$(uname -m)"
  if [[ "${os}" != "Linux" ]]; then
    echo "Error: unsupported OS '${os}'. Only Linux is supported." >&2
    exit 1
  fi
  if [[ "${arch}" != "x86_64" ]]; then
    echo "Error: unsupported architecture '${arch}'. Only x86_64 is supported." >&2
    exit 1
  fi
}

release_url() {
  if [[ "${VERSION}" == "latest" ]]; then
    echo "https://github.com/${REPO}/releases/latest/download"
  else
    echo "https://github.com/${REPO}/releases/download/${VERSION}"
  fi
}

build_from_source() {
  command -v cargo >/dev/null 2>&1 || {
    echo "Error: cargo not found. Install Rust from https://rustup.rs/ to build from source." >&2
    exit 1
  }
  local tmp
  tmp="$(mktemp -d)"
  echo "Building ${BIN} from source (${TARGET}) ..."
  git clone --depth 1 "https://github.com/${REPO}.git" "${tmp}/src"
  ( cd "${tmp}/src" && cargo build --release --target "${TARGET}" )
  install -Dm 755 "${tmp}/src/target/${TARGET}/release/${BIN}" "${PREFIX}/${BIN}"
}

install_from_release() {
  local tmp base archive checksum
  tmp="$(mktemp -d)"
  base="$(release_url)"
  archive="${BIN}-x86_64-linux.tar.gz"
  checksum="${archive}.sha256"

  echo "Downloading ${archive} (${VERSION}) ..."
  if ! curl -fsSL "${base}/${archive}" -o "${tmp}/${archive}"; then
    echo "No prebuilt release asset found; falling back to building from source." >&2
    build_from_source
    return
  fi

  if curl -fsSL "${base}/${checksum}" -o "${tmp}/${checksum}" 2>/dev/null; then
    ( cd "${tmp}" && sha256sum -c "${checksum}" )
  else
    echo "Warning: checksum file unavailable, skipping verification." >&2
  fi

  tar -xzf "${tmp}/${archive}" -C "${tmp}"
  install -Dm 755 "${tmp}/${BIN}" "${PREFIX}/${BIN}"
}

main() {
  parse_args "$@"
  detect_platform
  mkdir -p "${PREFIX}"
  if [[ "${FROM_SOURCE}" -eq 1 ]]; then
    build_from_source
  else
    install_from_release
  fi
  echo ""
  echo "Installed ${BIN} -> ${PREFIX}/${BIN}"
  if ! command -v "${BIN}" >/dev/null 2>&1; then
    echo "Note: ${PREFIX} is not on your PATH. Add it with:"
    echo "  export PATH=\"${PREFIX}:\$PATH\""
  fi
  echo "Run it with: ${BIN}"
}

main "$@"
