How to Include Jinja Templates
==============================

You may want to include files in your template that is (or at least looks like) a Jinja template. On initialization,
cookie cutter will attempt to fill in template variables immediately.

To protect any portion of text in your angreal template from being edited wrap it in Jinja's `raw` tags.


The marked up text :

.. code-block:: bash

    {% raw %}
      Hello  {{ template.variable }}.
    {% endraw % }

will render as:

.. code-block:: bash

    Hello {{ template.variable }}.
