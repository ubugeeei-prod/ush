[runtime prelude]
__ush_runtime_map_origin='example.ush'
__ush_runtime_map_offset='39'
__ush_runtime_map_generated=''
__ush_runtime_map_section=''
__ush_runtime_map_source_line=''
__ush_runtime_map_source=''
__ush_runtime_map_shell=''
__ush_runtime_map_mapped=''

__ush_runtime_map_track() {
  __ush_runtime_map_generated="$1"
  __ush_runtime_map_section="$2"
  __ush_runtime_map_source_line="$3"
  __ush_runtime_map_source="$4"
  __ush_runtime_map_shell="$5"
  __ush_runtime_map_mapped="$6"
}

__ush_runtime_map_report() {
  __ush_runtime_map_status="$1"
  [ "$__ush_runtime_map_status" -eq 0 ] && return 0
  [ -z "$__ush_runtime_map_generated" ] && return "$__ush_runtime_map_status"
  __ush_runtime_map_line=$((__ush_runtime_map_generated + __ush_runtime_map_offset))
  if [ -n "$__ush_runtime_map_source_line" ]; then
    printf '\nush runtime map: %s:%s (exit %s)\n' "$__ush_runtime_map_origin" "$__ush_runtime_map_source_line" "$__ush_runtime_map_status" >&2
    printf '  source : %s\n' "$__ush_runtime_map_source" >&2
  else
    printf '\nush runtime map: %s (exit %s)\n' "$__ush_runtime_map_origin" "$__ush_runtime_map_status" >&2
    printf '  source : (no direct source mapping)\n' >&2
  fi
  printf '  section: %s\n' "$__ush_runtime_map_section" >&2
  printf '  shell  : line %s | G%04d | %s\n' "$__ush_runtime_map_line" "$__ush_runtime_map_generated" "$__ush_runtime_map_shell" >&2
  printf '  mapped : %s\n' "$__ush_runtime_map_mapped" >&2
  printf '  explain: ush explain %s %s\n' "$__ush_runtime_map_origin" "$__ush_runtime_map_line" >&2
  return "$__ush_runtime_map_status"
}

trap '__ush_runtime_map_report "$?"' 0


[tail]
__ush_runtime_map_track '449' 'user-code' '1' 'print "hello"' 'printf '"'"'%s\n'"'"' '"'"'hello'"'"'' 'G0449'; printf '%s\n' 'hello'

if [ -n "$__ush_jobs" ]; then
  for __ush_job in $__ush_jobs; do
    wait "$__ush_job" 2>/dev/null || true
  done
fi
if [ -n "$__ush_task_files" ]; then
  for __ush_task_file in $__ush_task_files; do
    rm -f "$__ush_task_file"
  done
fi
