#!/bin/sh
# Generated from 'test_task_await.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: test_task_await.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'test_task_await.ush' --checked-in -o 'test_task_await.sh'
set -eu

__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) __ush_entrypoint=$(command -v "$__ush_entrypoint") ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'test_task_await.ush'
__ush_origin='test_task_await.ush'

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
  -h|--help) printf '%s' 'test_task_await.ush

Usage:
  test_task_await.ush [--help]
  test_task_await.ush [--man [ITEM]]
  test_task_await.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
test_task_await.ush

SYNOPSIS
test_task_await.ush [--help]
test_task_await.ush [--man [ITEM]]
test_task_await.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

__ush_at 2 'shell """'
set -eu

ush=${1:?compiler path required}
target=${2:-sh}
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' 0

compile() {
  "$ush" compile "$tmp/job.ush" --target "$target" --checked-in -o "$tmp/job.sh"
  chmod +x "$tmp/job.sh"
  rm "$tmp/job.ush"
}

cat > "$tmp/job.ush" <<'USH'
fn build() -> Int / unknown, stderr {
  fail(23, "missing input")
}
let job = async build()
print job.await
USH
compile
status=0
"$tmp/job.sh" >"$tmp/out" 2>"$tmp/err" || status=$?
[ "$status" -eq 23 ]
[ ! -s "$tmp/out" ]
cat > "$tmp/expected" <<'EXPECTED'
job.ush:2: missing input (exit 23)
  fail(23, "missing input")
job.ush:5: command failed (exit 23)
  print job.await
EXPECTED
diff -u "$tmp/expected" "$tmp/err"

cat > "$tmp/job.ush" <<'USH'
fn build() -> Int / unknown, stdin, stderr, fs, net, env, exec {
  let pause = $(sleep 0.05 2>/dev/null || sleep 1)?
  41
}
let job = async build()
let first = async {
  let value = job.await
  value + 1
}
let second = async {
  let value = job.await
  value + 2
}
print first.await
print second.await
print job.await
print job.await
USH
compile
"$tmp/job.sh" >"$tmp/out" 2>"$tmp/err"
printf '%s\n' 42 43 41 41 > "$tmp/expected"
diff -u "$tmp/expected" "$tmp/out"
[ ! -s "$tmp/err" ]

cat > "$tmp/job.ush" <<'USH'
fn build() -> Int / unknown, stderr {
  fail(23, "missing input")
}
let job = async build()
let sibling = async {
  let value = job.await
  value
}
print sibling.await
USH
compile
status=0
"$tmp/job.sh" >"$tmp/out" 2>"$tmp/err" || status=$?
[ "$status" -eq 23 ]
[ ! -s "$tmp/out" ]
cat > "$tmp/expected" <<'EXPECTED'
job.ush:2: missing input (exit 23)
  fail(23, "missing input")
job.ush:6: command failed (exit 23)
  let value = job.await
job.ush:9: command failed (exit 23)
  print sibling.await
EXPECTED
diff -u "$tmp/expected" "$tmp/err"
