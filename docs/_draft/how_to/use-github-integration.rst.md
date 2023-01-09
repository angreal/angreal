---
title: GitHub Integrations
---

The Basics
==========

Access to GitHub\'s API is available throught the class. An access token
is required to interact with GitHub\'s service.

::: {.note}
::: {.admonition-title}
Note
:::

For information on how to get and setup an user authentication token
check out GitHub\'s documentation
[here](https://help.github.com/en/articles/creating-a-personal-access-token-for-the-command-line).
For the sake of this documentation it is assumed that the token is
stored in an environmental variable called [GITHUB\_TOKEN]{.title-ref}.
:::

``` {.sourceCode .python}
from angreal import GitHub
github = GitHub('http://github.com',token=os.environ.get('GITHUB_TOKEN'))
```

After initializing the GitHub object, the repository to interact with
must be either fetched or created. Using the method does require having
the integer id from the remote. If you are creating a template that will
use this, it is suggested that you store the id inside of the for later
retrieval.

``` {.sourceCode .python}
github.get_repo(1)
github.create_repository('new_repo')
```

Once the objects repository has been set, you can begin interacting with
it provided methods in the class. :

-   
-   
-   

::: {.note}
::: {.admonition-title}
Note
:::

NoOp methods

While available the following do nothing as GitHub\'s API does not
support the functionality. They will silently pass if called. - - - -
:::

::: {.warning}
::: {.admonition-title}
Warning
:::

, is absolutely destructive and does not provide any protections, use at
your own risk.
:::

More Advanced Usage
===================

Once the GitHub object is initialized the
:py`remote <angreal.integrations.gh.GitHub.remote>`{.interpreted-text
role="attr"} attribute is available for direct manipulation. This is
simply a binding to a
[PyGithub](https://pygithub.readthedocs.io/en/latest/) object, and as
such can be used as though interacting with that library directly.
