__ush_std_encode_args() {
  for __ush_arg do
    printf x
    printf '%s' "$__ush_arg" | od -A n -v -t x1 | tr -d ' \n'
    printf ' '
  done
}

ush_fn_std__process__args() {
  # Keep the encoder linked even when args is the only process helper used.
  if [ "${__ush_argv+x}" != x ]; then __ush_std_encode_args; else printf '%s' "$__ush_argv"; fi
}

ush_fn_std__process__arg() {
  __ush_requested=$1
  __ush_args__len=0
  ush_fn_std__unpack __ush_args "$(ush_fn_std__process__args)"
  case $__ush_requested in ''|*[!0-9]*) return 1 ;; esac
  if [ "$__ush_requested" -lt "$__ush_args__len" ]; then
    eval "printf '%s' \"\${__ush_args__${__ush_requested}}\""
  fi
}

ush_fn_std__process__exit() {
  case $1 in ''|*[!0-9]*) return 1 ;; esac
  [ "$1" -le 255 ] || return 1
  exit "$1"
}

ush_fn_std__io__eprint() {
  printf '%s\n' "$1" >&2
}

ush_fn_std__fs__temp_dir() {
  mktemp -d "$__ush_task_dir/tmp.XXXXXXXXXX"
}

ush_fn_std__process__exec() {
  __ush_program=$1
  __ush_args__len=0
  __ush_arg=''
  ush_fn_std__unpack __ush_args "$2" || return "$?"
  set --
  __ush_index=0
  while [ "$__ush_index" -lt "$__ush_args__len" ]; do
    eval "__ush_arg=\${__ush_args__${__ush_index}}"
    set -- "$@" "$__ush_arg"
    __ush_index=$((__ush_index + 1))
  done
  "$__ush_program" "$@"
}

ush_fn_std__process__output() {
  ush_fn_std__process__exec "$@"
}

ush_fn_std__process__status() {
  if (ush_fn_std__process__exec "$@") >/dev/null 2>&1; then printf 0; else printf '%s' "$?"; fi
}

ush_fn_std__process__redirect() {
  if (ush_fn_std__process__exec "$1" "$2") > "$3" 2> "$4"; then printf 0; else printf '%s' "$?"; fi
}

ush_fn_std__string__split() {
  [ -n "$2" ] || { printf '%s\n' 'std::string::split: separator must not be empty' >&2; return 1; }
  __ush_rest=$1
  __ush_separator=$2
  while :; do
    case $__ush_rest in
      *"$__ush_separator"*)
        __ush_std_encode_args "${__ush_rest%%"$__ush_separator"*}"
        __ush_rest=${__ush_rest#*"$__ush_separator"} ;;
      *) __ush_std_encode_args "$__ush_rest"; break ;;
    esac
  done
}

ush_fn_std__string__lines() {
  [ -n "$1" ] || return 0
  __ush_newline='
'
  __ush_carriage=$(printf '\r')
  __ush_rest=${1%"$__ush_newline"}
  while :; do
    case $__ush_rest in
      *"$__ush_newline"*)
        __ush_line=${__ush_rest%%"$__ush_newline"*}
        __ush_std_encode_args "${__ush_line%"$__ush_carriage"}"
        __ush_rest=${__ush_rest#*"$__ush_newline"} ;;
      *) __ush_std_encode_args "${__ush_rest%"$__ush_carriage"}"; break ;;
    esac
  done
}

ush_fn_std__string__contains() {
  case $1 in *"$2"*) printf true ;; *) printf false ;; esac
}

ush_fn_std__string__trim() {
  __ush_trimmed=$1
  __ush_trimmed=${__ush_trimmed#"${__ush_trimmed%%[![:space:]]*}"}
  __ush_trimmed=${__ush_trimmed%"${__ush_trimmed##*[![:space:]]}"}
  printf '%s' "$__ush_trimmed"
}

ush_fn_std__string__to_int() {
  if __ush_integer=$(ush_fn_std____extern__int "$1" 2>/dev/null); then
    printf '%s' "$__ush_integer"
  else
    printf '%s\n' 'std::string::to_int: expected a signed 64-bit decimal integer' >&2
    return 1
  fi
}
