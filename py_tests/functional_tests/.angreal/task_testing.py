import angreal



group1 = angreal.command_group(name="group1", about="testing group")


@group1()
@angreal.command(name="flag", about="test a flag argument")
@angreal.argument(name="test", long="test", short="t", takes_value=False, is_flag=True)
def flag_tests(test):
    """
    """
    if test:
        open("group.txt","w").close()


@group1()
@angreal.group(name="group2", about="group2")
@angreal.command(name="flag2", about="test a flag argument")
@angreal.argument(name="test", long="test", short="t", takes_value=False, is_flag=True)
def flag_test2(test):
    """
    """
    if test:
        open("nested_group.txt","w").close()
