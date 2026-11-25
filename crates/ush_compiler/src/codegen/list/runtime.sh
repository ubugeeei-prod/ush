__ush_std_list_contains() {
  eval "__ush_list_len=\${${1}__len}"
  __ush_list_index=0
  while [ "$__ush_list_index" -lt "$__ush_list_len" ]; do
    eval "__ush_list_item=\${${1}__${__ush_list_index}}"
    if [ "$__ush_list_item" = "$2" ]; then return 0; fi
    __ush_list_index=$((__ush_list_index + 1))
  done
  return 1
}

__ush_std_list_join() {
  eval "__ush_list_len=\${${1}__len}"
  __ush_std_list_joined=''
  __ush_list_index=0
  while [ "$__ush_list_index" -lt "$__ush_list_len" ]; do
    eval "__ush_list_item=\${${1}__${__ush_list_index}}"
    if [ "$__ush_list_index" -gt 0 ]; then __ush_std_list_joined=$__ush_std_list_joined$2; fi
    __ush_std_list_joined=$__ush_std_list_joined$__ush_list_item
    __ush_list_index=$((__ush_list_index + 1))
  done
}
