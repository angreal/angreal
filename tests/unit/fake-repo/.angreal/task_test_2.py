import angreal

@angreal.command(name='test_2')
@angreal.option('--noun',default='World')
def angreal_cmd(noun):
    """
    This it test_2's docstring
    """
    angreal.echo('Hello {}'.format(noun))