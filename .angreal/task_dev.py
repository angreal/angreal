import angreal
import os
import shutil

project_root = os.path.join(angreal.get_root(),'..')

dev = angreal.command_group(name="dev", about="development utilities")

def is_program_available(program_name):
    return shutil.which(program_name) is not None

@dev()
@angreal.command(
    name="check-deps",
    about="Verify required development tools are installed",
    tool=angreal.ToolDescription("""
Checks for system-level dependencies required for angreal development.

## When to use
- Setting up a new development environment
- Troubleshooting build failures
- Before running documentation or build tasks for the first time

## When NOT to use
- During normal development workflow when deps are known to be installed
- In CI/CD pipelines (use explicit dependency installation instead)

## Examples
```
angreal dev check-deps
```

## Output
Shows a checklist of required tools (hugo, cargo) with installation
instructions for any that are missing.
""", risk_level="read_only")
)
def check_system_dependencies():
    """
    Check for required system-level dependencies
    """
    dependencies_required = (
        ("hugo" , "please visit : https://gohugo.io/installation/"),
        ("cargo", "curl --proto '=https' --tlsv1.2"
         " -sSf https://sh.rustup.rs | sh && rustup update")
    )

    missing_deps = False
    for dep in dependencies_required:
        if is_program_available(dep[0]):
            print(f"OK: {dep[0]} is available")
        else:
            print(f"MISSING: {dep[0]} - install via: {dep[1]}")
            missing_deps = True

    if missing_deps:
        print("\nWARN: Some system dependencies are missing. "
              "Install them to use all features.")
        return 1
    else:
        print("\nAll system dependencies are available!")
        return 0
