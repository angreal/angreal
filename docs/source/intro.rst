#############
Introduction
#############



.. admonition:: TL;DR

    **Angreal is meant to :**

    - allow the consistent setup of projects
    - provide consistent methods for interacting with the project

Why?
====

Angreal is an attempt to solve two problems that i was running into in both my personal and professional life as a data
scientist and software developer.


So many tools so little time ...
--------------------------------

I think it's fairto say that when writing and documenting software individuals spend at least as much
time working with tools that make development easier and more efficient as they do writing code. This problem only gets
worse as the size of software grows and you start looking at: issue tracking, version control systems, distribution
channels etc.

**Angreal allows common processes needed within a project (i.e. unit tests, linting, creating new content ) to travel
with the project.**


So many projects doing very similar things ...
----------------------------------------------

As a former bio-informatician ( and current data scientist ), I've spent most of my career jumping between multiple small
software projects that often have very similar structures and methods of interacting with them. Having consistently set
up projects for myself (and team members) made the mental overhead of switching between projects limited to simply thinking
about how the problem changed - not how the project structure had. This software came out of the original solutions I
had for this (mainly a myriad of bash/make/doit files floating around a hard drive).

**Angreal leverages project templating to ensure that projects are set up consistently.**


Partial solutions
-----------------


- `**cookiecutter**`_

I got wind of cookie cutter about a year ago when i first started on working on this problem as a way
to help data scientists create projects that were correct, reproducible, and distributable. Its mature and solves the issue
of templating. **angreal uses cookie cutter as its core for templating.**



- `**click**`_

click is an absolutely fabulous tool for creating command line interfaces in python programs. I've known about it
for a while - just never really sat down with it. I wish I had earlier; it's intuitive powerful and flexible.
**angreal uses click to define project level commands**.

- `**doit**`_

doit is a task management and automation tool. I used it as an alternative to make for creating scientific work flows. I really like it as both a program
and a library. It's a great bit of software and allows itself to be used within other peoples software, only down side is writing doit tasks
isn't necessarily intuitive.
**Initial builds of angreal used doit for loading project commands, there is a plan to reintroduce it at a later date.**



Angreal uses, has used, or plans to use portions of all of these tools to achieve its goals. It would not exist without
these tools so I'd like to thank their creators, maintainers, and contributors here.


.. _**cookiecutter**: https://cookiecutter.readthedocs.io/en/latest/
.. _**click**:  http://click.pocoo.org/
.. _**doit**: http://pydoit.org/