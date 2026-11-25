#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/docs_bin.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/docs_bin.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/docs_bin.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/docs_bin.ush'

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
docs_bin.ush - Demo CLI.

Usage:
  docs_bin.ush [--help]
  docs_bin.ush [--man [ITEM]]
  docs_bin.ush [--complete [PREFIX]]
  docs_bin.ush --man greet

Description:

  A snapshot fixture for generated docs and wrapper code.

Notes:
  - Generated docs stay available through --help and --man.

Examples:
  docs_bin.ush --help

See also:
  - docs_bin.ush --help

Documented items:
  fn greet(name: String) -> String
      Greet a user.
  fn bin(#[alias("n")] name: String, #[default(2)] count: Int, verbose: Bool)
      Run the CLI.
__USH_DOC__
}

__ush_print_man() {
  case "${1:-}" in
    '')
      cat <<'__USH_MAIN__'
NAME
docs_bin.ush - Demo CLI.

SYNOPSIS
docs_bin.ush [--help]
docs_bin.ush [--man [ITEM]]
docs_bin.ush [--complete [PREFIX]]
docs_bin.ush --man greet

DESCRIPTION
Demo CLI.

A snapshot fixture for generated docs and wrapper code.

NOTES
Generated docs stay available through --help and --man.

EXAMPLES
docs_bin.ush --help

FUNCTIONS
greet
  fn greet(name: String) -> String
  Greet a user.
bin
  fn bin(#[alias("n")] name: String, #[default(2)] count: Int, verbose: Bool)
  Run the CLI.

SEE ALSO
docs_bin.ush --help
__USH_MAIN__
      ;;
    'greet')
      cat <<'__USH_GREET__'
NAME
docs_bin.ush greet

SIGNATURE
fn greet(name: String) -> String

DESCRIPTION
Greet a user.

PARAMETERS
name - target user

RETURNS
greeting text
__USH_GREET__
      ;;
    'bin')
      cat <<'__USH_BIN__'
NAME
docs_bin.ush bin

SIGNATURE
fn bin(#[alias("n")] name: String, #[default(2)] count: Int, verbose: Bool)

DESCRIPTION
Run the CLI.

PARAMETERS
name - target user
count - repeat count
__USH_BIN__
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
bin
complete
greet
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

exec 3>&1

ush_fn_greet() {
  __ush_fn_greet_arg_0="${1}"
  # shellcheck disable=SC2034
  name="${__ush_fn_greet_arg_0}"
  __ush_at 13 '"hi " + name'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' 'hi '"${__ush_fn_greet_arg_0}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' 'hi '"${__ush_fn_greet_arg_0}" >&4
  fi
  return 0
}

ush_fn_bin() {
  __ush_fn_bin_arg_0="${1}"
  # shellcheck disable=SC2034
  name="${__ush_fn_bin_arg_0}"
  __ush_fn_bin_arg_1="${2}"
  # shellcheck disable=SC2034
  count="${__ush_fn_bin_arg_1}"
  __ush_fn_bin_arg_2="${3}"
  # shellcheck disable=SC2034
  verbose="${__ush_fn_bin_arg_2}"
  __ush_at 20 'print $ greet name'
  __ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_greet "${__ush_fn_bin_arg_0}" 4>&1 >&3)" || return "$?"
  printf '%s\n' "${__ush_value_0}"
  __ush_at 21 'print count'
  printf '%s\n' "${__ush_fn_bin_arg_1}"
  __ush_at 22 'print verbose'
  printf '%s\n' "${__ush_fn_bin_arg_2}"
}
