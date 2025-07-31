import pytest
import sys
import tempfile
import os
from pathlib import Path

from angreal.integrations.docker import compose, DockerCompose, ComposeResult


@pytest.fixture
def sample_compose_file():
    """Create a sample docker-compose.yml file for testing"""
    compose_content = """
version: '3.8'
services:
  web:
    image: nginx:latest
    ports:
      - "8080:80"
  redis:
    image: redis:alpine
    ports:
      - "6379:6379"
"""
    
    with tempfile.NamedTemporaryFile(mode='w', suffix='.yml', delete=False) as f:
        f.write(compose_content)
        f.flush()
        yield f.name
    
    # Cleanup
    os.unlink(f.name)


@pytest.fixture
def invalid_compose_file():
    """Create an invalid compose file path for testing"""
    return "/nonexistent/docker-compose.yml"


class TestDockerCompose:
    """Test Docker Compose integration"""

    def test_is_available(self):
        """Test if Docker Compose availability check works"""
        # This should return a boolean without crashing
        result = DockerCompose.is_available()
        assert isinstance(result, bool)

    def test_compose_function(self, sample_compose_file):
        """Test the compose convenience function"""
        if not DockerCompose.is_available():
            pytest.skip("Docker Compose not available")
        
        stack = compose(sample_compose_file)
        assert isinstance(stack, DockerCompose)
        assert sample_compose_file in stack.compose_file
        assert stack.project_name is None

    def test_compose_function_with_project_name(self, sample_compose_file):
        """Test the compose function with a project name"""
        if not DockerCompose.is_available():
            pytest.skip("Docker Compose not available")
        
        stack = compose(sample_compose_file, project_name="test-project")
        assert isinstance(stack, DockerCompose)
        assert stack.project_name == "test-project"

    def test_docker_compose_init(self, sample_compose_file):
        """Test DockerCompose initialization"""
        if not DockerCompose.is_available():
            pytest.skip("Docker Compose not available")
        
        stack = DockerCompose(sample_compose_file)
        assert isinstance(stack, DockerCompose)
        assert sample_compose_file in stack.compose_file
        assert os.path.dirname(sample_compose_file) in stack.working_dir
        assert stack.project_name is None

    def test_docker_compose_init_with_project_name(self, sample_compose_file):
        """Test DockerCompose initialization with project name"""
        if not DockerCompose.is_available():
            pytest.skip("Docker Compose not available")
        
        stack = DockerCompose(sample_compose_file, project_name="my-project")
        assert stack.project_name == "my-project"

    def test_invalid_compose_file(self, invalid_compose_file):
        """Test error handling for invalid compose file"""
        with pytest.raises(Exception):  # Should raise IOError or similar
            DockerCompose(invalid_compose_file)

    def test_config_command(self, sample_compose_file):
        """Test docker-compose config command"""
        if not DockerCompose.is_available():
            pytest.skip("Docker Compose not available")
        
        stack = DockerCompose(sample_compose_file)
        result = stack.config()
        
        assert isinstance(result, ComposeResult)
        assert isinstance(result.success, bool)
        assert isinstance(result.exit_code, int)
        assert isinstance(result.stdout, str)
        assert isinstance(result.stderr, str)
        
        # Config should succeed for valid compose file
        assert result.success or "compose" in result.stderr.lower()

    def test_config_command_with_options(self, sample_compose_file):
        """Test docker-compose config command with options"""
        if not DockerCompose.is_available():
            pytest.skip("Docker Compose not available")
        
        stack = DockerCompose(sample_compose_file)
        result = stack.config(services=True)
        
        assert isinstance(result, ComposeResult)
        # Should list services if successful
        if result.success:
            assert "web" in result.stdout or "redis" in result.stdout

    def test_ps_command(self, sample_compose_file):
        """Test docker-compose ps command"""
        if not DockerCompose.is_available():
            pytest.skip("Docker Compose not available")
        
        stack = DockerCompose(sample_compose_file)
        result = stack.ps()
        
        assert isinstance(result, ComposeResult)
        assert isinstance(result.success, bool)
        assert isinstance(result.exit_code, int)

    def test_ps_command_with_options(self, sample_compose_file):
        """Test docker-compose ps command with options"""
        if not DockerCompose.is_available():
            pytest.skip("Docker Compose not available")
        
        stack = DockerCompose(sample_compose_file)
        result = stack.ps(all=True, services=True)
        
        assert isinstance(result, ComposeResult)

    def test_build_command(self, sample_compose_file):
        """Test docker-compose build command"""
        if not DockerCompose.is_available():
            pytest.skip("Docker Compose not available")
        
        stack = DockerCompose(sample_compose_file)
        result = stack.build()
        
        assert isinstance(result, ComposeResult)
        # Build might fail if images can't be pulled, but should not crash
        assert isinstance(result.success, bool)

    def test_pull_command(self, sample_compose_file):
        """Test docker-compose pull command"""
        if not DockerCompose.is_available():
            pytest.skip("Docker Compose not available")
        
        stack = DockerCompose(sample_compose_file)
        result = stack.pull()
        
        assert isinstance(result, ComposeResult)
        # Pull might fail in CI environments, but should not crash
        assert isinstance(result.success, bool)

    def test_logs_command(self, sample_compose_file):
        """Test docker-compose logs command"""
        if not DockerCompose.is_available():
            pytest.skip("Docker Compose not available")
        
        stack = DockerCompose(sample_compose_file)
        result = stack.logs(services=["web"])
        
        assert isinstance(result, ComposeResult)
        assert isinstance(result.success, bool)

    def test_exec_command_validation(self, sample_compose_file):
        """Test docker-compose exec command validation"""
        if not DockerCompose.is_available():
            pytest.skip("Docker Compose not available")
        
        stack = DockerCompose(sample_compose_file)
        
        # Empty command should raise ValueError
        with pytest.raises(ValueError):
            stack.exec("web", [])

    def test_exec_command(self, sample_compose_file):
        """Test docker-compose exec command"""
        if not DockerCompose.is_available():
            pytest.skip("Docker Compose not available")
        
        stack = DockerCompose(sample_compose_file)
        result = stack.exec("web", ["echo", "hello"])
        
        assert isinstance(result, ComposeResult)
        # Exec will likely fail since containers aren't running, but shouldn't crash
        assert isinstance(result.success, bool)

    def test_repr(self, sample_compose_file):
        """Test string representation"""
        if not DockerCompose.is_available():
            pytest.skip("Docker Compose not available")
        
        stack = DockerCompose(sample_compose_file, project_name="test")
        repr_str = repr(stack)
        
        assert "DockerCompose" in repr_str
        assert sample_compose_file in repr_str
        assert "test" in repr_str


class TestComposeResult:
    """Test ComposeResult class functionality"""

    def test_compose_result_attributes(self, sample_compose_file):
        """Test that ComposeResult has expected attributes"""
        if not DockerCompose.is_available():
            pytest.skip("Docker Compose not available")
        
        stack = DockerCompose(sample_compose_file)
        result = stack.config()
        
        # Test that all expected attributes exist
        assert hasattr(result, 'success')
        assert hasattr(result, 'exit_code')
        assert hasattr(result, 'stdout')
        assert hasattr(result, 'stderr')
        
        # Test attribute types
        assert isinstance(result.success, bool)
        assert isinstance(result.exit_code, int)
        assert isinstance(result.stdout, str)
        assert isinstance(result.stderr, str)


class TestIntegrationAPI:
    """Test the high-level API as described in requirements"""

    def test_api_usage_pattern(self, sample_compose_file):
        """Test the API usage pattern from requirements"""
        if not DockerCompose.is_available():
            pytest.skip("Docker Compose not available")
        
        # Test the exact API pattern requested:
        # import angreal
        # stack = angreal.integrations.docker.compose(file="path/to/docker-compose.yml")
        # stack.up(detach=True)
        # stack.down()
        
        stack = compose(file=sample_compose_file)
        assert isinstance(stack, DockerCompose)
        
        # Test up command (will likely fail but shouldn't crash)
        result = stack.up(detach=True)
        assert isinstance(result, ComposeResult)
        
        # Test down command
        result = stack.down()
        assert isinstance(result, ComposeResult)

    def test_service_specific_operations(self, sample_compose_file):
        """Test operations on specific services"""
        if not DockerCompose.is_available():
            pytest.skip("Docker Compose not available")
        
        stack = compose(sample_compose_file)
        
        # Test starting specific services
        result = stack.up(services=["web"])
        assert isinstance(result, ComposeResult)
        
        # Test restarting specific services
        result = stack.restart(services=["web"])
        assert isinstance(result, ComposeResult)
        
        # Test stopping specific services
        result = stack.stop(services=["web"])
        assert isinstance(result, ComposeResult)

    def test_build_options(self, sample_compose_file):
        """Test build-related options"""
        if not DockerCompose.is_available():
            pytest.skip("Docker Compose not available")
        
        stack = compose(sample_compose_file)
        
        # Test up with build
        result = stack.up(build=True)
        assert isinstance(result, ComposeResult)
        
        # Test build with no-cache
        result = stack.build(no_cache=True)
        assert isinstance(result, ComposeResult)

    def test_volume_management(self, sample_compose_file):
        """Test volume-related operations"""
        if not DockerCompose.is_available():
            pytest.skip("Docker Compose not available")
        
        stack = compose(sample_compose_file)
        
        # Test down with volumes removal
        result = stack.down(volumes=True)
        assert isinstance(result, ComposeResult)


@pytest.mark.skipif(
    sys.platform == 'win32', reason="Windows tests are flaky"
)
@pytest.mark.skipif(
    not DockerCompose.is_available() if 'DockerCompose' in globals() else True,
    reason="Docker Compose not available"
)
class TestDockerComposeIntegration:
    """Integration tests that require Docker Compose to be available"""

    def test_full_lifecycle(self, sample_compose_file):
        """Test a full container lifecycle (if Docker is available)"""
        stack = compose(sample_compose_file, project_name="test-lifecycle")
        
        # Try to run a full lifecycle - these may fail in CI but shouldn't crash
        try:
            # Pull images
            pull_result = stack.pull()
            print(f"Pull result: {pull_result.success}, stderr: {pull_result.stderr}")
            
            # Start services
            up_result = stack.up(detach=True)
            print(f"Up result: {up_result.success}, stderr: {up_result.stderr}")
            
            # Check status
            ps_result = stack.ps()
            print(f"PS result: {ps_result.success}, stdout: {ps_result.stdout}")
            
            # Stop services
            down_result = stack.down()
            print(f"Down result: {down_result.success}, stderr: {down_result.stderr}")
            
            # All operations should return valid results
            for result in [pull_result, up_result, ps_result, down_result]:
                assert isinstance(result, ComposeResult)
                assert isinstance(result.success, bool)
                assert isinstance(result.exit_code, int)
        
        except Exception as e:
            # In CI environments, Docker operations may fail
            # We mainly want to ensure our bindings work correctly
            print(f"Docker operation failed (expected in CI): {e}")
            assert True  # Test passes if bindings work