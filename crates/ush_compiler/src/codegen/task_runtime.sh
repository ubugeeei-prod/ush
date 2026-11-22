
__ush_jobs=''
__ush_task_seq=0
__ush_task_dir=$(
  umask 077
  CDPATH= cd -P "${TMPDIR:-/tmp}" || exit 1
  __ush_base=$(pwd -P)
  __ush_n=0
  while [ "$__ush_n" -lt 1000 ]; do
    __ush_dir=$__ush_base/ush-tasks.$$.$__ush_n
    if mkdir "$__ush_dir" 2>/dev/null; then printf '%s' "$__ush_dir"; exit 0; fi
    __ush_n=$((__ush_n + 1))
  done
  printf '%s\n' 'ush: cannot create private task directory' >&2
  exit 1
)
__ush_task_slot() {
  while :; do
    __ush_task_seq=$((__ush_task_seq + 1))
    __ush_slot=$__ush_task_dir/$__ush_task_seq
    if mkdir "$__ush_slot" 2>/dev/null; then break; fi
    [ -d "$__ush_slot" ] || return 1
  done
}
__ush_forget_job() {
  __ush_remaining=''
  for __ush_pid in $__ush_jobs; do
    if [ "$__ush_pid" != "$1" ]; then __ush_remaining="$__ush_remaining $__ush_pid"; fi
  done
  __ush_jobs=$__ush_remaining
}
__ush_join() {
  __ush_exit=$1
  __ush_report "$__ush_exit"
  for __ush_pid in $__ush_jobs; do
    wait "$__ush_pid" || { __ush_child=$?; [ "$__ush_exit" -ne 0 ] || __ush_exit=$__ush_child; }
  done
}
__ush_task_scope() {
  __ush_jobs=''
  trap '__ush_join "$?"; exit "$__ush_exit"' 0
}
__ush_finish() {
  __ush_join "$1"
  rm -rf "$__ush_task_dir"
  exit "$__ush_exit"
}
trap '__ush_finish "$?"' 0
trap 'exit 129' HUP
trap 'exit 130' INT
trap 'exit 143' TERM
