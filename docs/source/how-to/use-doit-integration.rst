Using Doit Integration
======================

Angreal includes the ability to interact with `doit` for tasks with dependencies.


To create a simple doit task :

.. code-block:: python

    import angreal
    from angreal.integrations.doit import doit_task



    @angreal.command()
    @doit_task
    def angreal_cmd(number,base,suffix):
        """
        A command to process a file
        """

        def process_file(input,output):
            """
            get the sum of each row
            """
            with open(input,'r') as f :
                with open(output,'a') as f2:
                    for l in f :
                        items = ','.split(l)
                        print(sum(items), file=f2)

        return {
            'actions': [ process_file, input, output ],
            'targets': [ output ],
            'file_deps' : [ input ]
                }


When called this file will:

* check if the target exists, if not run the action
* check if the file dependency has changed, run the action
* if both files exist, and the file dependency(ies) has(have) not changed, do nothing


To create a more complex task list that will check its intermediate dependencies and only update what's required.


.. code-block:: python

    import angreal
    from angreal.integrations.doit import make_doit_task, run_task


    @angreal.command()
    def angreal_cmd():
        """
        run our processing pipeline
        """

        @make_doit_task
        def pre_process_data():
            return {
                'actions' : [process_file],
                'targets' : ['clean_data.csv'],
                'file_deps' : ['raw_data.csv', 'raw_data2.csv']
            }


        @make_doit_task
        def create_linear_regression():
            return{
                'actions' : [build_linear_regression],
                'targets' : ['linear_regression.pkl'],
                'file_deps' : ['clean_data.csv']
            }

        @make_doit_task
        def create_random_forest():
            return{
                'actions' : [build_random_forest],
                'targets' : ['random_forest.pkl'],
                'file_deps' : ['clean_data.csv']
            }

         @make_doit_task
         def create_report():
            return{
                'actions' : [build_report],
                'targets' : ['report.md'],
                'file_deps' : ['random_forest.pkl','linear_regression.pkl']
            }

        run_doit_tasks([pre_process_data,
                        create_linear_regression,
                        create_random_forest,
                        create_report
                        ],['run'])


This angreal command is really just a doit pipeline, it leverages doit to figure out:

* what needs to be run
* the order it needs to be run in
* if the task needs to be run in order to update its outputs