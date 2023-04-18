---
title: Add Arguments to a Task
weight: 20
---

First, all control mechanisms for commands are managed by a single decorator called `@argument`. The full signature for this decorator is:

- __name__: the argument name, must match the provided arg/kwarg in the wrapped python function (_Required_)
- __python_type__: the python type to apply when passing to the wrapped function. Must be one of ("str", "int", "float"), __default__ "str"
- __takes_value__: does the argument consumer a trailing value, __default__ True
- __default_value__: the default value to apply if none is provided, __default__ None
- __require_equals__: the applied value requires an equal sign (i.e. `--arg=value` ), __default__ None
- __multiple_values__: the argument takes multiple values, __default__ None
- __number_of_values__: the argument takes a specific number of values, __default__ None
- __max_values__: the argument takes at most X values, __default__ None
- __min_values__: the argument takes at min X values, __default__ None
- __short__: the short name for the argument, a single character (i.e. `-i` in the CLI would be 'i'), __default__ None
- __long__: the long name for the argument, a single word (i.e. `--information` in the CLI would be 'information'), __default__ None
- __long_help__: the help message to show when a long help message is requested via `--help`, __default__ None
- __help__: the short help message to show during failure or when `-h` is requested, __default__ None
- __required__: whether this argument is required at run time, __default__ None


## Arguments

Arguments are positional after the defined command.

```python
import angreal

@angreal.command(name="echo", about="an echo replacement")
@angreal.argument(name="phrase", help="the phrase to echo", required=True)
def task_echo(phrase):
    print(phrase)
```

```bash
angreal echo --help                                                                                                                           ─╯
echo
an echo replacement

USAGE:
    echo <phrase>

ARGS:
    <phrase>    the phrase to echo

OPTIONS:
    -h, --help    Print help information
```

## Options

An option is usually something that takes an argument in order control command behavior.

```python
import angreal

green = "\33[32m"
red = "\33[31m"
end = "\33[0m"

@angreal.command(name="echo", about="an echo replacement")
@angreal.argument(name="phrase", help="the phrase to echo", required=True)
@angreal.argument(name="color", long="color", short='c', help="apply a color to the echo phrase")
def task_echo(phrase,color=None):

    if color=="red":
        print(red + phrase + end )
        return
    if color=="green":
        print(green + phrase + end )
        return
    print(phrase)
```

```bash
$ angreal echo --help                                                                                                                           ─╯

echo
an echo replacement

USAGE:
    echo [OPTIONS] <phrase>

ARGS:
    <phrase>    the phrase to echo

OPTIONS:
    -c, --color <color>    apply a color to the echo phrase
    -h, --help             Print help information
```

## Flags

A flag is just a binary value that will set a resulting value to True without taking a value.

```python
import angreal

green = "\33[32m"
red = "\33[31m"
end = "\33[0m"

@angreal.command(name="echo", about="an echo replacement")
@angreal.argument(name="phrase", help="the phrase to echo", required=True)
@angreal.argument(name="color", long="color", short='c', help="apply a color to the echo phrase")
@angreal.argument(name="yell", long="yell", short='y', takes_value=False, help="yell it from the tree tops")
def task_echo(phrase,color=None,yell=False):

    if yell:
        phrase = phrase.upper() + "! ! !"

    if color=="red":
        print(red + phrase + end )
        return
    if color=="green":
        print(green + phrase + end )
        return
    print(phrase)
```

```bash
$ angreal echo --help                                                                                                                           ─╯

echo
an echo replacement

USAGE:
    echo [OPTIONS] <phrase>

ARGS:
    <phrase>    the phrase to echo

OPTIONS:
    -c, --color <color>    apply a color to the echo phrase
    -h, --help             Print help information
    -y, --yell             yell it from the tree tops
```
