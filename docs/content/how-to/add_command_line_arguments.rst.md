---
title: Add Command Line Arguments
---

The execution of a templates `init.py` and tasks is handled through an
extension of the [click](https://click.palletsprojects.com/en/7.x/)
project, so if you\'re familiar with it you should be pretty well set.

Arguments
=========

To add an argument (something that is required) to either the
:py`init`{.interpreted-text role="func"} or
:py`angreal_cmd`{.interpreted-text role="func"} functions, simply add an
:py`angreal.argument()`{.interpreted-text role="func"} decorator and
pass it into the decorated function.

``` {.sourceCode .python}
import angreal

@angreal.command()
@angreal.argument('name')
def angreal_cmd(name):

    print("Hello {}!".format(name))
    return
```

When executed this task would expect an argument to be passed and would
simply print \"Hello X !\" to the screen.

To add an option (something that is optional) the
:py`init`{.interpreted-text role="func"} or
:py`angreal_cmd`{.interpreted-text role="func"} functions, simply add an
:py`angreal.option()`{.interpreted-text role="func"} decorator and pass
it into the decorated function.

``` {.sourceCode .python}
import angreal

@angreal.command()
@angreal.option('--name',default='world', help="Who to say hello to")
def angreal_cmd(name):

    print("Hello {}!".format(name))
    return
```

When executed, this task could accept an argument. If one isn\'t
provided it would print \"Hello world!\" to the screen.

For a full treatment of what you can do, I\'d strongly recommend reading
:

-   [paramaters](https://click.palletsprojects.com/en/7.x/parameters/)
-   [options](https://click.palletsprojects.com/en/7.x/options/)
-   [arguments](https://click.palletsprojects.com/en/7.x/arguments/)
