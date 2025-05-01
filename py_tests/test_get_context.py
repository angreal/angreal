import os
import tempfile
import angreal



def test_get_context():
    """Test retrieving context from .angreal/angreal.toml"""
    result = angreal.get_context()

    # Verify result
    assert isinstance(result, dict)
    assert result["key1"] == "value1"
    assert result["key2"] == 2


def test_get_context_no_angreal_dir():
    """Test get_context when no .angreal directory exists"""
    # Save current directory
    original_dir = os.getcwd()

    try:
        # Create temporary directory for test
        with tempfile.TemporaryDirectory() as tmp_dir:
            os.chdir(tmp_dir)

            # Test that get_context returns empty dict when no .angreal directory exists
            context = angreal.get_context()
            assert isinstance(context, dict)
            assert len(context) == 0
            assert context == {}
    finally:
        # Always return to original directory
        os.chdir(original_dir)
