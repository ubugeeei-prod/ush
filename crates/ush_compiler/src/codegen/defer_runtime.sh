# Run the deferred blocks of the current scope, last registered first. A
# block that fails has reported its own line; the rest still run, and the
# first failure becomes the exit status if the scope was succeeding.
__ush_run_defers() {
  while [ -n "${__ush_defers:-}" ]; do
    __ush_defer=${__ush_defers%% *}
    __ush_defers=${__ush_defers#* }
    "$__ush_defer" || {
      __ush_status=$?
      [ "$__ush_exit" -ne 0 ] || __ush_exit=$__ush_status
    }
  done
}
# A function that defers runs in a process of its own; its exit, including
# one caused by a handled signal, runs its blocks.
__ush_defer_scope() {
  __ush_signal=''
  __ush_defers=''
  trap '__ush_defer_exit "${__ush_signal:-$?}"' 0
  trap '__ush_signal=129; exit 129' HUP
  trap '__ush_signal=130; exit 130' INT
  trap '__ush_signal=143; exit 143' TERM
}
__ush_defer_exit() {
  __ush_exit=$1
  __ush_run_defers
  exit "$__ush_exit"
}
