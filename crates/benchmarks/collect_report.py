#!/usr/bin/env python3
"""Format cargo benchmark results from criterion estimates.json files to markdown table."""

import argparse
import glob
import json
import os
import platform
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import List, Optional


def get_system_info() -> str:
    """Get system hardware information.
    
    Returns:
        String containing system hardware information.
    """
    info_lines = []
    
    # Get CPU information from /proc/cpuinfo
    cpu_info = None
    cpu_cores = os.cpu_count()
    
    if os.path.exists("/proc/cpuinfo"):
        with open("/proc/cpuinfo", "r") as f:
            for line in f:
                if line.startswith("model name"):
                    cpu_info = line.split(":", 1)[1].strip()
                    break
    
    if cpu_info:
        info_lines.append(f"- **CPU 型号**: {cpu_info}")
    else:
        info_lines.append("- **CPU 型号**: 无法获取 CPU 信息")
    
    info_lines.append(f"- **CPU 核心数**: {cpu_cores}")
    
    # Get memory information
    try:
        if sys.platform == "linux":
            # Linux: read from /proc/meminfo
            with open("/proc/meminfo", "r") as f:
                for line in f:
                    if line.startswith("MemTotal:"):
                        mem_kb = int(line.split()[1])
                        mem_gb = mem_kb / 1024 / 1024
                        info_lines.append(f"- **内存大小**: {mem_gb:.1f} GB")
                        break
        elif sys.platform == "darwin":
            # macOS: use sysctl
            result = subprocess.run(["sysctl", "-n", "hw.memsize"], 
                                  capture_output=True, text=True)
            if result.returncode == 0:
                mem_bytes = int(result.stdout.strip())
                mem_gb = mem_bytes / 1024 / 1024 / 1024
                info_lines.append(f"- **内存大小**: {mem_gb:.1f} GB")
        else:
            # Windows or other platforms
            info_lines.append("- **内存大小**: 该平台无法获取内存信息")
    except Exception as e:
        info_lines.append(f"- **内存大小**: 无法获取内存信息: {e}")
    
    # Get OS information
    try:
        os_info = f"{platform.system()} {platform.release()}"
        info_lines.append(f"- **操作系统**: {os_info}")
    except Exception as e:
        info_lines.append(f"- **操作系统**: 无法获取操作系统信息: {e}")
    
    return "\n".join(info_lines)


@dataclass
class BenchmarkResult:
    """Benchmark result data class."""

    name: str
    lower: float
    mean: float
    upper: float
    unit: str


def format_benchmark_name(name: str, prefix: str) -> str:
    """Format benchmark name by removing prefix and handling async_ prefix.

    Args:
        name: The original benchmark name.
        prefix: The prefix to remove (e.g., "instantiate_" or "call_").

    Returns:
        Formatted benchmark name.
    """
    # Remove the specified prefix
    name = name.removeprefix(prefix)

    # If the name starts with "async_", move it to suffix as "-async"
    if name.startswith("async_"):
        name = name[6:] + "-async"

    return name


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

        # Try to get slope or mean data
        slope = data.get("slope")
        mean = data.get("mean")

        # Use slope if available, otherwise use mean
        if slope:
            lower_bound = slope["confidence_interval"]["lower_bound"]
            upper_bound = slope["confidence_interval"]["upper_bound"]
            point_estimate = slope["point_estimate"]
        elif mean:
            lower_bound = mean["confidence_interval"]["lower_bound"]
            upper_bound = mean["confidence_interval"]["upper_bound"]
            point_estimate = mean["point_estimate"]
        else:
            return None

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

    # Add system information header
    system_info = get_system_info()
    
    table = []
    table.append("# 基准测试结果")
    table.append("")
    table.append("## 系统信息")
    table.append(system_info)
    table.append("")
    table.append("## 结果")
    table.append("时间单位：微妙")

    # 1. 总览表格
    table.append("")
    table.append("### 总览")
    table.append("| 基准测试名称 | 下限 | 平均值 | 上限 | 单位 |")
    table.append("|--------------|------|--------|------|------|")
    for result in results:
        table.append(f"| {result.name} | {result.lower:.4f} | {result.mean:.4f} | {result.upper:.4f} | {result.unit} |")

    # 2. v21 实例化
    v21_instantiate = [r for r in results if r.name.startswith("instantiate_") and r.name.endswith("_v21")]
    if v21_instantiate:
        table.append("")
        table.append("### v21 实例化")
        table.append("| 基准测试名称 | 下限 | 平均值 | 上限 | 单位 |")
        table.append("|--------------|------|--------|------|------|")
        for result in sorted(v21_instantiate, key=lambda x: format_benchmark_name(x.name, "instantiate_")):
            name = format_benchmark_name(result.name, "instantiate_")
            table.append(f"| {name} | {result.lower:.4f} | {result.mean:.4f} | {result.upper:.4f} | {result.unit} |")

    # 3. v21 函数调用
    v21_call = [r for r in results if r.name.startswith("call_") and r.name.endswith("_v21")]
    if v21_call:
        table.append("")
        table.append("### v21 函数调用")
        table.append("| 基准测试名称 | 下限 | 平均值 | 上限 | 单位 |")
        table.append("|--------------|------|--------|------|------|")
        for result in sorted(v21_call, key=lambda x: format_benchmark_name(x.name, "call_")):
            name = format_benchmark_name(result.name, "call_")
            table.append(f"| {name} | {result.lower:.4f} | {result.mean:.4f} | {result.upper:.4f} | {result.unit} |")

    # 4. v41 实例化
    v41_instantiate = [r for r in results if r.name.startswith("instantiate_") and r.name.endswith("_v41")]
    if v41_instantiate:
        table.append("")
        table.append("### v41 实例化")
        table.append("| 基准测试名称 | 下限 | 平均值 | 上限 | 单位 |")
        table.append("|--------------|------|--------|------|------|")
        for result in sorted(v41_instantiate, key=lambda x: format_benchmark_name(x.name, "instantiate_")):
            name = format_benchmark_name(result.name, "instantiate_")
            table.append(f"| {name} | {result.lower:.4f} | {result.mean:.4f} | {result.upper:.4f} | {result.unit} |")

    # 5. v41 函数调用
    v41_call = [r for r in results if r.name.startswith("call_") and r.name.endswith("_v41")]
    if v41_call:
        table.append("")
        table.append("### v41 函数调用")
        table.append("| 基准测试名称 | 下限 | 平均值 | 上限 | 单位 |")
        table.append("|--------------|------|--------|------|------|")
        for result in sorted(v41_call, key=lambda x: format_benchmark_name(x.name, "call_")):
            name = format_benchmark_name(result.name, "call_")
            table.append(f"| {name} | {result.lower:.4f} | {result.mean:.4f} | {result.upper:.4f} | {result.unit} |")

    # 6. v21 vs v41 对比
    # 实例化对比
    table.append("")
    table.append("### v21 vs v41 对比")
    table.append("")
    table.append("#### 实例化对比")
    table.append("| 基准测试名称 | v21 平均值 | v41 平均值 |")
    table.append("|--------------|-----------|----------|")

    # Build name-based comparison for instantiate
    instantiate_results_by_name = {}
    for result in results:
        if result.name.startswith("instantiate_") and (result.name.endswith("_v21") or result.name.endswith("_v41")):
            base_name = result.name[:-4]  # Remove _v21 or _v41
            if base_name not in instantiate_results_by_name:
                instantiate_results_by_name[base_name] = {}
            if result.name.endswith("_v21"):
                instantiate_results_by_name[base_name]["v21"] = result
            elif result.name.endswith("_v41"):
                instantiate_results_by_name[base_name]["v41"] = result

    for base_name in sorted(instantiate_results_by_name.keys(), key=lambda x: format_benchmark_name(x, "instantiate_")):
        data = instantiate_results_by_name[base_name]
        if "v21" in data and "v41" in data:
            v21 = data["v21"]
            v41 = data["v41"]
            # Convert to microseconds if needed
            v21_mean = v21.mean if v21.unit == "µs" else v21.mean / 1000
            v41_mean = v41.mean if v41.unit == "µs" else v41.mean / 1000
            name = format_benchmark_name(base_name, "instantiate_")
            table.append(f"| {name} | {v21_mean:.4f} | {v41_mean:.4f} |")

    # 函数调用对比
    table.append("")
    table.append("#### 函数调用对比")
    table.append("| 基准测试名称 | v21 平均值 | v41 平均值 |")
    table.append("|--------------|-----------|----------|")

    # Build name-based comparison for call
    call_results_by_name = {}
    for result in results:
        if result.name.startswith("call_") and (result.name.endswith("_v21") or result.name.endswith("_v41")):
            base_name = result.name[:-4]  # Remove _v21 or _v41
            if base_name not in call_results_by_name:
                call_results_by_name[base_name] = {}
            if result.name.endswith("_v21"):
                call_results_by_name[base_name]["v21"] = result
            elif result.name.endswith("_v41"):
                call_results_by_name[base_name]["v41"] = result

    for base_name in sorted(call_results_by_name.keys(), key=lambda x: format_benchmark_name(x, "call_")):
        data = call_results_by_name[base_name]
        if "v21" in data and "v41" in data:
            v21 = data["v21"]
            v41 = data["v41"]
            # Convert to microseconds if needed
            v21_mean = v21.mean if v21.unit == "µs" else v21.mean / 1000
            v41_mean = v41.mean if v41.unit == "µs" else v41.mean / 1000
            name = format_benchmark_name(base_name, "call_")
            table.append(f"| {name} | {v21_mean:.4f} | {v41_mean:.4f} |")

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

    print(f"Found {estimates_files}")

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
