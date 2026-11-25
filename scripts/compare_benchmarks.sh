#!/bin/sh
# Generated from 'compare_benchmarks.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: compare_benchmarks.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'compare_benchmarks.ush' --checked-in -o 'compare_benchmarks.sh'
set -eu

__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) __ush_entrypoint=$(command -v "$__ush_entrypoint") ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'compare_benchmarks.ush'
__ush_origin='compare_benchmarks.ush'

__ush_at() { __ush_line=$1; __ush_source=$2; }
__ush_report() {
  if [ "$1" -ne 0 ] && [ -n "${__ush_line:-}" ]; then
    __ush_message=${2:-command failed}
    if [ "$#" -lt 2 ]; then
      case "$1" in
        127) __ush_message='command not found; check the command name and PATH' ;;
        126) __ush_message='command cannot be executed; check permissions and interpreter' ;;
        130) __ush_message='interrupted' ;;
        137) __ush_message='process killed (SIGKILL)' ;;
        143) __ush_message='terminated (SIGTERM)' ;;
      esac
    fi
    printf '%s:%s: %s (exit %s)\n  %s\n' "$__ush_origin" "$__ush_line" "$__ush_message" "$1" "$__ush_source" >&2
    __ush_line=''
  fi
}
trap '__ush_report "$?"' 0

case "${1:-}" in
  -h|--help) printf '%s' 'compare_benchmarks.ush

Usage:
  compare_benchmarks.ush [--help]
  compare_benchmarks.ush [--man [ITEM]]
  compare_benchmarks.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
compare_benchmarks.ush

SYNOPSIS
compare_benchmarks.ush [--help]
compare_benchmarks.ush [--man [ITEM]]
compare_benchmarks.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

__ush_at 2 'shell """'
python3 - "$@" <<'USH_BENCH_PY'
"""Compare the PR to its merge base on the same machine. Times are nanoseconds."""

import json
import math
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path


SUITES = [("ush_shell", "parser"), ("ush_compiler", "compile")]
MAX_RATIO = 1.25


def build_binary(checkout, package, suite, destination, target_dir):
    command = ["cargo", "bench", "--locked", "--no-run", "-p", package,
               "--bench", suite, "--message-format=json"]
    output = subprocess.check_output(command, cwd=checkout, text=True,
                                     env={**os.environ, "CARGO_TARGET_DIR": str(target_dir)})
    artifacts = (json.loads(line) for line in output.splitlines())
    binary, = [item["executable"] for item in artifacts
               if item["reason"] == "compiler-artifact" and item.get("executable")
               and item["target"]["name"] == suite]
    shutil.copy2(binary, destination)
    return destination


def list_cases(binary):
    suffix = ": benchmark"
    output = subprocess.check_output([binary, "--list"], text=True)
    cases = [line[:-len(suffix)] for line in output.splitlines() if line.endswith(suffix)]
    if not cases:
        raise ValueError(f"No benchmarks in {binary}")
    return cases


def measure(binary, name, directory):
    # Two 50-sample runs per revision preserve 100 samples, 3 s of warmup
    # and 5 s of measurement in total. Each run has its own Criterion output.
    command = [binary, "--bench", "--exact", name, "--noplot",
               "--sample-size", "50", "--warm-up-time", "1.5",
               "--measurement-time", "2.5", "--nresamples", "10000"]
    with tempfile.TemporaryDirectory(dir=directory) as run:
        result = subprocess.run(command, env={**os.environ, "CRITERION_HOME": run},
                                text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        if result.returncode:
            raise RuntimeError(result.stdout)
        estimate_file, = Path(run).glob("**/new/estimates.json")
        estimates = json.loads(estimate_file.read_text())
        return (estimates["slope"] or estimates["mean"])["point_estimate"]


def compare_suite(binaries, directory, report):
    base, head = binaries
    base_cases, head_cases = list_cases(base), list_cases(head)
    failures = []
    for name in dict.fromkeys(base_cases + head_cases):
        if name not in base_cases or name not in head_cases:
            is_new = name in head_cases
            ns = measure(head if is_new else base, name, directory)
            cells = f"| — | {ns:,.2f} | new" if is_new else f"| {ns:,.2f} | — | removed"
            row = f"| {name} {cells} |"
        else:
            # Mirror the run order to balance warmup and clock-frequency drift.
            b1, h1, h2, b2 = (measure(exe, name, directory) for exe in (base, head, head, base))
            before, after = math.sqrt(b1 * b2), math.sqrt(h1 * h2)
            ratio = after / before
            row = f"| {name} | {before:,.2f} | {after:,.2f} | {ratio:.3f}× |"
            if ratio > MAX_RATIO:
                failures.append(name)
        print(row, flush=True)
        report.append(row)
    return failures


def main():
    head = Path.cwd()
    base = Path(sys.argv[1]).resolve()
    report = ["## Paired Criterion measurements", "",
              "Base and PR binaries run on the same runner. The regression threshold is 1.25×.", "",
              "| Benchmark | Base (ns) | PR (ns) | Ratio |", "| --- | ---: | ---: | ---: |"]
    failures = []
    print("\n".join(report), flush=True)
    with tempfile.TemporaryDirectory() as tmp:
        directory = Path(tmp)
        for package, suite in SUITES:
            binaries = [
                build_binary(base, package, suite, directory / f"base-{suite}", head / "target" / "bench-base"),
                build_binary(head, package, suite, directory / f"head-{suite}", head / "target"),
            ]
            failures.extend(compare_suite(binaries, directory, report))
    if summary := os.environ.get("GITHUB_STEP_SUMMARY"):
        with open(summary, "a", encoding="utf-8") as output:
            output.write("\n".join(report) + "\n")
    if failures:
        sys.exit("Performance regression above 25%: " + ", ".join(failures))


if __name__ == "__main__":
    main()

USH_BENCH_PY
