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
