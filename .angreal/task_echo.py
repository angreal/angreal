import angreal
from angreal.integrations.venv import venv_required



@angreal.command(name="echo", about="an echo replacement")
@angreal.argument(name="phrase", help="the phrase to echo", required=True)
@angreal.argument(name="color", long="color", short='c', help="apply a color to the echo phrase")
@venv_required('__test',requirements='flask')
def task_echo(phrase,color=None):
    import flask
    if color=="red":
        print(red + phrase + end )
        return
    if color=="green":
        print(green + phrase + end )
        return
    print(phrase)
