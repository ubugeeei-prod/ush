#!/bin/sh
# Generated from 'test_install.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: test_install.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'test_install.ush' --checked-in -o 'test_install.sh'
set -eu

__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) __ush_entrypoint=$(command -v "$__ush_entrypoint") ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'test_install.ush'
__ush_origin='test_install.ush'

__ush_at() { __ush_line=$1; __ush_source=$2; }
__ush_report() {
  if [ "$1" -ne 0 ] && [ -n "${__ush_line:-}" ]; then
    __ush_message=${2:-command failed}
    if [ "$#" -lt 2 ]; then
      case "$1" in
        127) __ush_message='command not found; check the command name and PATH' ;;
        126) __ush_message='command cannot be executed; check permissions and interpreter' ;;
        130) __ush_message='interrupted' ;;
        137) __ush_message='process killed (SIGKILL)' ;;
        143) __ush_message='terminated (SIGTERM)' ;;
      esac
    fi
    printf '%s:%s: %s (exit %s)\n  %s\n' "$__ush_origin" "$__ush_line" "$__ush_message" "$1" "$__ush_source" >&2
    __ush_line=''
  fi
}
trap '__ush_report "$?"' 0

case "${1:-}" in
  -h|--help) printf '%s' 'test_install.ush

Usage:
  test_install.ush [--help]
  test_install.ush [--man [ITEM]]
  test_install.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
test_install.ush

SYNOPSIS
test_install.ush [--help]
test_install.ush [--man [ITEM]]
test_install.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

__ush_at 2 'shell """'
set -eu

# Clear CDPATH so the cd does not print a path; the trailing space
# after '=' is intentional and POSIX, hence the disable.
# shellcheck disable=SC1007
ROOT_DIR="$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)"
INSTALL_SH="$ROOT_DIR/install.sh"

need_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "test_install.sh: $1 is required" >&2
    exit 1
  fi
}

detect_target() {
  os="$(uname -s)"
  arch="$(uname -m)"

  case "$os" in
    Linux) os="unknown-linux-gnu" ;;
    Darwin) os="apple-darwin" ;;
    *) echo "test_install.sh: unsupported os: $os" >&2; exit 1 ;;
  esac

  case "$arch" in
    x86_64 | amd64) arch="x86_64" ;;
    arm64 | aarch64) arch="aarch64" ;;
    *) echo "test_install.sh: unsupported arch: $arch" >&2; exit 1 ;;
  esac

  printf '%s-%s\n' "$arch" "$os"
}

make_archive() {
  tmpdir="$1"
  target="$2"
  pkg="$tmpdir/ush-$target"

  mkdir -p "$pkg"
  printf '#!/bin/sh\necho ush-test\n' > "$pkg/ush"
  printf '#!/bin/sh\necho ush-lsp-test\n' > "$pkg/ush_lsp"
  chmod +x "$pkg/ush" "$pkg/ush_lsp"
  tar -czf "$tmpdir/ush.tar.gz" -C "$tmpdir" "ush-$target"
}

compute_sha256() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
    return
  fi

  shasum -a 256 "$1" | awk '{print $1}'
}

write_checksums() {
  tmpdir="$1"
  target="$2"
  archive="$tmpdir/ush.tar.gz"
  checksum="$(compute_sha256 "$archive")"
  printf '%s  %s\n' "$checksum" "ush-$target.tar.gz" > "$tmpdir/sha256sums.txt"
}

assert_contains() {
  haystack="$1"
  needle="$2"

  case "$haystack" in
    *"$needle"*) ;;
    *)
      echo "test_install.sh: expected output to contain: $needle" >&2
      printf '%s\n' "$haystack" >&2
      exit 1
      ;;
  esac
}

test_existing_path_dir() {
  tmpdir="$(mktemp -d)"

  target="$(detect_target)"
  make_archive "$tmpdir" "$target"
  write_checksums "$tmpdir" "$target"

  home_dir="$tmpdir/home"
  mkdir -p "$home_dir/bin"
  output="$(
    HOME="$home_dir" \
    PATH="$home_dir/bin:/usr/bin:/bin" \
    SHELL=/bin/zsh \
    USH_DOWNLOAD_URL="file://$tmpdir/ush.tar.gz" \
    USH_CHECKSUM_URL="file://$tmpdir/sha256sums.txt" \
    sh "$INSTALL_SH" 2>&1
  )"

  [ -x "$home_dir/bin/ush" ]
  [ -x "$home_dir/bin/ush_lsp" ]
  [ ! -e "$home_dir/.zshrc" ]
  assert_contains "$output" "ush is already on PATH"
  rm -rf "$tmpdir"
}

test_auto_path_update() {
  tmpdir="$(mktemp -d)"

  target="$(detect_target)"
  make_archive "$tmpdir" "$target"
  write_checksums "$tmpdir" "$target"

  home_dir="$tmpdir/home"
  mkdir -p "$home_dir"
  output="$(
    HOME="$home_dir" \
    PATH="/usr/bin:/bin" \
    SHELL=/bin/zsh \
    USH_DOWNLOAD_URL="file://$tmpdir/ush.tar.gz" \
    USH_CHECKSUM_URL="file://$tmpdir/sha256sums.txt" \
    sh "$INSTALL_SH" 2>&1
  )"

  [ -x "$home_dir/.local/bin/ush" ]
  [ -x "$home_dir/.local/bin/ush_lsp" ]
  grep -F 'export PATH="$HOME/.local/bin:$PATH"' "$home_dir/.zshrc" >/dev/null
  assert_contains "$output" "added $home_dir/.local/bin to PATH"
  rm -rf "$tmpdir"
}

test_auto_path_disabled() {
  tmpdir="$(mktemp -d)"

  target="$(detect_target)"
  make_archive "$tmpdir" "$target"
  write_checksums "$tmpdir" "$target"

  home_dir="$tmpdir/home"
  mkdir -p "$home_dir"
  output="$(
    HOME="$home_dir" \
    PATH="/usr/bin:/bin" \
    SHELL=/bin/zsh \
    USH_AUTO_PATH=0 \
    USH_DOWNLOAD_URL="file://$tmpdir/ush.tar.gz" \
    USH_CHECKSUM_URL="file://$tmpdir/sha256sums.txt" \
    sh "$INSTALL_SH" 2>&1
  )"

  [ -x "$home_dir/.local/bin/ush" ]
  [ ! -e "$home_dir/.zshrc" ]
  assert_contains "$output" 'export PATH="'
  rm -rf "$tmpdir"
}

test_cli_flags() {
  tmpdir="$(mktemp -d)"

  target="$(detect_target)"
  make_archive "$tmpdir" "$target"
  write_checksums "$tmpdir" "$target"

  home_dir="$tmpdir/home"
  bin_dir="$home_dir/tools/bin"
  mkdir -p "$home_dir"
  output="$(
    HOME="$home_dir" \
    PATH="/usr/bin:/bin" \
    SHELL=/bin/zsh \
    sh "$INSTALL_SH" \
      --bin-dir "$bin_dir" \
      --no-modify-path \
      --download-url "file://$tmpdir/ush.tar.gz" \
      --checksum-url "file://$tmpdir/sha256sums.txt" 2>&1
  )"

  [ -x "$bin_dir/ush" ]
  [ -x "$bin_dir/ush_lsp" ]
  [ ! -e "$home_dir/.zshrc" ]
  assert_contains "$output" "ush installed to $bin_dir/ush"
  rm -rf "$tmpdir"
}

test_checksum_lookup_requires_exact_asset_name() {
  tmpdir="$(mktemp -d)"

  target="$(detect_target)"
  make_archive "$tmpdir" "$target"
  archive="$tmpdir/ush.tar.gz"
  checksum="$(compute_sha256 "$archive")"
  printf '%s  %s\n' "$checksum" "ush-$target.tarXgz" > "$tmpdir/sha256sums.txt"

  home_dir="$tmpdir/home"
  output_file="$(mktemp "${TMPDIR:-/tmp}/ush_install_exact_checksum.XXXXXX")"
  mkdir -p "$home_dir"
  if HOME="$home_dir" \
    PATH="/usr/bin:/bin" \
    SHELL=/bin/zsh \
    USH_DOWNLOAD_URL="file://$archive" \
    USH_CHECKSUM_URL="file://$tmpdir/sha256sums.txt" \
    sh "$INSTALL_SH" >"$output_file" 2>&1
  then
    echo "test_install.sh: installer accepted a near-miss checksum filename" >&2
    cat "$output_file" >&2
    rm -rf "$tmpdir" "$output_file"
    exit 1
  fi

  rm -rf "$tmpdir" "$output_file"
}

main() {
  need_cmd mktemp
  need_cmd tar
  need_cmd install
  test_existing_path_dir
  test_auto_path_update
  test_auto_path_disabled
  test_cli_flags
  test_checksum_lookup_requires_exact_asset_name
}

main "$@"
