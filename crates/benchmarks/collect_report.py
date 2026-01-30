#!/usr/bin/env python3
"""Format cargo benchmark results from criterion estimates.json files to markdown table."""

import argparse
import glob
import json
import os
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import List, Optional


@dataclass
class BenchmarkResult:
    """Benchmark result data class."""

    name: str
    lower: float
    mean: float
    upper: float
    unit: str


def parse_estimates_json(json_path: Path) -> Optional[BenchmarkResult]:
    """Parse a single estimates.json file.

    Args:
        json_path: Path to the estimates.json file.

    Returns:
        BenchmarkResult if successful, None otherwise.
    """
    try:
        with open(json_path, "r", encoding="utf-8") as f:
            data = json.load(f)

        slope = data.get("slope")
        if not slope:
            return None

        lower_bound = slope["confidence_interval"]["lower_bound"]
        upper_bound = slope["confidence_interval"]["upper_bound"]
        point_estimate = slope["point_estimate"]

        # Extract benchmark name from directory path
        # Path format: target/criterion/<benchmark_name>/new/estimates.json
        parts = json_path.parts
        criterion_idx = -1
        for i, part in enumerate(parts):
            if part == "criterion":
                criterion_idx = i
                break

        if criterion_idx >= 0 and criterion_idx + 1 < len(parts):
            benchmark_name = parts[criterion_idx + 1]
        else:
            benchmark_name = json_path.parent.parent.name

        # Convert to microseconds for display
        # Criterion stores in nanoseconds by default
        unit = "ns"
        if lower_bound > 1000:
            lower_bound /= 1000
            upper_bound /= 1000
            point_estimate /= 1000
            unit = "µs"

        return BenchmarkResult(
            name=benchmark_name,
            lower=lower_bound,
            mean=point_estimate,
            upper=upper_bound,
            unit=unit
        )

    except (json.JSONDecodeError, KeyError, IOError) as e:
        print(f"Error parsing {json_path}: {e}", file=sys.stderr)
        return None


def find_estimates_files(base_path: Path) -> List[Path]:
    """Find all estimates.json files under target/criterion.

    Args:
        base_path: Base directory to search from (default: current directory).

    Returns:
        List of paths to estimates.json files.
    """
    # Search for target/criterion/**/new/estimates.json
    search_pattern = base_path / "target" / "criterion" / "*" / "new" / "estimates.json"
    return list(Path(base_path).glob("target/criterion/*/new/estimates.json"))


def generate_markdown_table(results: List[BenchmarkResult]) -> str:
    """Generate markdown table from benchmark results.

    Args:
        results: List of benchmark results.

    Returns:
        Markdown table string.
    """
    if not results:
        return "No benchmark results found."

    # Sort by name for consistent output
    results.sort(key=lambda x: x.name)

    table = []
    table.append("| Benchmark Name | Lower | Mean | Upper | Unit |")
    table.append("|----------------|-------|------|-------|------|")

    for result in results:
        table.append(f"| {result.name} | {result.lower:.4f} | {result.mean:.4f} | {result.upper:.4f} | {result.unit} |")

    return "\n".join(table)


def main():
    """Main function to run the script."""
    parser = argparse.ArgumentParser(
        description="Format cargo benchmark results from criterion estimates.json to markdown table"
    )
    parser.add_argument(
        "-d", "--directory",
        default=".",
        help="Base directory containing target/criterion (default: current directory)"
    )
    parser.add_argument(
        "-o", "--output",
        help="Output file (default: stdout)"
    )
    args = parser.parse_args()

    # Find all estimates.json files
    base_path = Path(args.directory).resolve()
    estimates_files = find_estimates_files(base_path)

    if not estimates_files:
        print(f"No estimates.json files found under {base_path}/target/criterion/", file=sys.stderr)
        return 1

    # Parse all files
    results = []
    for json_path in estimates_files:
        result = parse_estimates_json(json_path)
        if result:
            results.append(result)

    # Generate and output table
    markdown_table = generate_markdown_table(results)

    if args.output:
        with open(args.output, "w", encoding="utf-8") as f:
            f.write(markdown_table)
    else:
        print(markdown_table)

    return 0


if __name__ == "__main__":
    sys.exit(main())
