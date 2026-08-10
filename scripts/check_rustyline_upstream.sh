#!/bin/sh
set -eu

# Compare the pinned vendor/rustyline tag against the latest stable
# release published on crates.io. Fails when the vendored copy is
# behind — this is the signal to review upstream for advisories and
# either rebase the vendored copy or document why we are staying.

UPSTREAM_FILE="vendor/rustyline/UPSTREAM"

if [ ! -f "$UPSTREAM_FILE" ]; then
  echo "check_rustyline_upstream: missing $UPSTREAM_FILE" >&2
  exit 1
fi

pinned_tag="$(awk -F': *' '/^tag:/ { print $2; exit }' "$UPSTREAM_FILE")"
if [ -z "$pinned_tag" ]; then
  echo "check_rustyline_upstream: could not read 'tag:' from $UPSTREAM_FILE" >&2
  exit 1
fi
pinned_version="${pinned_tag#v}"

vendored_version="$(awk -F'"' '/^version =/ { print $2; exit }' vendor/rustyline/Cargo.toml)"
if [ -z "$vendored_version" ]; then
  echo "check_rustyline_upstream: could not read version from vendor/rustyline/Cargo.toml" >&2
  exit 1
fi

if [ "$pinned_version" != "$vendored_version" ]; then
  cat >&2 <<MSG
check_rustyline_upstream: tag drift inside the vendor tree.
  UPSTREAM tag: $pinned_tag (=$pinned_version)
  Cargo.toml:   $vendored_version
Update one or the other so they agree.
MSG
  exit 1
fi

# crates.io rejects requests without a descriptive User-Agent with a
# bare 403, so send one. Fetch and parse are separate steps: piping
# curl straight into python turns a network failure into an opaque
# JSONDecodeError traceback instead of a readable message.
crates_json="$(curl -fsSL \
  -A "ush-ci check_rustyline_upstream (https://github.com/ubugeeei-prod/ush)" \
  https://crates.io/api/v1/crates/rustyline)" || {
  echo "check_rustyline_upstream: failed to query crates.io for rustyline" >&2
  exit 1
}

latest_version="$(printf '%s' "$crates_json" \
  | python3 -c 'import json,sys; print(json.load(sys.stdin)["crate"]["max_stable_version"])')" || {
  echo "check_rustyline_upstream: could not parse crates.io response" >&2
  exit 1
}

if [ -z "$latest_version" ]; then
  echo "check_rustyline_upstream: could not determine latest rustyline version" >&2
  exit 1
fi

echo "pinned: $pinned_version"
echo "latest: $latest_version"

if [ "$pinned_version" != "$latest_version" ]; then
  cat >&2 <<MSG
check_rustyline_upstream: vendored rustyline is behind upstream.
  Pinned: $pinned_version
  Latest: $latest_version

Review https://github.com/kkawakam/rustyline/releases/tag/v$latest_version
for security advisories or behaviour changes, then either:
  * rebase the vendored copy onto the new tag and update
    vendor/rustyline/UPSTREAM, or
  * document an explicit reason for staying on the old tag.
MSG
  exit 1
fi

echo "rustyline vendored copy matches latest upstream."
