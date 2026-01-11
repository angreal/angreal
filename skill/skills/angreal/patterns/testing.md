# Testing Patterns

How to test angreal tasks effectively.

## Testing Philosophy

Angreal tasks are Python functions. Test them like any other Python code:
- Unit test individual functions
- Integration test full task execution
- Functional test CLI behavior

## Unit Testing Tasks

### Basic Function Testing

```python
# .angreal/task_build.py
import angreal

@angreal.command(name="build", about="Build project")
@angreal.argument(name="release", long="release", is_flag=True, takes_value=False)
def build(release=False):
    mode = "release" if release else "debug"
    # Build logic here
    return {"mode": mode, "success": True}
```

```python
# tests/test_build.py
import sys
sys.path.insert(0, ".angreal")
from task_build import build

def test_build_debug_mode():
    result = build(release=False)
    assert result["mode"] == "debug"
    assert result["success"] is True

def test_build_release_mode():
    result = build(release=True)
    assert result["mode"] == "release"
```

### Mocking External Commands

```python
# .angreal/task_test.py
import subprocess
import angreal

@angreal.command(name="test", about="Run tests")
def run_tests():
    result = subprocess.run(["pytest"], capture_output=True, text=True)
    return result.returncode
```

```python
# tests/test_task_test.py
from unittest.mock import patch, MagicMock
import sys
sys.path.insert(0, ".angreal")
from task_test import run_tests

def test_run_tests_success():
    with patch("subprocess.run") as mock_run:
        mock_run.return_value = MagicMock(returncode=0)
        result = run_tests()
        assert result == 0
        mock_run.assert_called_once_with(["pytest"], capture_output=True, text=True)

def test_run_tests_failure():
    with patch("subprocess.run") as mock_run:
        mock_run.return_value = MagicMock(returncode=1)
        result = run_tests()
        assert result == 1
```

### Testing with Fixtures

```python
# tests/conftest.py
import pytest
import tempfile
import os

@pytest.fixture
def temp_project():
    """Create a temporary project directory."""
    with tempfile.TemporaryDirectory() as tmpdir:
        # Create minimal project structure
        os.makedirs(os.path.join(tmpdir, ".angreal"))
        os.makedirs(os.path.join(tmpdir, "src"))
        yield tmpdir

@pytest.fixture
def mock_root(temp_project, monkeypatch):
    """Mock angreal.get_root() to return temp directory."""
    import angreal
    monkeypatch.setattr(angreal, "get_root", lambda: temp_project)
    return temp_project
```

```python
# tests/test_with_fixtures.py
def test_task_uses_project_root(mock_root):
    from task_files import list_files
    # Task will use mock_root as project root
    result = list_files()
    assert ".angreal" in result
```

## Integration Testing

### Testing Full Task Execution

```python
# tests/test_integration.py
import subprocess

def test_build_command():
    """Test the build command via CLI."""
    result = subprocess.run(
        ["angreal", "build", "--release"],
        capture_output=True,
        text=True,
        cwd="/path/to/project"
    )
    assert result.returncode == 0
    assert "Build succeeded" in result.stdout

def test_test_command():
    """Test the test command via CLI."""
    result = subprocess.run(
        ["angreal", "test", "all"],
        capture_output=True,
        text=True,
        cwd="/path/to/project"
    )
    # Check for expected output patterns
    assert "tests passed" in result.stdout.lower() or result.returncode == 0
```

## Testing Patterns

### Test Task Arguments

```python
def test_argument_validation():
    """Verify argument handling."""
    # Test with valid arguments
    result = my_task(count=5, verbose=True)
    assert result is not None

    # Test with edge cases
    result = my_task(count=0, verbose=False)
    assert result is not None

    # Test with defaults
    result = my_task()
    assert result is not None
```

### Test Error Handling

```python
def test_error_handling():
    """Verify graceful error handling."""
    from task_build import build

    # Mock a failure condition
    with patch("os.path.exists", return_value=False):
        result = build()
        assert result == 1  # Non-zero exit code
```

### Test Output

```python
def test_task_output(capsys):
    """Verify task produces expected output."""
    from task_status import status

    status()

    captured = capsys.readouterr()
    assert "Project Status" in captured.out
    assert "Git branch:" in captured.out
```

## Organizing Tests

### Test Directory Structure

```
my-project/
├── .angreal/
│   ├── task_build.py
│   ├── task_test.py
│   └── task_deploy.py
├── tests/
│   ├── conftest.py          # Shared fixtures
│   ├── test_build.py        # Unit tests for build task
│   ├── test_test.py         # Unit tests for test task
│   ├── test_deploy.py       # Unit tests for deploy task
│   └── integration/
│       └── test_cli.py      # CLI integration tests
└── pytest.ini
```

### pytest.ini Configuration

```ini
[pytest]
testpaths = tests
python_files = test_*.py
python_functions = test_*

# Add .angreal to import path
pythonpath = .angreal

# Markers for test categories
markers =
    unit: Unit tests
    integration: Integration tests
    slow: Slow-running tests
```

## Best Practices

### Do

- Test each task function independently
- Mock external dependencies (file system, network, subprocesses)
- Test both success and failure paths
- Verify output messages match expectations
- Test argument combinations
- Use fixtures for common setup

### Don't

- Skip testing error handling paths
- Depend on external state in unit tests
- Test implementation details instead of behavior
- Leave flaky tests in the suite
- Ignore test failures

## Running Tests

### Via angreal (if test tasks exist)

```bash
angreal test all
angreal test unit
angreal test integration
```

### Via pytest directly

```bash
pytest tests/
pytest tests/test_build.py -v
pytest tests/ -m "not slow"
```
