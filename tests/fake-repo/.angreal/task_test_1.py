import angreal

@angreal.command(short_help='test_1')
def angreal_cmd():
    """
    Test one's doc string
    :return:
    """
    print('This is a simple test')
