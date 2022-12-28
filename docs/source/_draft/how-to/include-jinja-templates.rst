Include Template Variables 
==========================


You may want to protect a template variable from rendering during the initialization process. 
(Say having a Jinja template that is part of the angreal template but shouldn't actually be templated at initialization.)

Within a File
"""""""""""""

.. code-block:: bash

    {% raw %}
      Hello  {{ template.variable }}.
    {% endraw % }

will render as:

.. code-block:: bash

    Hello {{ template.variable }}.


Within a file/directory name
""""""""""""""""""""""""""""

.. code-block:: bash

	
    ├── README.md
    ├── VERSION
    ├── angreal.json
    ├── setup.py
    └── {% raw %}{{ angreal.name }}{% endraw %}


will render as 

.. code-block:: bash
    
	
    ├── README.md
    ├── VERSION
    ├── angreal.json
    ├── setup.py
    └── {{ angreal.name }} 