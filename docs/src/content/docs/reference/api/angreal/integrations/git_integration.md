---
title: "angreal.integrations.git_integration"
description: "Git integration module"
editUrl: false
sidebar:
  order: 6
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>

Git integration module

This will be exposed as angreal.integrations.git in Python

## Classes

### <span id="class.GitException"></span><code style="font-size: 0.85em">class GitException</code>

> **Rust Implementation**: [`angreal::python_bindings::integrations::git::GitException`](../../../rust/angreal/python_bindings/integrations/git/#struct.GitException)

#### Methods

##### <span id="GitException.new"></span><code>new</code>

```python
def new(message: str) -> Self
```

> **Rust Implementation**: [`angreal::python_bindings::integrations::git::GitException::new`](../../../rust/angreal/python_bindings/integrations/git/#GitException.new)

<details>
<summary>Source</summary>

```rust
    fn new(message: String) -> Self {
        Self { message }
    }
```

</details>

##### <span id="GitException.__str__"></span><code>_​_​str_​_​</code>

```python
def __str__() -> str
```

> **Rust Implementation**: [`angreal::python_bindings::integrations::git::GitException::__str__`](../../../rust/angreal/python_bindings/integrations/git/#GitException.__str__)

<details>
<summary>Source</summary>

```rust
    fn __str__(&self) -> String {
        self.message.clone()
    }
```

</details>
