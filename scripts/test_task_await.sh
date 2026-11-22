#!/bin/sh
set -eu

ush=${1:?compiler path required}
target=${2:-sh}
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' 0

compile() {
  "$ush" compile "$tmp/job.ush" --target "$target" --checked-in -o "$tmp/job.sh"
  chmod +x "$tmp/job.sh"
  rm "$tmp/job.ush"
}

cat > "$tmp/job.ush" <<'USH'
fn build() -> Int / unknown, stderr {
  fail(23, "missing input")
}
let job = async build()
print job.await
USH
compile
status=0
"$tmp/job.sh" >"$tmp/out" 2>"$tmp/err" || status=$?
[ "$status" -eq 23 ]
[ ! -s "$tmp/out" ]
cat > "$tmp/expected" <<'EXPECTED'
job.ush:2: missing input (exit 23)
  fail(23, "missing input")
job.ush:5: command failed (exit 23)
  print job.await
EXPECTED
diff -u "$tmp/expected" "$tmp/err"

cat > "$tmp/job.ush" <<'USH'
fn build() -> Int / unknown, stdin, stderr, fs, net, env, exec {
  let pause = $(sleep 0.05 2>/dev/null || sleep 1)?
  41
}
let job = async build()
let first = async {
  let value = job.await
  value + 1
}
let second = async {
  let value = job.await
  value + 2
}
print first.await
print second.await
print job.await
print job.await
USH
compile
"$tmp/job.sh" >"$tmp/out" 2>"$tmp/err"
printf '%s\n' 42 43 41 41 > "$tmp/expected"
diff -u "$tmp/expected" "$tmp/out"
[ ! -s "$tmp/err" ]

cat > "$tmp/job.ush" <<'USH'
fn build() -> Int / unknown, stderr {
  fail(23, "missing input")
}
let job = async build()
let sibling = async {
  let value = job.await
  value
}
print sibling.await
USH
compile
status=0
"$tmp/job.sh" >"$tmp/out" 2>"$tmp/err" || status=$?
[ "$status" -eq 23 ]
[ ! -s "$tmp/out" ]
cat > "$tmp/expected" <<'EXPECTED'
job.ush:2: missing input (exit 23)
  fail(23, "missing input")
job.ush:6: command failed (exit 23)
  let value = job.await
job.ush:9: command failed (exit 23)
  print sibling.await
EXPECTED
diff -u "$tmp/expected" "$tmp/err"
