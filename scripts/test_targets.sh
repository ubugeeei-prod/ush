#!/bin/sh
set -eu

ush=${USH_BIN:-target/debug/ush}
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' 0
ln -s "$(command -v busybox)" "$tmp/ash"
PATH=$tmp:$PATH
export PATH

for target in sh bash zsh ash; do
  for source in examples/*.ush; do
    "$ush" compile "$source" --target "$target" -o "$tmp/script"
    "$target" -n "$tmp/script"
  done
  "$ush" compile examples/contracts.ush --target "$target" -o "$tmp/script"
  chmod +x "$tmp/script"
  actual=$("$tmp/script")
  [ "$actual" = "$(printf '%s\n' 2 20 ready done)" ]

  "$ush" compile examples/error_streams.ush --target "$target" -o "$tmp/script"
  code=0
  "$tmp/script" >"$tmp/out" 2>"$tmp/err" || code=$?
  [ "$code" -eq 1 ]
  grep -F 'error_streams.ush:6: Problem::MissingConfig' "$tmp/err"
done

# Exercise the portable target explicitly with dash and check its shell dialect.
"$ush" compile examples/contracts.ush -o "$tmp/script"
dash "$tmp/script"
shellcheck -s sh --severity=error "$tmp/script"
