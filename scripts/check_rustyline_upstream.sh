#!/bin/sh
# Generated from 'check_rustyline_upstream.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: check_rustyline_upstream.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'check_rustyline_upstream.ush' --checked-in -o 'check_rustyline_upstream.sh'
set -eu

__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) __ush_entrypoint=$(command -v "$__ush_entrypoint") ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'check_rustyline_upstream.ush'
__ush_origin='check_rustyline_upstream.ush'

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
  -h|--help) printf '%s' 'check_rustyline_upstream.ush

Usage:
  check_rustyline_upstream.ush [--help]
  check_rustyline_upstream.ush [--man [ITEM]]
  check_rustyline_upstream.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
check_rustyline_upstream.ush

SYNOPSIS
check_rustyline_upstream.ush [--help]
check_rustyline_upstream.ush [--man [ITEM]]
check_rustyline_upstream.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

__ush_at 2 'shell """'
set -eu

# Compare the pinned vendor/rustyline tag against the latest stable
# release published on crates.io. Fails when the vendored copy is
# behind — this is the signal to review upstream for advisories and
# either rebase the vendored copy or document why we are staying.

UPSTREAM_FILE="vendor/rustyline/UPSTREAM"

if [ ! -f "$UPSTREAM_FILE" ]; then
  echo "check_rustyline_upstream: missing $UPSTREAM_FILE" >&2
  exit 1
fi

pinned_tag="$(awk -F': *' '/^tag:/ { print $2; exit }' "$UPSTREAM_FILE")"
if [ -z "$pinned_tag" ]; then
  echo "check_rustyline_upstream: could not read 'tag:' from $UPSTREAM_FILE" >&2
  exit 1
fi
pinned_version="${pinned_tag#v}"

vendored_version="$(awk -F'"' '/^version =/ { print $2; exit }' vendor/rustyline/Cargo.toml)"
if [ -z "$vendored_version" ]; then
  echo "check_rustyline_upstream: could not read version from vendor/rustyline/Cargo.toml" >&2
  exit 1
fi

if [ "$pinned_version" != "$vendored_version" ]; then
  cat >&2 <<MSG
check_rustyline_upstream: tag drift inside the vendor tree.
  UPSTREAM tag: $pinned_tag (=$pinned_version)
  Cargo.toml:   $vendored_version
Update one or the other so they agree.
MSG
  exit 1
fi

# The Cargo sparse index exposes published versions and yanked status.
# Check the maintained major separately so a reviewed major never hides a patch.
versions="$(curl -fsSL --retry 3 https://index.crates.io/ru/st/rustyline | python3 -c '
import json, re, sys
stable = []
for line in sys.stdin:
    item = json.loads(line)
    if not item["yanked"] and re.fullmatch(r"[0-9]+\.[0-9]+\.[0-9]+", item["vers"]):
        stable.append(tuple(map(int, item["vers"].split("."))))
major = int(sys.argv[1].split(".")[0])
for version in [max(stable), max(v for v in stable if v[0] == major)]:
    print(".".join(map(str, version)))
' "$pinned_version")"
latest_version=$(printf '%s\n' "$versions" | head -n 1)
latest_patch=$(printf '%s\n' "$versions" | tail -n 1)
reviewed=$(awk -F': *' '/^reviewed:/ { print $2; exit }' "$UPSTREAM_FILE")

echo "pinned: $pinned_version, latest in major: $latest_patch, latest overall: $latest_version"
if [ "$pinned_version" != "$latest_patch" ] ||
   { [ "$pinned_version" != "$latest_version" ] && [ "$reviewed" != "v$latest_version" ]; }; then
  cat >&2 <<MSG
check_rustyline_upstream: vendored rustyline is behind upstream.
  Pinned: $pinned_version
  Latest: $latest_version

Review https://github.com/kkawakam/rustyline/releases/tag/v$latest_version
for security advisories or behaviour changes, then either:
  * rebase the vendored copy onto the new tag and update
    vendor/rustyline/UPSTREAM, or
  * document an explicit reason for staying on the old tag.
MSG
  exit 1
fi

echo "Vendored rustyline is current in its major and upstream releases have been reviewed."
