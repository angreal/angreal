import os
import tempfile
import toml
import angreal

def setup_angreal_project(tmp_dir):
    """Helper function to set up a basic angreal project structure"""
    # Create .angreal directory
    angreal_dir = os.path.join(tmp_dir, ".angreal")
    os.makedirs(angreal_dir)

    # Create angreal.toml
    toml_path = os.path.join(angreal_dir, "angreal.toml")
    return angreal_dir, toml_path

def test_get_context():
    """Test retrieving context from .angreal/angreal.toml"""
    # Save current directory
    original_dir = os.getcwd()

    try:
        # Create temporary directory for test
        with tempfile.TemporaryDirectory() as tmp_dir:
            os.chdir(tmp_dir)

            # Set up angreal project
            angreal_dir, toml_path = setup_angreal_project(tmp_dir)

            # Create test data
            context_data = {
                "name": "test_project",
                "version": "1.0.0",
                "description": "Test angreal project",
                "author": "Test Author"
            }

            # Write toml file
            with open(toml_path, "w") as f:
                toml.dump(context_data, f)

            # Test get_context function
            result = angreal.get_context()

            # Verify result
            assert isinstance(result, dict)
            assert result == context_data
            assert result["name"] == "test_project"
            assert result["version"] == "1.0.0"
            assert result["description"] == "Test angreal project"
            assert result["author"] == "Test Author"
    finally:
        # Always return to original directory
        os.chdir(original_dir)

def test_get_context_no_angreal_dir():
    """Test get_context when no .angreal directory exists"""
    # Save current directory
    original_dir = os.getcwd()

    try:
        # Create temporary directory for test
        with tempfile.TemporaryDirectory() as tmp_dir:
            os.chdir(tmp_dir)

            # Test that get_context raises an error when no .angreal directory exists

        context = angreal.get_context()
        assert isinstance(context, dict)
        assert len(context) == 0
        assert context == {}
    finally:
        # Always return to original directory
        os.chdir(original_dir)

def test_get_context_empty_toml():
    """Test get_context with empty angreal.toml file"""
    # Save current directory
    original_dir = os.getcwd()

    try:
        # Create temporary directory for test
        with tempfile.TemporaryDirectory() as tmp_dir:
            os.chdir(tmp_dir)

            # Set up angreal project
            angreal_dir, toml_path = setup_angreal_project(tmp_dir)

            # Create empty toml file
            with open(toml_path, "w") as f:
                f.write("")

            # Test get_context function
            result = angreal.get_context()

            # Verify result is empty dict
            assert isinstance(result, dict)
            assert len(result) == 0
    finally:
        # Always return to original directory
        os.chdir(original_dir)
