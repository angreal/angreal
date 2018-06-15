import angreal


@angreal.command()
@angreal.argument('args')
@angreal.option('--foo',default='bar')
def init(args,foo):
    """
    This is a test init command
    """
    print(foo)
    open('file.test','w')
    return