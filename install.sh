#!/bin/sh
# Generated from 'install.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: install.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'install.ush' --checked-in -o 'install.sh'
set -eu

__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) if [ -f "$__ush_entrypoint" ]; then __ush_entrypoint=./$__ush_entrypoint; else __ush_entrypoint=$(command -v "$__ush_entrypoint"); fi ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'install.ush'
__ush_origin='install.ush'

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

__ush_print_help() {
  cat <<'__USH_DOC__'
install.ush - Install ush and ush_lsp from a checksum-verified release archive.

Usage:
  install.ush [--help]
  install.ush [--man [ITEM]]
  install.ush [--complete [PREFIX]]

Description:
  Options: --version VERSION, --bin-dir DIR, --prefix DIR,
  --download-url URL, --checksum-url URL, --no-modify-path.
  Run sh install.sh --version v0.12.0 to install a specific release.
__USH_DOC__
}

__ush_print_man() {
  case "${1:-}" in
    '')
      cat <<'__USH_MAIN__'
NAME
install.ush - Install ush and ush_lsp from a checksum-verified release archive.

SYNOPSIS
install.ush [--help]
install.ush [--man [ITEM]]
install.ush [--complete [PREFIX]]

DESCRIPTION
Install ush and ush_lsp from a checksum-verified release archive.
Options: --version VERSION, --bin-dir DIR, --prefix DIR,
--download-url URL, --checksum-url URL, --no-modify-path.
Run sh install.sh --version v0.12.0 to install a specific release.
__USH_MAIN__
      ;;
    *)
      printf '%s\n' "No documented item named: ${1}" >&2
      return 1
      ;;
  esac
}

__ush_complete() {
  __ush_prefix="${1:-}"
  while IFS= read -r __ush_item; do
    case "$__ush_item" in
      "$__ush_prefix"*) printf '%s\n' "$__ush_item" ;;
    esac
  done <<'__USH_COMPLETE__'
--complete
--help
--man
-h
complete
man
__USH_COMPLETE__
}

case "${1:-}" in
  -h|--help)
    __ush_print_help
    exit 0
    ;;
  man|--man)
    shift
    __ush_print_man "${1:-}"
    exit $?
    ;;
  complete|--complete)
    shift
    __ush_complete "${1:-}"
    exit 0
    ;;
esac

__ush_at 6 'shell """'
set -eu

# Defend against world-readable files created during the install, in
# case $TMPDIR is on a shared filesystem.
umask 077

# pipefail is not POSIX, but every shell we realistically run under
# (bash, dash, busybox sh, zsh) supports it as an extension. Enable it
# when it is available so that 'fetch | sha256_check' style pipelines
# fail loudly on partial downloads instead of silently returning 0.
# shellcheck disable=SC3040
(set -o pipefail 2>/dev/null) && set -o pipefail || true

REPO_INPUT="${USH_REPO:-${UBSH_REPO:-ubugeeei/ush}}"
VERSION="${USH_VERSION:-${UBSH_VERSION:-latest}}"
PREFIX="${USH_PREFIX:-${UBSH_PREFIX:-$HOME/.local}}"
BIN_DIR_INPUT="${USH_BIN_DIR:-${UBSH_BIN_DIR:-}}"
DOWNLOAD_URL="${USH_DOWNLOAD_URL:-${UBSH_DOWNLOAD_URL:-}}"
CHECKSUM_URL="${USH_CHECKSUM_URL:-${UBSH_CHECKSUM_URL:-}}"
AUTO_PATH="${USH_AUTO_PATH:-${UBSH_AUTO_PATH:-1}}"

# Use a dedicated variable so we never clobber the caller's $TMPDIR
# (which is consulted by mktemp, etc.). The directory is created with
# the restrictive umask above; mktemp -d's own mode is 0700 on every
# modern coreutils.
INSTALL_WORK_DIR="$(mktemp -d 2>/dev/null || mktemp -d -t ush.XXXXXX)"

cleanup() {
  rm -rf "$INSTALL_WORK_DIR"
}

trap cleanup EXIT INT TERM

usage() {
  cat <<'EOF'
usage: install.sh [--version VERSION] [--bin-dir DIR] [--prefix DIR] [--download-url URL] [--checksum-url URL] [--no-modify-path]

options:
  --version VERSION     install a specific release tag such as v0.6.0
  --bin-dir DIR         install ush into DIR
  --prefix DIR          install ush into DIR/bin
  --download-url URL    override the release archive URL
  --checksum-url URL    override the sha256sums.txt URL
  --no-modify-path      do not edit shell rc files
  -h, --help            show this help
EOF
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --version)
      [ "$#" -ge 2 ] || { echo "install.sh: --version requires a value" >&2; exit 1; }
      VERSION="$2"
      shift 2
      ;;
    --bin-dir)
      [ "$#" -ge 2 ] || { echo "install.sh: --bin-dir requires a value" >&2; exit 1; }
      BIN_DIR_INPUT="$2"
      shift 2
      ;;
    --prefix)
      [ "$#" -ge 2 ] || { echo "install.sh: --prefix requires a value" >&2; exit 1; }
      PREFIX="$2"
      shift 2
      ;;
    --download-url)
      [ "$#" -ge 2 ] || { echo "install.sh: --download-url requires a value" >&2; exit 1; }
      DOWNLOAD_URL="$2"
      shift 2
      ;;
    --checksum-url)
      [ "$#" -ge 2 ] || { echo "install.sh: --checksum-url requires a value" >&2; exit 1; }
      CHECKSUM_URL="$2"
      shift 2
      ;;
    --no-modify-path)
      AUTO_PATH=0
      shift
      ;;
    -h | --help)
      usage
      exit 0
      ;;
    *)
      echo "install.sh: unknown option: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

need_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "install.sh: $1 is required" >&2
    exit 1
  fi
}

normalize_repo() {
  printf '%s' "$1" | sed \
    -e 's#^https\{0,1\}://github\.com/##' \
    -e 's#^git@github\.com:##' \
    -e 's#\.git$##'
}

path_contains() {
  case ":$PATH:" in
    *":$1:"*) return 0 ;;
    *) return 1 ;;
  esac
}

pick_bin_dir() {
  if [ -n "$BIN_DIR_INPUT" ]; then
    printf '%s\n' "$BIN_DIR_INPUT"
    return
  fi

  if [ "${USH_PREFIX+x}" = "x" ] || [ "${UBSH_PREFIX+x}" = "x" ]; then
    printf '%s/bin\n' "$PREFIX"
    return
  fi

  for candidate in "$HOME/.local/bin" "$HOME/bin"; do
    if path_contains "$candidate"; then
      printf '%s\n' "$candidate"
      return
    fi
  done

  old_ifs="$IFS"
  IFS=":"
  for candidate in $PATH; do
    [ -n "$candidate" ] || continue
    case "$candidate" in
      "$HOME"/*)
        if [ -d "$candidate" ] && [ -w "$candidate" ]; then
          printf '%s\n' "$candidate"
          IFS="$old_ifs"
          return
        fi
        ;;
    esac
  done
  IFS="$old_ifs"

  printf '%s\n' "$HOME/.local/bin"
}

detect_profile() {
  shell_name="${SHELL##*/}"

  case "$shell_name" in
    zsh) printf '%s\n' "$HOME/.zshrc" ;;
    bash)
      if [ -f "$HOME/.bashrc" ] || [ ! -f "$HOME/.bash_profile" ]; then
        printf '%s\n' "$HOME/.bashrc"
      else
        printf '%s\n' "$HOME/.bash_profile"
      fi
      ;;
    ksh) printf '%s\n' "$HOME/.kshrc" ;;
    fish) printf '\n' ;;
    *) printf '%s\n' "$HOME/.profile" ;;
  esac
}

ensure_path() {
  bin_dir="$1"

  if path_contains "$bin_dir"; then
    printf 'ready\n'
    return
  fi

  if [ "$AUTO_PATH" = "0" ]; then
    printf 'manual\n'
    return
  fi

  profile="$(detect_profile)"
  if [ -z "$profile" ]; then
    printf 'manual\n'
    return
  fi

  display_dir="$bin_dir"
  case "$bin_dir" in
    "$HOME"/*) display_dir="\$HOME/${bin_dir#"$HOME"/}" ;;
  esac

  line="export PATH=\"$display_dir:\$PATH\""
  if [ -f "$profile" ] && grep -F "$line" "$profile" >/dev/null 2>&1; then
    printf '%s\n' "$profile"
    return
  fi

  touch "$profile" || return "$?"
  {
    printf '\n# Added by ush install.sh\n'
    printf '%s\n' "$line"
  } >> "$profile" || return "$?"
  printf '%s\n' "$profile"
}

detect_target() {
  os="$(uname -s)"
  arch="$(uname -m)"

  case "$os" in
    Linux) os="unknown-linux-gnu" ;;
    Darwin) os="apple-darwin" ;;
    *)
      echo "install.sh: unsupported operating system: $os" >&2
      echo "install.sh: build from source or use the tynix/Nix package on this platform" >&2
      exit 1
      ;;
  esac

  case "$arch" in
    x86_64 | amd64) arch="x86_64" ;;
    arm64 | aarch64) arch="aarch64" ;;
    *)
      echo "install.sh: unsupported architecture: $arch" >&2
      echo "install.sh: build from source on this platform" >&2
      exit 1
      ;;
  esac

  printf '%s-%s\n' "$arch" "$os"
}

fetch() {
  url="$1"
  output="$2"

  # When the URL is local (the release smoke test in CI uses file://),
  # network-hardening flags do not apply, so we do not pass them.
  case "$url" in
    file://*) is_local=1 ;;
    *) is_local=0 ;;
  esac

  if command -v curl >/dev/null 2>&1; then
    if [ "$is_local" = "1" ]; then
      curl -fsSL "$url" -o "$output"
    else
      # --proto '=https' rejects any redirect to a non-HTTPS scheme.
      # --tlsv1.2 refuses to negotiate older TLS versions.
      # --retry adds resilience against transient network failures
      # without weakening trust.
      curl -fsSL \
        --proto '=https' \
        --tlsv1.2 \
        --retry 3 \
        --retry-delay 2 \
        "$url" -o "$output"
    fi
    return
  fi

  if command -v wget >/dev/null 2>&1; then
    if [ "$is_local" = "1" ]; then
      wget -qO "$output" "$url"
    else
      wget -q \
        --https-only \
        --secure-protocol=TLSv1_2 \
        --tries=3 \
        -O "$output" \
        "$url"
    fi
    return
  fi

  echo "install.sh: curl or wget is required" >&2
  exit 1
}

build_download_url() {
  repo="$1"
  version="$2"
  target="$3"
  asset="ush-$target.tar.gz"

  if [ -n "$DOWNLOAD_URL" ]; then
    printf '%s\n' "$DOWNLOAD_URL"
    return
  fi

  if [ "$version" = "latest" ]; then
    printf 'https://github.com/%s/releases/latest/download/%s\n' "$repo" "$asset"
    return
  fi

  printf 'https://github.com/%s/releases/download/%s/%s\n' "$repo" "$version" "$asset"
}

build_checksum_url() {
  repo="$1"
  version="$2"

  if [ -n "$CHECKSUM_URL" ]; then
    printf '%s\n' "$CHECKSUM_URL"
    return
  fi

  if [ "$version" = "latest" ]; then
    printf 'https://github.com/%s/releases/latest/download/sha256sums.txt\n' "$repo"
    return
  fi

  printf 'https://github.com/%s/releases/download/%s/sha256sums.txt\n' "$repo" "$version"
}

sha256_tool() {
  if command -v sha256sum >/dev/null 2>&1; then
    printf 'sha256sum\n'
    return
  fi

  if command -v shasum >/dev/null 2>&1; then
    printf 'shasum -a 256\n'
    return
  fi

  if command -v openssl >/dev/null 2>&1; then
    printf 'openssl\n'
    return
  fi

  if command -v python3 >/dev/null 2>&1; then
    printf 'python3\n'
    return
  fi

  printf '\n'
}

compute_sha256() {
  tool="$(sha256_tool)"
  [ -n "$tool" ] || return 1

  case "$tool" in
    sha256sum)
      sha256sum "$1" | awk '{print $1}'
      ;;
    "shasum -a 256")
      shasum -a 256 "$1" | awk '{print $1}'
      ;;
    openssl)
      openssl dgst -sha256 "$1" | awk '{print $NF}'
      ;;
    python3)
      python3 - "$1" <<'PY'
import hashlib
import sys

digest = hashlib.sha256()
with open(sys.argv[1], "rb") as handle:
    for chunk in iter(lambda: handle.read(1024 * 1024), b""):
        digest.update(chunk)
print(digest.hexdigest())
PY
      ;;
  esac
}

verify_archive() {
  repo="$1"
  version="$2"
  asset="$3"
  archive="$4"

  tool="$(sha256_tool)"
  if [ -z "$tool" ]; then
    echo "install.sh: no supported sha256 tool found (sha256sum, shasum, openssl, python3); refusing to install without checksum verification" >&2
    exit 1
  fi

  sums_url="$(build_checksum_url "$repo" "$version")"
  sums_file="$INSTALL_WORK_DIR/sha256sums.txt"

  echo "install.sh: downloading $sums_url" >&2
  fetch "$sums_url" "$sums_file"

  line="$(awk -v asset="$asset" '
    {
      name = $2
      sub(/^\*/, "", name)
      if (name == asset) {
        print
        exit
      }
    }
  ' "$sums_file")"
  if [ -z "$line" ]; then
    echo "install.sh: could not find checksum for $asset" >&2
    exit 1
  fi

  set -- $line
  expected="$1"
  actual="$(compute_sha256 "$archive")"

  if [ "$actual" != "$expected" ]; then
    echo "install.sh: checksum verification failed for $asset" >&2
    exit 1
  fi

  echo "install.sh: verified checksum for $asset" >&2
}

if [ "${USH_BRANCH+x}" = "x" ] || [ "${UBSH_BRANCH+x}" = "x" ]; then
  echo "install.sh: USH_BRANCH/UBSH_BRANCH is no longer supported" >&2
  echo "install.sh: use USH_VERSION or build from source for branch installs" >&2
  exit 1
fi

need_cmd tar
need_cmd install

__ush_at 432 'let repo = $(normalize_repo "$REPO_INPUT")?'
__ush_stdout_0="$(
normalize_repo "$REPO_INPUT"
)" || { __ush_status=$?; __ush_report "$__ush_status"; exit "$__ush_status"; }
repo="${__ush_stdout_0}"
__ush_at 433 'let target = $(detect_target)?'
__ush_stdout_1="$(
detect_target
)" || { __ush_status=$?; __ush_report "$__ush_status"; exit "$__ush_status"; }
target="${__ush_stdout_1}"
__ush_at 434 'let bin_dir = $(pick_bin_dir)?'
__ush_stdout_2="$(
pick_bin_dir
)" || { __ush_status=$?; __ush_report "$__ush_status"; exit "$__ush_status"; }
bin_dir="${__ush_stdout_2}"
__ush_at 435 'let asset = "ush-" + target + ".tar.gz"'
asset="$(printf '%s' 'ush-' "${target}" '.tar.gz')"
__ush_at 436 'let url = $(build_download_url "$repo" "$VERSION" "$target")?'
__ush_stdout_3="$(
build_download_url "$repo" "$VERSION" "$target"
)" || { __ush_status=$?; __ush_report "$__ush_status"; exit "$__ush_status"; }
url="${__ush_stdout_3}"
__ush_at 437 'let work_dir = $(printf '"'"'%s'"'"' "$INSTALL_WORK_DIR")?'
__ush_stdout_4="$(
printf '%s' "$INSTALL_WORK_DIR"
)" || { __ush_status=$?; __ush_report "$__ush_status"; exit "$__ush_status"; }
work_dir="${__ush_stdout_4}"
__ush_at 438 'let archive = work_dir + "/ush.tar.gz"'
archive="$(printf '%s' "${work_dir}" '/ush.tar.gz')"
__ush_at 439 'let unpack_dir = work_dir + "/unpack"'
unpack_dir="$(printf '%s' "${work_dir}" '/unpack')"
__ush_at 441 'shell '"'"'echo "install.sh: downloading $url" >&2'"'"''
echo "install.sh: downloading $url" >&2
__ush_at 442 'shell '"'"'fetch "$url" "$archive"'"'"''
fetch "$url" "$archive"
__ush_at 443 'shell '"'"'verify_archive "$repo" "$VERSION" "$asset" "$archive"'"'"''
verify_archive "$repo" "$VERSION" "$asset" "$archive"
__ush_at 444 'shell '"'"'mkdir -p "$unpack_dir"'"'"''
mkdir -p "$unpack_dir"
__ush_at 445 'shell '"'"'tar -xzf "$archive" -C "$unpack_dir"'"'"''
tar -xzf "$archive" -C "$unpack_dir"
__ush_at 446 'let package_dir = $('
__ush_stdout_5="$(

  PACKAGE_DIR="$unpack_dir/ush-$target"
  if [ ! -f "$PACKAGE_DIR/ush" ] && [ -f "$unpack_dir/ush" ]; then
    PACKAGE_DIR="$unpack_dir"
  fi
  if [ ! -f "$PACKAGE_DIR/ush" ]; then
    echo "install.sh: release archive did not contain ush" >&2
    exit 1
  fi
  printf '%s' "$PACKAGE_DIR"

)" || { __ush_status=$?; __ush_report "$__ush_status"; exit "$__ush_status"; }
package_dir="${__ush_stdout_5}"
__ush_at 457 'shell '"'"'mkdir -p "$bin_dir"'"'"''
mkdir -p "$bin_dir"
__ush_at 458 'shell '"'"'install -m 755 "$package_dir/ush" "$bin_dir/ush"'"'"''
install -m 755 "$package_dir/ush" "$bin_dir/ush"
__ush_at 459 'shell """'
if [ -f "$package_dir/ush_lsp" ]; then
  install -m 755 "$package_dir/ush_lsp" "$bin_dir/ush_lsp"
fi
__ush_at 464 'let path_status = $(ensure_path "$bin_dir")?'
__ush_stdout_6="$(
ensure_path "$bin_dir"
)" || { __ush_status=$?; __ush_report "$__ush_status"; exit "$__ush_status"; }
path_status="${__ush_stdout_6}"
__ush_at 465 'print "ush installed to " + bin_dir + "/ush"'
printf '%s\n' "$(printf '%s' 'ush installed to ' "${bin_dir}" '/ush')"
__ush_at 466 'shell """'
if [ -f "$package_dir/ush_lsp" ]; then
  echo "ush_lsp installed to $bin_dir/ush_lsp"
fi

case "$path_status" in
  ready)
    echo "ush is already on PATH. Try: ush --version"
    ;;
  manual)
    echo "add this to your shell config, then restart your shell:"
    echo "export PATH=\"$bin_dir:\$PATH\""
    ;;
  *)
    echo "added $bin_dir to PATH in $path_status"
    echo "restart your shell or run: . \"$path_status\""
    ;;
esac

if command -v "$bin_dir/ush" >/dev/null 2>&1; then
  echo "quick check: $bin_dir/ush --version"
fi
