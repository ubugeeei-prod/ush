# Send signal $2 to task $1, whose status file is $3, and every process it
# started. A task this scope has already reaped is left alone: its ID may
# belong to another process by now. A killed task's status does not fail
# the scope when it is joined; awaiting it still reports the signal.
__ush_task_kill() {
  case $2 in
    HUP|INT|QUIT|KILL|USR1|USR2|ALRM|TERM|STOP|CONT) ;;
    *) __ush_report 2 "unknown signal: $2"; return 2 ;;
  esac
  case " $__ush_jobs " in *" $1 "*) ;; *) return 0 ;; esac
  # A task that has not started yet still holds the traps of the shell it
  # was forked from; wait until it has dropped them.
  while [ ! -e "$3.started" ] && kill -0 "$1" 2>/dev/null; do
    sleep 0.01 2>/dev/null || sleep 1
  done
  __ush_killed="${__ush_killed:-} $1"
  # The task itself is this shell's child and is not stopped: some shells
  # would report the stop to a later wait as the task's status.
  __ush_kill_children=$(__ush_child_pids "$1")
  kill -s "$2" "$1" 2>/dev/null || :
  for __ush_kill_child in $__ush_kill_children; do
    __ush_kill_tree "$__ush_kill_child" "$2"
  done
}
# Each descendant is stopped before its children are listed, so none can
# start another one unseen, signalled while stopped, and continued once its
# children are signalled, so a stopped process receives the signal.
__ush_kill_tree() {
  case $2 in STOP|CONT) ;; *) kill -s STOP "$1" 2>/dev/null || return 0 ;; esac
  __ush_kill_children=$(__ush_child_pids "$1")
  kill -s "$2" "$1" 2>/dev/null || :
  for __ush_kill_child in $__ush_kill_children; do
    __ush_kill_tree "$__ush_kill_child" "$2"
  done
  [ "$2" = STOP ] || kill -s CONT "$1" 2>/dev/null || :
}
__ush_child_pids() {
  { ps -A -o pid= -o ppid= 2>/dev/null || ps -o pid= -o ppid=; } |
    awk -v parent="$1" '$2 == parent { print $1 }'
}
