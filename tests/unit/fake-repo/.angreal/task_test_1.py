import angreal

@angreal.command(name='test_1', short_help='test_1')
def angreal_cmd():
    """
    Test one's doc string
    """
    print('This is a simple test')
