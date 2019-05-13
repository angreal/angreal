import angreal

@angreal.command()
@angreal.option('--noun',default='World')
def angreal_cmd(noun):
    """
    This it test_2's docstring
    """
    angreal.echo('Hello {}'.format(noun))