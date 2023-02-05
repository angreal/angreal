import angreal
import pytest


def func():
    pass


def test_cmd_init():
    """Test command initialization"""

    cmd = angreal.Command(name="sub_command", func=func)
    assert cmd.name == "sub_command"
    assert cmd.func == func
    assert cmd.about == None
    assert cmd.long_about == None

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
    assert arg.takes_value == True
    assert arg.default_value == None
    assert arg.require_equals == None
    assert arg.multiple_values == None
    assert arg.max_values == None
    assert arg.min_values == None
    assert arg.python_type == "str"
    assert arg.short == None
    assert arg.long == None
    assert arg.long_help == None
    assert arg.help == None
    assert arg.required == None

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
    assert arg.takes_value == True
    assert arg.default_value == "thang"
    assert arg.require_equals == True
    assert arg.multiple_values == True
    assert arg.max_values == 3
    assert arg.min_values == 4
    assert arg.python_type == "str"
    assert arg.short == "a"
    assert arg.long == "arg_2"
    assert arg.long_help == "This is a long help message"
    assert arg.help == "This is a help message"
    assert arg.required == True

    with pytest.raises(TypeError):
        angreal.Arg()

    with pytest.raises(TypeError):
        angreal.Arg(name="test")

    with pytest.raises(TypeError):
        angreal.Arg(command_name="test")
