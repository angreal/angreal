---
title: GitLab Integrations
---

The Basics
==========

Access to GitLab\'s API is available throught the class. An access token
is required to interact with GitLab\'s service.

::: {.note}
::: {.admonition-title}
Note
:::

For information on how to get and setup an user authentication token
check out Gitlab\'s documentation
[here](https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html).
For the sake of this documentation it is assumed that the token is
stored in an environmental variable called [GITLAB\_TOKEN]{.title-ref}.
:::

``` {.sourceCode .python}
from angreal import GitLab
gitlab = GitLab('http://gitlab.com',token=os.environ.get('GITLAB_TOKEN'))
```

After initializing the GitLab object, the repository to interact with
must be either fetched or created. Using the method does require having
the integer id from the remote. If you are creating a template that will
use this, it is suggested that you store the id inside of the for later
retrieval.

``` {.sourceCode .python}
gitlab.get_repo(1)
gitlab.create_repository('new_repo')
```

Once the objects repository has been set, you can begin interacting with
it provided methods in the class. :

-   
-   
-   
-   
-   
-   
-   

::: {.warning}
::: {.admonition-title}
Warning
:::

, is absolutely destructive and does not provide any protections, use at
your own risk.
:::

More Advanced Usage
===================

Once the GitLab object is initialized the
:py`remote <angreal.integrations.gl.GitLab.remote>`{.interpreted-text
role="attr"} attribute is available for direct manipulation. This is
simply a binding to a
[python-gitlab](https://python-gitlab.readthedocs.io/en/latest/api-usage.html)
object, and as such can be used as though interacting with that library
directly.
