"""Run paired, checksum-checked timing samples and render the benchmark report."""

import argparse
from collections.abc import Sequence
import json
import os
from pathlib import Path
import platform
import statistics
import subprocess
import tempfile

from chart import format_ms, render_charts

ROOT = Path(__file__).resolve().parent.parent
TIERS = ("Small", "Medium", "Large")
WORKLOADS = (
    ("kadane", "Kadane", (8_192, 262_144, 8_388_608)),
    ("union-find", "Union-Find", (1_024, 16_384, 262_144)),
    ("sieve", "Prime sieve", (8_192, 262_144, 8_388_608)),
    ("kmp", "KMP first match", (8_192, 262_144, 8_388_608)),
    ("fenwick", "Fenwick sums", (1_024, 16_384, 262_144)),
    ("segment-tree", "Segment-tree sums", (1_024, 16_384, 262_144)),
)
CPP_FLAGS = ("-std=c++23", "-O3", "-DNDEBUG", "-Wall", "-Wextra", "-Wpedantic", "-Werror")
RUST_FLAGS = ("--edition=2024", "-C", "opt-level=3", "-D", "warnings")


def command(args: Sequence[str | Path]) -> str:
    return subprocess.check_output(args, cwd=ROOT, text=True).strip()


def distribution(values):
    p25, _, p75 = statistics.quantiles(values, n=4, method="inclusive")
    return {"median": statistics.median(values), "p25": p25, "p75": p75}


def input_description(key: str, size: int) -> str:
    if key == "kadane":
        return f"{size:,} integers"
    if key == "union-find":
        return f"{size:,} nodes, {4 * size:,} merges and connectivity checks"
    if key == "sieve":
        return f"Primality flags through {size:,}"
    if key == "kmp":
        return f"{size:,} text bytes, {max(1, size // 1024):,} pattern bytes"
    return f"{size:,} values, {4 * size:,} update–query pairs"


def measure_case(binaries, key, size, samples, warmups):
    pairs = []
    checksum = None
    for i in range(warmups + samples):
        current = {}
        order = ("cpp", "rust") if i % 2 == 0 else ("rust", "cpp")
        for language in order:
            elapsed, result = map(int, command([binaries[language], key, str(size)]).split())
            if elapsed <= 0:
                raise ValueError(f"{key}/{size}: nonpositive runtime from {language}")
            if checksum is None:
                checksum = result
            if result != checksum:
                raise ValueError(f"{key}/{size}: {language} checksum {result}, expected {checksum}")
            current[language] = elapsed
        if i >= warmups:
            pairs.append(current)
    return {
        "checksum": checksum,
        "samples_ns": pairs,
        "cpp_ms": distribution([pair["cpp"] / 1e6 for pair in pairs]),
        "rust_ms": distribution([pair["rust"] / 1e6 for pair in pairs]),
        "rust_over_cpp": distribution([pair["rust"] / pair["cpp"] for pair in pairs]),
    }


def platform_label():
    if platform.system() == "Linux":
        system = platform.freedesktop_os_release()["PRETTY_NAME"]
    elif platform.system() == "Darwin":
        system = f"macOS {platform.mac_ver()[0]}"
    else:
        system = platform.system()
    return f"{system} · {platform.machine()}"


def collect(cxx, samples, warmups, tiers):
    report = {
        "commit": command(["git", "rev-parse", "HEAD"]),
        "dirty": bool(command(["git", "status", "--porcelain"])),
        "machine": platform_label(),
        "runner_image": os.environ.get("ImageVersion"),
        "cpp": command([cxx, "--version"]).splitlines()[0],
        "rust": command(["rustc", "--version"]),
        "cpp_flags": CPP_FLAGS,
        "rust_flags": RUST_FLAGS,
        "samples": samples,
        "warmups": warmups,
        "tiers": [],
    }
    with tempfile.TemporaryDirectory() as directory:
        binaries = {language: Path(directory) / language for language in ("cpp", "rust")}
        command([cxx, *CPP_FLAGS, "benchmarks/run.cc", "-o", binaries["cpp"]])
        command(["rustc", *RUST_FLAGS, "benchmarks/run.rs", "-o", binaries["rust"]])
        for tier_index, tier in enumerate(TIERS):
            if tier.lower() not in tiers:
                continue
            cases = []
            for key, label, sizes in WORKLOADS:
                size = sizes[tier_index]
                print(f"{tier}: {label} ({size:,})", flush=True)
                cases.append({
                    "workload": key,
                    "label": label,
                    "size": size,
                    "input": input_description(key, size),
                    **measure_case(binaries, key, size, samples, warmups),
                })
            report["tiers"].append({"name": tier, "cases": cases})
    return report


def write_summary(report, destination):
    commit = report["commit"]
    revision = f"`{commit[:7]}`" + (" with local edits" if report["dirty"] else "")
    lines = [
        "# C++23 and Rust benchmarks",
        "",
        f"Revision {revision} on {report['machine']}.",
        f"{report['samples']} timed pairs follow {report['warmups']} unreported pairs per workload.",
        "",
        f"- C++ compiler `{report['cpp']}` with `{' '.join(report['cpp_flags'])}`",
        f"- Rust compiler `{report['rust']}` with `{' '.join(report['rust_flags'])}`",
    ]
    if report["runner_image"]:
        lines.append(f"- GitHub runner image `{report['runner_image']}`")
    lines.extend([
        "",
        "Both implementations receive the same deterministic input. The runner alternates which",
        "language goes first and rejects mismatched checksums. Timing starts after input generation",
        "and tree construction. The sieve includes allocating and building its result.",
        "",
        "Times are medians. Ratio intervals show the middle 50% of paired Rust / C++23 runtimes.",
        "A ratio below 1 favors Rust. A ratio above 1 favors C++23. Shared-runner load,",
        "compiler choices, algorithms, and data layouts can all affect the results.",
    ])
    for tier in report["tiers"]:
        lines.extend([
            "",
            f"## {tier['name']}",
            "",
            "| Workload | Input | C++23 (ms) | Rust (ms) | Rust / C++23 | Middle 50% |",
            "| --- | --- | ---: | ---: | ---: | ---: |",
        ])
        for case in tier["cases"]:
            ratio = case["rust_over_cpp"]
            values = [
                case["label"],
                case["input"],
                format_ms(case["cpp_ms"]["median"]),
                format_ms(case["rust_ms"]["median"]),
                f"{ratio['median']:.2f}×",
                f"{ratio['p25']:.2f}–{ratio['p75']:.2f}×",
            ]
            lines.append("| " + " | ".join(values) + " |")
    destination.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--cxx", default=os.environ.get("CXX", "c++"))
    parser.add_argument("--samples", type=int, default=12)
    parser.add_argument("--warmups", type=int, default=2)
    parser.add_argument("--tiers", nargs="+", choices=[tier.lower() for tier in TIERS])
    args = parser.parse_args()
    if args.samples < 2 or args.warmups < 0:
        parser.error("use at least two timed pairs and a nonnegative warmup count")

    report = collect(args.cxx, args.samples, args.warmups, args.tiers or [t.lower() for t in TIERS])
    args.output.mkdir(parents=True, exist_ok=True)
    (args.output / "results.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    write_summary(report, args.output / "README.md")

    render_charts(report, args.output)


if __name__ == "__main__":
    main()
