#!/usr/bin/env python3
# /// script
# requires-python = ">=3.10"
# ///

"""archr CLI wrapper for AI agent workflows."""

import argparse
import json
import os
import subprocess
import sys
import shutil


def _find_binary() -> str | None:
    """Find the archr binary path from ARCHR_BIN env var or PATH."""
    return os.getenv("ARCHR_BIN") or shutil.which("archr")


def _check_version(binary: str) -> bool:
    """Check if archr binary version is >= 1.0.0.

    Args:
        binary: Path to the archr binary.

    Returns:
        True if version is >= 1.0.0, False otherwise.
    """
    try:
        result = subprocess.run(
            [binary, "--version"],
            capture_output=True,
            text=True,
            timeout=5,
        )
        output = result.stdout.strip()
        if not output:
            return False

        # Parse semver: "archr 1.0.0" or "archr 10.0.5"
        parts = output.split()
        if len(parts) < 2:
            return False

        version_str = parts[1]
        try:
            version_parts = version_str.split(".")
            major = int(version_parts[0])
            minor = int(version_parts[1])
        except (ValueError, IndexError):
            return False

        return (major, minor) >= (1, 0)
    except (subprocess.TimeoutExpired, subprocess.CalledProcessError, FileNotFoundError):
        return False


def _run(binary: str, args: list[str], timeout: int = 10) -> tuple[int, str, str]:
    """Run the archr subprocess with timeout.

    Args:
        binary: Path to archr binary.
        args: Command-line arguments to pass to archr.
        timeout: Maximum seconds to wait for completion.

    Returns:
        Tuple of (exit_code, stdout, stderr).
    """
    try:
        result = subprocess.run(
            [binary] + args,
            capture_output=True,
            text=True,
            timeout=timeout,
        )
        return result.returncode, result.stdout, result.stderr
    except subprocess.TimeoutExpired:
        print("ERROR: archr command timed out", file=sys.stderr)
        sys.exit(4)
    except FileNotFoundError:
        print("ERROR: archr binary not found", file=sys.stderr)
        sys.exit(2)
    except Exception as e:
        print(f"ERROR: {e}", file=sys.stderr)
        sys.exit(2)


def cmd_validate(args: argparse.Namespace) -> None:
    """Validate a YAML file against ArchiMate rules.

    Args:
        args: Parsed command-line arguments.
    """
    binary = _find_binary()
    if binary is None:
        print("ERROR: archr binary not found", file=sys.stderr)
        sys.exit(2)

    if not _check_version(binary):
        print("ERROR: archr binary version incompatible (requires ≥ 1.0.0)", file=sys.stderr)
        sys.exit(3)

    if not os.path.exists(args.input):
        print(f"ERROR: Input file not found: {args.input}", file=sys.stderr)
        sys.exit(2)

    exit_code, stdout, stderr = _run(
        binary,
        ["validate", "--input", args.input, "--format", "json"],
    )

    print(stdout, end="")
    sys.exit(exit_code)


def cmd_generate(args: argparse.Namespace) -> None:
    """Generate an Open Exchange XML file from YAML.

    Args:
        args: Parsed command-line arguments.
    """
    binary = _find_binary()
    if binary is None:
        print("ERROR: archr binary not found", file=sys.stderr)
        sys.exit(2)

    if not _check_version(binary):
        print("ERROR: archr binary version incompatible (requires ≥ 1.0.0)", file=sys.stderr)
        sys.exit(3)

    if not os.path.exists(args.input):
        print(f"ERROR: Input file not found: {args.input}", file=sys.stderr)
        sys.exit(2)

    # Ensure output directory exists
    output_dir = os.path.dirname(args.output) or "."
    os.makedirs(output_dir, exist_ok=True)

    exit_code, stdout, stderr = _run(
        binary,
        ["generate", "--input", args.input, "--output", args.output],
    )

    print(stdout, end="")
    sys.exit(exit_code)


def main() -> None:
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Validate and generate ArchiMate 3.2 models using archr",
        prog="archr",
    )

    parser.add_argument(
        "--version",
        action="store_true",
        help="Show archr version",
    )

    subparsers = parser.add_subparsers(dest="command", help="Available commands")

    # validate subcommand
    validate_parser = subparsers.add_parser("validate", help="Validate YAML against ArchiMate rules")
    validate_parser.add_argument("input", help="Input YAML file path")
    validate_parser.set_defaults(func=cmd_validate)

    # generate subcommand
    generate_parser = subparsers.add_parser("generate", help="Generate Open Exchange XML from YAML")
    generate_parser.add_argument("input", help="Input YAML file path")
    generate_parser.add_argument("--output", required=True, help="Output .archimate XML file path")
    generate_parser.set_defaults(func=cmd_generate)

    args = parser.parse_args()

    # Check if binary exists
    binary = _find_binary()
    if binary is None:
        print("ERROR: archr binary not found", file=sys.stderr)
        sys.exit(2)

    # Show version if requested
    if args.version:
        exit_code, stdout, _ = _run(binary, ["--version"])
        print(stdout, end="")
        sys.exit(exit_code)

    # Validate binary version
    if not _check_version(binary):
        print("ERROR: archr binary version incompatible (requires ≥ 1.0.0)", file=sys.stderr)
        sys.exit(3)

    # No command provided
    if not args.command:
        parser.print_help()
        sys.exit(64)

    # Execute subcommand
    args.func(args)


if __name__ == "__main__":
    main()
