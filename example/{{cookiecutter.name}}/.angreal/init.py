import angreal

@angreal.command()
@angreal.option('--no_objectives',is_flag=True, help="These meetings are pointless")
def init(no_objectives):
    """
    Initialize your meetings project.
    """

    with open('Introduction.md','w') as f:
        print('Meeting Objectives', file=f)
        if not no_objectives:
            print( input("Describe the objective(s) of this meeting series:\n"), file=f)

    return
