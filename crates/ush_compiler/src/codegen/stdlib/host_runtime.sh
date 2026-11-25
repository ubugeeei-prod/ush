__ush_std_host() {
  __ush_host=${USH_RUNTIME:-ush}
  if ! command -v "$__ush_host" >/dev/null 2>&1; then
    printf '%s requires the ush runtime; install ush or set USH_RUNTIME to its executable path\n' "$1" >&2
    return 127
  fi
  "$__ush_host" __runtime v1 "$@"
}

ush_fn_std__runtime__available() {
  if "${USH_RUNTIME:-ush}" __runtime v1 ping >/dev/null 2>&1; then printf true; else printf false; fi
}

ush_fn_std__string__length() {
  __ush_std_host std::string::length "$@"
}

ush_fn_std__string__byte_length() {
  __ush_std_host std::string::byte_length "$@"
}

ush_fn_std__string__reverse() {
  __ush_std_host std::string::reverse "$@"
}

ush_fn_std__string__to_lower() {
  __ush_std_host std::string::to_lower "$@"
}

ush_fn_std__string__to_upper() {
  __ush_std_host std::string::to_upper "$@"
}

ush_fn_std__string__chars() {
  __ush_std_host std::string::chars "$@"
}

ush_fn_std__string__slice() {
  __ush_std_host std::string::slice "$@"
}

ush_fn_std__string__find() {
  __ush_std_host std::string::find "$@"
}

ush_fn_std__fs__read_link() {
  __ush_path=''
  __ush_std_fs_path "$1" || return "$?"
  shift
  __ush_std_host std::fs::read_link "$__ush_path" "$@"
}

ush_fn_std__fs__canonicalize() {
  __ush_path=''
  __ush_std_fs_path "$1" || return "$?"
  shift
  __ush_std_host std::fs::canonicalize "$__ush_path" "$@"
}

ush_fn_std__fs__mode() {
  __ush_path=''
  __ush_std_fs_path "$1" || return "$?"
  shift
  __ush_std_host std::fs::mode "$__ush_path" "$@"
}

ush_fn_std__fs__owner() {
  __ush_path=''
  __ush_std_fs_path "$1" || return "$?"
  shift
  __ush_std_host std::fs::owner "$__ush_path" "$@"
}

ush_fn_std__fs__group() {
  __ush_path=''
  __ush_std_fs_path "$1" || return "$?"
  shift
  __ush_std_host std::fs::group "$__ush_path" "$@"
}

ush_fn_std__fs__modified() {
  __ush_path=''
  __ush_std_fs_path "$1" || return "$?"
  shift
  __ush_std_host std::fs::modified "$__ush_path" "$@"
}

ush_fn_std__fs__accessed() {
  __ush_path=''
  __ush_std_fs_path "$1" || return "$?"
  shift
  __ush_std_host std::fs::accessed "$__ush_path" "$@"
}

ush_fn_std__fs__inode() {
  __ush_path=''
  __ush_std_fs_path "$1" || return "$?"
  shift
  __ush_std_host std::fs::inode "$__ush_path" "$@"
}

ush_fn_std__fs__links() {
  __ush_path=''
  __ush_std_fs_path "$1" || return "$?"
  shift
  __ush_std_host std::fs::links "$__ush_path" "$@"
}

ush_fn_std__fs__is_socket() {
  __ush_path=''
  __ush_std_fs_path "$1" || return "$?"
  shift
  __ush_std_host std::fs::is_socket "$__ush_path" "$@"
}

ush_fn_std__fs__read_bytes() {
  __ush_path=''
  __ush_std_fs_path "$1" || return "$?"
  shift
  __ush_std_host std::fs::read_bytes "$__ush_path" "$@"
}

ush_fn_std__fs__write_bytes() {
  __ush_path=''
  __ush_std_fs_path "$1" || return "$?"
  shift
  __ush_std_host std::fs::write_bytes "$__ush_path" "$@"
}

ush_fn_std__fs__write_atomic() {
  __ush_path=''
  __ush_std_fs_path "$1" || return "$?"
  shift
  __ush_std_host std::fs::write_atomic "$__ush_path" "$@"
}

ush_fn_std__bytes__from_hex() {
  __ush_std_host std::bytes::from_hex "$@"
}

ush_fn_std__bytes__from_text() {
  __ush_std_host std::bytes::from_text "$@"
}

ush_fn_std__bytes__decode_base64() {
  __ush_std_host std::bytes::decode_base64 "$@"
}

ush_fn_std__bytes__to_hex() {
  __ush_std_host std::bytes::to_hex "$@"
}

ush_fn_std__bytes__to_text() {
  __ush_std_host std::bytes::to_text "$@"
}

ush_fn_std__bytes__encode_base64() {
  __ush_std_host std::bytes::encode_base64 "$@"
}

ush_fn_std__time__unix_seconds() {
  __ush_std_host std::time::unix_seconds "$@"
}

ush_fn_std__time__unix_millis() {
  __ush_std_host std::time::unix_millis "$@"
}

ush_fn_std__time__sleep_millis() {
  __ush_std_host std::time::sleep_millis "$@"
}

ush_fn_std__time__format_utc() {
  __ush_std_host std::time::format_utc "$@"
}

ush_fn_std__random__bytes() {
  __ush_std_host std::random::bytes "$@"
}

ush_fn_std__random__hex() {
  __ush_std_host std::random::hex "$@"
}

ush_fn_std__random__int() {
  __ush_std_host std::random::int "$@"
}

ush_fn_std__net__resolve() {
  __ush_std_host std::net::resolve "$@"
}

ush_fn_std__os__euid() {
  __ush_std_host std::os::euid "$@"
}

ush_fn_std__os__egid() {
  __ush_std_host std::os::egid "$@"
}

ush_fn_std__os__page_size() {
  __ush_std_host std::os::page_size "$@"
}

ush_fn_std__os__cpu_count() {
  __ush_std_host std::os::cpu_count "$@"
}

ush_fn_std__os__error_message() {
  __ush_std_host std::os::error_message "$@"
}

ush_fn_std__int__add() {
  __ush_std_host std::int::add "$@"
}

ush_fn_std__int__sub() {
  __ush_std_host std::int::sub "$@"
}

ush_fn_std__int__mul() {
  __ush_std_host std::int::mul "$@"
}

ush_fn_std__int__div() {
  __ush_std_host std::int::div "$@"
}

ush_fn_std__int__rem() {
  __ush_std_host std::int::rem "$@"
}

ush_fn_std__int__pow() {
  __ush_std_host std::int::pow "$@"
}
