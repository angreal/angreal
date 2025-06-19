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
    when_to_use=[
        "Setting up development environment",
        "Troubleshooting build issues",
        "Before running documentation or build tasks"
    ],
    when_not_to_use=[
        "During normal development workflow",
        "When all dependencies are known to be installed"
    ]
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
            print(f"✅ {dep[0]} is available")
        else:
            print(f"❌ {dep[0]} is not available - install via: {dep[1]}")
            missing_deps = True

    if missing_deps:
        print("\n⚠️  Some system dependencies are missing. "
              "Install them to use all features.")
        return 1
    else:
        print("\n🎉 All system dependencies are available!")
        return 0
