"""Installation smoke tests for angreal."""
import subprocess
import sys
import tempfile
from pathlib import Path

import angreal

project_root = Path(angreal.get_root()).parent

smoke = angreal.command_group(name="smoke", about="installation smoke tests")


@smoke()
@angreal.command(
    name="install",
    about="Test fresh installation in isolated environment",
    tool=angreal.ToolDescription("""
Build wheel, install in isolated environment, and verify basic commands work.

## When to use
- Before releases
- After build system changes
- To verify installation works end-to-end

## When NOT to use
- During rapid development
- When build system unchanged

## Examples
```
angreal smoke install
```
""", risk_level="safe")
)
def smoke_install():
    """
    Build wheel, install in isolated environment, and verify basic commands work.
    Catches missing dependencies, broken entry points, or runtime panics.
    """
    print("=== Installation Smoke Test ===\n")

    with tempfile.TemporaryDirectory() as tmpdir:
        tmpdir = Path(tmpdir)

        # Create isolated venv
        print("1. Creating isolated virtual environment...")
        venv_path = tmpdir / "venv"
        subprocess.run([sys.executable, "-m", "venv", str(venv_path)], check=True)

        if sys.platform == "win32":
            pip = venv_path / "Scripts" / "pip.exe"
            angreal_bin = venv_path / "Scripts" / "angreal.exe"
        else:
            pip = venv_path / "bin" / "pip"
            angreal_bin = venv_path / "bin" / "angreal"

        # Ensure pip
        bin_dir = "Scripts" if sys.platform == "win32" else "bin"
        subprocess.run(
            [str(venv_path / bin_dir / "python"), "-m", "ensurepip"],
            check=True
        )

        # Install maturin
        print("2. Installing maturin...")
        subprocess.run([str(pip), "install", "maturin"], check=True)

        # Build wheel
        print("3. Building wheel with maturin...")
        wheel_dir = tmpdir / "wheels"
        wheel_dir.mkdir()
        result = subprocess.run([
            str(pip).replace("pip", "maturin").replace(".exe", ".exe"),
            "build", "--release",
            "--manifest-path", str(project_root / "crates" / "angreal" / "Cargo.toml"),
            "--out", str(wheel_dir)
        ], capture_output=True, text=True)

        # maturin might not be in PATH, try with python -m
        if result.returncode != 0:
            manifest = str(project_root / "crates" / "angreal" / "Cargo.toml")
            result = subprocess.run([
                str(venv_path / bin_dir / "python"),
                "-m", "maturin",
                "build", "--release",
                "--manifest-path", manifest,
                "--out", str(wheel_dir)
            ], check=True)

        # Find and install wheel
        print("4. Installing wheel...")
        wheels = list(wheel_dir.glob("*.whl"))
        if not wheels:
            print(f"FAIL: No wheel found in {wheel_dir}")
            return 1
        wheel = wheels[0]
        print(f"   Installing: {wheel.name}")
        subprocess.run([str(pip), "install", str(wheel)], check=True)

        # Test --version
        print("5. Testing --version...")
        result = subprocess.run(
            [str(angreal_bin), "--version"],
            capture_output=True, text=True
        )
        if result.returncode != 0:
            print(f"FAIL: --version failed: {result.stderr}")
            return 1
        if "angreal" not in result.stdout.lower():
            print(f"FAIL: Unexpected version output: {result.stdout}")
            return 1
        print(f"   OK: {result.stdout.strip()}")

        # Test --help
        print("6. Testing --help...")
        result = subprocess.run(
            [str(angreal_bin), "--help"],
            capture_output=True, text=True
        )
        if result.returncode != 0:
            print(f"FAIL: --help failed: {result.stderr}")
            return 1
        if "usage" not in result.stdout.lower():
            print(f"FAIL: Unexpected help output: {result.stdout[:200]}")
            return 1
        print("   OK: Help displays correctly")

        print("\nPASS: Installation smoke test")
