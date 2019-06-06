import angreal


@angreal.command()
@angreal.option('--foo',default='bar')
def init(foo):
    """
    This is a test init command
    """
    print(foo)
    open('file.test','w')
    return