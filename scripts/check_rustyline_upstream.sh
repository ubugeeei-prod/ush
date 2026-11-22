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

# The Cargo sparse index exposes published versions and yanked status.
# Check the maintained major separately so a reviewed major never hides a patch.
versions="$(curl -fsSL --retry 3 https://index.crates.io/ru/st/rustyline | python3 -c '
import json, re, sys
stable = []
for line in sys.stdin:
    item = json.loads(line)
    if not item["yanked"] and re.fullmatch(r"[0-9]+\.[0-9]+\.[0-9]+", item["vers"]):
        stable.append(tuple(map(int, item["vers"].split("."))))
major = int(sys.argv[1].split(".")[0])
for version in [max(stable), max(v for v in stable if v[0] == major)]:
    print(".".join(map(str, version)))
' "$pinned_version")"
latest_version=$(printf '%s\n' "$versions" | head -n 1)
latest_patch=$(printf '%s\n' "$versions" | tail -n 1)
reviewed=$(awk -F': *' '/^reviewed:/ { print $2; exit }' "$UPSTREAM_FILE")

echo "pinned: $pinned_version, latest in major: $latest_patch, latest overall: $latest_version"
if [ "$pinned_version" != "$latest_patch" ] ||
   { [ "$pinned_version" != "$latest_version" ] && [ "$reviewed" != "v$latest_version" ]; }; then
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

echo "Vendored rustyline is current in its major and upstream releases have been reviewed."
