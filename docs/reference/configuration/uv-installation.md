---
title: "UV Installation and Management"
weight: 10
---

# UV Installation and Management

This document explains how Angreal manages the UV binary for ultra-fast virtual environment operations.

## Automatic Installation

UV is automatically installed when Angreal needs it, with no manual setup required.

### When Installation Occurs

UV installation is triggered automatically when:

1. **First Virtual Environment Operation**: Any `VirtualEnv` class usage
2. **Module Import**: When `angreal.integrations.venv` is imported
3. **Decorator Usage**: When `@venv_required` is applied to a function
4. **Binary Not Found**: When UV is not available in the system PATH

### Installation Methods

#### Unix/macOS Installation
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

**Features:**
- Downloads latest stable UV release
- Installs to `~/.local/bin/` (added to PATH)
- Works on macOS and Linux distributions
- Uses secure HTTPS download

#### Windows Installation
```powershell
irm https://astral.sh/uv/install.ps1 | iex
```

**Features:**
- Downloads Windows-specific UV binary
- Installs to user-accessible location
- Automatically updates PATH environment variable
- Compatible with PowerShell 5.0+

### Installation Verification

After installation, Angreal automatically verifies:

1. **Binary Availability**: UV command is accessible via PATH
2. **Version Check**: UV responds to `--version` flag
3. **Functionality Test**: Basic UV operations work correctly

## Manual Installation

While automatic installation covers most cases, manual installation may be needed for:

- **Corporate Networks**: Restricted internet access
- **Offline Environments**: No external connectivity
- **Custom Locations**: Non-standard installation paths
- **Specific Versions**: Pinning to particular UV releases

### Manual Installation Methods

#### Via UV Installer (Recommended)
```bash
# Unix/macOS
curl -LsSf https://astral.sh/uv/install.sh | sh

# Windows
irm https://astral.sh/uv/install.ps1 | iex
```

#### Via Package Managers

**Homebrew (macOS/Linux)**:
```bash
brew install uv
```

**Scoop (Windows)**:
```bash
scoop install uv
```

**Cargo (All Platforms)**:
```bash
cargo install uv
```

#### Via Python (pip)
```bash
pip install uv
```

**Note**: The pip installation may be slower than binary installations.

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `UV_INSTALL_DIR` | Custom installation directory | Platform default |
| `UV_NO_PROGRESS` | Disable progress bars | `false` |
| `UV_CACHE_DIR` | UV cache directory | Platform default |

### Custom Installation Paths

If UV is installed in a non-standard location, ensure it's in your PATH:

```bash
# Add to your shell profile (.bashrc, .zshrc, etc.)
export PATH="/custom/path/to/uv:$PATH"
```

## Troubleshooting

### Installation Failures

#### Network Issues
```python
RuntimeError: UV installation failed
```

**Solutions:**
1. Check internet connectivity
2. Verify firewall/proxy settings allow HTTPS downloads
3. Try manual installation
4. Use alternative package managers

#### Permission Issues
```bash
Permission denied: Cannot install UV
```

**Solutions:**
1. Ensure write permissions to installation directory
2. Install to user directory (default behavior)
3. Use `sudo` for system-wide installation (not recommended)

#### Path Issues
```bash
UV binary not found after installation
```

**Solutions:**
1. Restart terminal/shell session
2. Manually add UV location to PATH
3. Verify installation directory is correct

### Version Conflicts

#### Multiple UV Installations
If multiple UV installations exist:

```python
from angreal.integrations.venv import VirtualEnv

# Check which UV version is being used
version = VirtualEnv.version()
print(f"Using UV version: {version}")
```

**Resolution:**
1. Use `which uv` (Unix) or `where uv` (Windows) to find active binary
2. Remove conflicting installations
3. Ensure desired UV binary is first in PATH

#### Outdated UV Version
Angreal works with any modern UV version, but newer versions offer better performance:

```bash
# Update UV manually
curl -LsSf https://astral.sh/uv/install.sh | sh  # Unix/macOS
irm https://astral.sh/uv/install.ps1 | iex       # Windows
```

## Performance Optimization

### Cache Configuration

UV uses caching for optimal performance. Configure cache settings:

```bash
# Set custom cache directory
export UV_CACHE_DIR="/path/to/large/disk/cache"

# Cache size will grow over time - monitor disk usage
du -sh $UV_CACHE_DIR
```

### Network Optimization

For faster package downloads:

```bash
# Use UV's built-in parallel downloads (default)
# No configuration needed - UV automatically optimizes

# For corporate networks, configure proxy if needed
export HTTP_PROXY=http://proxy.company.com:8080
export HTTPS_PROXY=http://proxy.company.com:8080
```

## Security Considerations

### Download Security
- All UV downloads use HTTPS encryption
- Official installation scripts are cryptographically signed
- UV binaries are verified during installation

### Subprocess Security
- Angreal uses secure subprocess calls (no shell injection vulnerabilities)
- UV binary path is validated before execution
- All UV operations use proper argument arrays

### Corporate Environments

For enhanced security in corporate environments:

1. **Pre-install UV**: Include in base system images
2. **Verify Checksums**: Validate UV binary integrity
3. **Network Restrictions**: Allow astral.sh domain for updates
4. **Audit Logs**: Monitor UV operations if required

## Integration Examples

### Checking UV Status

```python
from angreal.integrations.venv import VirtualEnv
from angreal import ensure_uv_installed, uv_version

def check_uv_status():
    """Check UV installation and version."""
    try:
        # Ensure UV is available
        ensure_uv_installed()

        # Get version information
        version = uv_version()
        print(f"✅ UV is available: {version}")

        # Test basic functionality
        pythons = VirtualEnv.discover_available_pythons()
        print(f"✅ UV discovered {len(pythons)} Python installations")

        return True
    except Exception as e:
        print(f"❌ UV issue: {e}")
        return False

# Usage
if check_uv_status():
    print("UV is ready for use!")
```

### Custom Installation Logic

```python
import subprocess
import sys
from pathlib import Path

def setup_uv_custom():
    """Custom UV setup with error handling."""
    uv_path = Path.home() / ".local" / "bin" / "uv"

    if not uv_path.exists():
        print("Installing UV...")
        try:
            if sys.platform == "win32":
                # Windows installation
                subprocess.run([
                    "powershell", "-Command",
                    "irm https://astral.sh/uv/install.ps1 | iex"
                ], check=True)
            else:
                # Unix/macOS installation
                subprocess.run([
                    "bash", "-c",
                    "curl -LsSf https://astral.sh/uv/install.sh | sh"
                ], check=True)
            print("✅ UV installed successfully")
        except subprocess.CalledProcessError:
            print("❌ UV installation failed")
            return False
    else:
        print("✅ UV already installed")

    return True

# Usage in Angreal tasks
import angreal

@angreal.command(name="setup", about="Set up project with UV")
def setup_project():
    """Set up project ensuring UV is available."""
    if setup_uv_custom():
        # Proceed with UV-powered operations
        from angreal.integrations.venv import VirtualEnv
        venv = VirtualEnv(".venv", requirements="requirements.txt")
        print("✅ Project environment ready")
    else:
        print("❌ Setup failed - UV not available")
        sys.exit(1)
```

## Monitoring and Maintenance

### Health Checks

Include UV health checks in your project setup:

```python
import angreal
from angreal.integrations.venv import VirtualEnv

@angreal.command(name="health", about="Check system health")
def health_check():
    """Comprehensive system health check."""
    checks = []

    # UV availability
    try:
        version = VirtualEnv.version()
        checks.append(f"✅ UV: {version}")
    except Exception as e:
        checks.append(f"❌ UV: {e}")

    # Python discovery
    try:
        pythons = VirtualEnv.discover_available_pythons()
        checks.append(f"✅ Python versions: {len(pythons)} found")
    except Exception as e:
        checks.append(f"❌ Python discovery: {e}")

    # Environment creation test
    try:
        test_env = VirtualEnv("health-test", now=False)
        if not test_env.exists:
            test_env = VirtualEnv("health-test")
            test_env.install("requests")  # Test package installation
        checks.append("✅ Environment operations: Working")
    except Exception as e:
        checks.append(f"❌ Environment operations: {e}")

    # Print results
    print("System Health Check:")
    for check in checks:
        print(f"  {check}")
```

### Updates and Maintenance

UV updates automatically when using the installation scripts. For maintenance:

```bash
# Check current version
uv --version

# Update to latest version
curl -LsSf https://astral.sh/uv/install.sh | sh  # Unix/macOS
irm https://astral.sh/uv/install.ps1 | iex       # Windows

# Clean UV cache if needed (saves disk space)
uv cache clean
```

## Further Reading

- [UV Integration Architecture](/angreal/explanation/uv_integration_architecture) - Architectural decisions
- [Virtual Environment API](/angreal/reference/python-api/integrations/venv) - Python API reference
- [Working with Virtual Environments](/angreal/how-to-guides/work-with-virtual-environments) - Usage guide
- [UV Documentation](https://docs.astral.sh/uv/) - Official UV documentation
