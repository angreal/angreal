import angreal
import pytest


def func():
    pass


def test_cmd_init():
    """Test command initialization"""

    cmd = angreal.Command(name="sub_command", func=func)
    assert cmd.name == "sub_command"
    assert cmd.func == func
    assert cmd.about is None
    assert cmd.long_about is None

    cmd = angreal.Command(
        name="sub_command", func=func, about="about", long_about="long_about"
    )
    assert cmd.name == "sub_command"
    assert cmd.func == func
    assert cmd.about == "about"
    assert cmd.long_about == "long_about"

    with pytest.raises(TypeError):
        angreal.Command(name="sub_command")

    with pytest.raises(TypeError):
        angreal.Command(func=func)


def test_arg_init():
    """Test arg initialization"""

    arg = angreal.Arg(name="test_arg", command_name="sub_command")
    assert arg.name == "test_arg"
    assert arg.command_name == "sub_command"
    assert arg.takes_value is True
    assert arg.default_value is None
    assert arg.require_equals is None
    assert arg.multiple_values is None
    assert arg.max_values is None
    assert arg.min_values is None
    assert arg.python_type == "str"
    assert arg.short is None
    assert arg.long is None
    assert arg.long_help is None
    assert arg.help is None
    assert arg.required is None

    arg = angreal.Arg(
        name="test_arg_2",
        command_name="sub_command",
        takes_value=True,
        default_value="thang",
        require_equals=True,
        multiple_values=True,
        max_values=3,
        min_values=4,
        python_type="str",
        short="a",
        long="arg_2",
        long_help="This is a long help message",
        help="This is a help message",
        required=True,
    )

    assert arg.name == "test_arg_2"
    assert arg.command_name == "sub_command"
    assert arg.takes_value is True
    assert arg.default_value == "thang"
    assert arg.require_equals is True
    assert arg.multiple_values is True
    assert arg.max_values == 3
    assert arg.min_values == 4
    assert arg.python_type == "str"
    assert arg.short == "a"
    assert arg.long == "arg_2"
    assert arg.long_help == "This is a long help message"
    assert arg.help == "This is a help message"
    assert arg.required is True

    with pytest.raises(TypeError):
        angreal.Arg()

    with pytest.raises(TypeError):
        angreal.Arg(name="test")

    with pytest.raises(TypeError):
        angreal.Arg(command_name="test")
