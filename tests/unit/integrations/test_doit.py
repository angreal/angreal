import click
from click.testing import CliRunner
import os
import tempfile



from doit.task import Task
from angreal.integrations.doit import doit_task, run_doit_tasks, make_doit_task






def test_make_doit_task():
    """
    test make doit task
    """

    @make_doit_task
    def echo():
        return { 'actions' : []}

    task = echo()
    assert isinstance(task, Task)

def test_run_doit_tasks():
    """
    run a doit task pipeline
    """

    @make_doit_task
    def echo1():
        return { 'actions' : []}

    @make_doit_task
    def echo2():
        return { 'actions' : []}

    run_doit_tasks([echo1,echo2()],['run'])


def test_doit_task():
    """
    test doit task
    """

    @doit_task
    def echo1():
        return { 'actions' : ['echo YAY > test.txt'] }

    echo1()

    assert os.path.exists('test.txt')
    os.unlink('test.txt')

def test_doit_with_click():
    """
    test doit from within click
    """

    tmp_file_name = os.path.join(os.path.dirname(__file__),'test.txt')
    @click.command()
    @click.option('--foo',default='bar')
    @doit_task
    def echo(foo):
        return { 'actions' : ['echo {} > {}'.format(foo,tmp_file_name)]}



    runner = CliRunner()
    result = runner.invoke(echo)
    assert result.exit_code == 0

    print(os.listdir(os.path.dirname(__file__)))
    file_result = open(tmp_file_name,'r').read().strip()
    assert file_result == 'bar'
    os.unlink(tmp_file_name)


    result = runner.invoke(echo,['--foo','baz'])
    assert result.exit_code == 0

    file_result = open(tmp_file_name,'r').read().strip()
    assert file_result == 'baz'
    os.unlink(tmp_file_name)



