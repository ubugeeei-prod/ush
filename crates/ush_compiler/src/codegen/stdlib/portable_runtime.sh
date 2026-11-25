ush_fn_std__string__is_empty() {
  if [ -z "$1" ]; then printf true; else printf false; fi
}

ush_fn_std__string__is_ascii() {
  if [ "$(printf '%s' "$1" | LC_ALL=C tr -d '\001-\177')" = '' ]; then printf true; else printf false; fi
}

ush_fn_std__string__to_lower_ascii() {
  printf '%s' "$1" | LC_ALL=C tr '[:upper:]' '[:lower:]'
}

ush_fn_std__string__to_upper_ascii() {
  printf '%s' "$1" | LC_ALL=C tr '[:lower:]' '[:upper:]'
}

ush_fn_std__string__trim_start() {
  printf '%s' "${1#"${1%%[![:space:]]*}"}"
}

ush_fn_std__string__trim_end() {
  printf '%s' "${1%"${1##*[![:space:]]}"}"
}

ush_fn_std__string__shell_quote() {
  __ush_rest=$1
  printf "'"
  while :; do
    case $__ush_rest in
      *"'"*) printf '%s%s' "${__ush_rest%%"'"*}" "'\''"; __ush_rest=${__ush_rest#*"'"} ;;
      *) printf "%s'" "$__ush_rest"; return ;;
    esac
  done
}

ush_fn_std__string__words() {
  set -f
  IFS=' 	
'
  # shellcheck disable=SC2086
  __ush_std_encode_args $1
}

ush_fn_std__string__join() {
  __ush_join_separator=$2
  __ush_join_items__len=0
  __ush_join_item=''
  ush_fn_std__unpack __ush_join_items "$1" || return "$?"
  __ush_join_index=0
  while [ "$__ush_join_index" -lt "$__ush_join_items__len" ]; do
    [ "$__ush_join_index" -eq 0 ] || printf '%s' "$__ush_join_separator"
    eval "__ush_join_item=\${__ush_join_items__${__ush_join_index}}"
    printf '%s' "$__ush_join_item"
    __ush_join_index=$((__ush_join_index + 1))
  done
}

ush_fn_std__string__repeat() {
  [ "$2" -ge 0 ] || { printf '%s\n' 'std::string::repeat: count must be nonnegative' >&2; return 1; }
  __ush_repeat_count=$2
  __ush_repeat_chunk=$1
  while [ "$__ush_repeat_count" -gt 0 ]; do
    if [ "$((__ush_repeat_count % 2))" -eq 1 ]; then printf '%s' "$__ush_repeat_chunk"; fi
    __ush_repeat_count=$((__ush_repeat_count / 2))
    [ "$__ush_repeat_count" -eq 0 ] || __ush_repeat_chunk=$__ush_repeat_chunk$__ush_repeat_chunk
  done
}

ush_fn_std__string__count() {
  [ -n "$2" ] || { printf '%s\n' 'std::string::count: part must not be empty' >&2; return 1; }
  __ush_rest=$1
  __ush_count=0
  while :; do
    case $__ush_rest in
      *"$2"*) __ush_count=$((__ush_count + 1)); __ush_rest=${__ush_rest#*"$2"} ;;
      *) printf '%s' "$__ush_count"; return ;;
    esac
  done
}

__ush_std_fs_path() {
  __ush_path=$(ush_fn_std__path__resolve "$1" && printf .) || return "$?"
  __ush_path=${__ush_path%.}
  [ -n "$__ush_path" ] || { printf '%s\n' 'std::fs: path must not be empty' >&2; return 1; }
  case $__ush_path in .|..|/*|./*|../*) ;; *) __ush_path=./$__ush_path ;; esac
}

ush_fn_std__fs__is_symlink() {
  __ush_std_fs_path "$1" || return "$?"
  if [ -L "$__ush_path" ]; then printf true; else printf false; fi
}

ush_fn_std__fs__is_fifo() {
  __ush_std_fs_path "$1" || return "$?"
  if [ -p "$__ush_path" ]; then printf true; else printf false; fi
}

ush_fn_std__fs__is_readable() {
  __ush_std_fs_path "$1" || return "$?"
  if [ -r "$__ush_path" ]; then printf true; else printf false; fi
}

ush_fn_std__fs__is_writable() {
  __ush_std_fs_path "$1" || return "$?"
  if [ -w "$__ush_path" ]; then printf true; else printf false; fi
}

ush_fn_std__fs__is_executable() {
  __ush_std_fs_path "$1" || return "$?"
  if [ -x "$__ush_path" ]; then printf true; else printf false; fi
}

ush_fn_std__fs__mkdir() {
  __ush_std_fs_path "$1" || return "$?"
  mkdir "$__ush_path"
}

ush_fn_std__fs__rmdir() {
  __ush_std_fs_path "$1" || return "$?"
  rmdir "$__ush_path"
}

ush_fn_std__fs__touch() {
  __ush_std_fs_path "$1" || return "$?"
  touch "$__ush_path"
}

ush_fn_std__fs__remove_tree() {
  __ush_std_fs_path "$1" || return "$?"
  rm -rf "$__ush_path"
}

ush_fn_std__fs__read_dir() {
  __ush_std_fs_path "$1" || return "$?"
  __ush_dir=$__ush_path
  [ -d "$__ush_dir" ] && [ -r "$__ush_dir" ] && [ -x "$__ush_dir" ] || {
    printf 'std::fs::read_dir: cannot read directory: %s\n' "$__ush_dir" >&2
    return 1
  }
  for __ush_entry in "$__ush_dir"/* "$__ush_dir"/.[!.]* "$__ush_dir"/..?*; do
    if [ -e "$__ush_entry" ] || [ -L "$__ush_entry" ]; then
      __ush_std_encode_args "$__ush_entry"
    fi
  done
}

ush_fn_std__fs__read_lines() {
  __ush_std_fs_path "$1" || return "$?"
  ush_fn_std__io__read_lines < "$__ush_path"
}

ush_fn_std__fs__symlink() {
  __ush_std_fs_path "$2" || return "$?"
  __ush_link=$__ush_path
  __ush_target=$(ush_fn_std__path__resolve "$1" && printf .) || return "$?"
  __ush_target=${__ush_target%.}
  [ -n "$__ush_target" ] || { printf '%s\n' 'std::fs::symlink: target must not be empty' >&2; return 1; }
  case $__ush_target in -*) __ush_target=./$__ush_target ;; esac
  ln -s "$__ush_target" "$__ush_link"
}

ush_fn_std__fs__hard_link() {
  __ush_std_fs_path "$1" || return "$?"
  __ush_target=$__ush_path
  __ush_std_fs_path "$2" || return "$?"
  __ush_link=$__ush_path
  ln "$__ush_target" "$__ush_link"
}

ush_fn_std__fs__chmod() {
  case $2 in [0-7][0-7][0-7]|[0-7][0-7][0-7][0-7]) ;; *)
    printf '%s\n' 'std::fs::chmod: mode must contain three or four octal digits' >&2; return 1 ;;
  esac
  __ush_std_fs_path "$1" || return "$?"
  chmod "$2" "$__ush_path"
}

ush_fn_std__fs__size() {
  __ush_std_fs_path "$1" || return "$?"
  __ush_size=$(wc -c < "$__ush_path") || return "$?"
  __ush_size=${__ush_size#"${__ush_size%%[![:space:]]*}"}
  printf '%s' "$__ush_size"
}

ush_fn_std__path__is_absolute() {
  case $1 in /*) printf true ;; *) printf false ;; esac
}

ush_fn_std__path__extension() {
  __ush_base=${1##*/}
  case $__ush_base in .|..|.*) __ush_base=${__ush_base#?} ;; esac
  case $__ush_base in *.*) printf '%s' "${__ush_base##*.}" ;; esac
}

ush_fn_std__path__stem() {
  __ush_base=${1##*/}
  case $__ush_base in
    .|..) printf '%s' "$__ush_base"; return ;;
    .*) __ush_tail=${__ush_base#?}; case $__ush_tail in *.*) __ush_base=${__ush_base%.*} ;; esac ;;
    *.*) __ush_base=${__ush_base%.*} ;;
  esac
  printf '%s' "$__ush_base"
}

ush_fn_std__path__normalize() {
  __ush_normal_rest=$1
  __ush_normal_result=''
  __ush_normal_root=''
  case $1 in /*) __ush_normal_root=/ ;; esac
  # POSIX permits a distinct interpretation for exactly two leading slashes.
  case $1 in //|//[!/]*) __ush_normal_root=// ;; esac
  while [ -n "$__ush_normal_rest" ]; do
    __ush_normal_part=${__ush_normal_rest%%/*}
    case $__ush_normal_rest in */*) __ush_normal_rest=${__ush_normal_rest#*/} ;; *) __ush_normal_rest='' ;; esac
    case $__ush_normal_part in
      ''|.) ;;
      ..)
        case $__ush_normal_result in
          ''|..|*/..)
            if [ -z "$__ush_normal_root" ]; then __ush_normal_result=${__ush_normal_result:+$__ush_normal_result/}..; fi ;;
          */*) __ush_normal_result=${__ush_normal_result%/*} ;;
          *) __ush_normal_result='' ;;
        esac ;;
      *) __ush_normal_result=${__ush_normal_result:+$__ush_normal_result/}$__ush_normal_part ;;
    esac
  done
  if [ -z "$__ush_normal_result$__ush_normal_root" ]; then printf .; else printf '%s%s' "$__ush_normal_root" "$__ush_normal_result"; fi
}

ush_fn_std__io__print() {
  printf '%s' "$1"
}

ush_fn_std__io__println() {
  printf '%s\n' "$1"
}

ush_fn_std__io__read_lines() {
  __ush_line=''
  while IFS= read -r __ush_line || [ -n "$__ush_line" ]; do
    __ush_std_encode_args "$__ush_line"
  done
}

ush_fn_std__io__read_all() {
  cat
}

ush_fn_std__terminal__is_tty() {
  [ "$1" -ge 0 ] || { printf '%s\n' 'std::terminal::is_tty: file descriptor must be nonnegative' >&2; return 1; }
  if [ -t "$1" ]; then printf true; else printf false; fi
}

ush_fn_std__process__pid() {
  printf '%s' "$$"
}

ush_fn_std__process__sleep() {
  [ "$1" -ge 0 ] || { printf '%s\n' 'std::process::sleep: seconds must be nonnegative' >&2; return 1; }
  sleep "$1"
}

ush_fn_std__process__chdir() {
  __ush_std_fs_path "$1" || return "$?"
  CDPATH='' cd "$__ush_path" || return "$?"
}

ush_fn_std__os__name() {
  uname -s
}

ush_fn_std__os__arch() {
  uname -m
}

ush_fn_std__os__hostname() {
  uname -n
}

ush_fn_std__os__uid() {
  id -u
}

ush_fn_std__os__gid() {
  id -g
}

ush_fn_std__int__min() {
  if [ "$1" -lt "$2" ]; then printf '%s' "$1"; else printf '%s' "$2"; fi
}

ush_fn_std__int__max() {
  if [ "$1" -gt "$2" ]; then printf '%s' "$1"; else printf '%s' "$2"; fi
}

ush_fn_std__int__abs() {
  if [ "$1" = -9223372036854775808 ]; then printf '%s\n' 'std::int::abs: integer overflow' >&2; return 1; fi
  if [ "$1" -lt 0 ]; then printf '%s' "${1#-}"; else printf '%s' "$1"; fi
}

ush_fn_std__int__sign() {
  if [ "$1" -lt 0 ]; then printf '%s' -1; else
    if [ "$1" -gt 0 ]; then printf 1; else printf 0; fi
  fi
}

ush_fn_std__int__clamp() {
  [ "$2" -le "$3" ] || { printf '%s\n' 'std::int::clamp: min must not exceed max' >&2; return 1; }
  if [ "$1" -lt "$2" ]; then printf '%s' "$2"; else ush_fn_std__int__min "$1" "$3"; fi
}
