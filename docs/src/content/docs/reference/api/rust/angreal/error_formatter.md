---
title: "angreal::error_formatter"
description: "API reference of the Rust module angreal::error_formatter."
editUrl: false
sidebar:
  order: 17
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

## Structs

### <span id="struct.PythonErrorFormatter"></span><code style="font-size: 0.85em">struct PythonErrorFormatter</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

Formats Python exception information in a more readable way

#### Methods

##### <span id="PythonErrorFormatter.new"></span><code>new</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn new(error: PyErr) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn new(error: PyErr) -> Self {
        Self { error }
    }
```

</details>

##### <span id="PythonErrorFormatter.format"></span><code>format</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn format(&self) -> String
```

Formats a Python error in a more readable way

<details>
<summary>Source</summary>

```rust
    pub fn format(&self) -> String {
        let mut output = String::new();

        let error_message = Python::attach(|py| {
            // Get the exception type and value
            let type_obj = self.error.get_type(py);
            let type_name = type_obj
                .name()
                .map(|n| n.to_string())
                .unwrap_or_else(|_| "Unknown".to_string());

            // Extract the error message
            let value = self.error.value(py).to_string();

            format!("\nError: {}\n{}", type_name, value)
        });

        output.push_str(&error_message);
        output.push('\n');

        // Format traceback in a simplified way
        Python::attach(|py| {
            if let Some(traceback) = self.error.traceback(py) {
                output.push_str("\nTraceback:\n");

                // Just extract the traceback as a string
                let tb_str = format!("  {}", traceback);
                for line in tb_str.lines() {
                    output.push_str(&format!("  {}\n", line));
                }
            }
        });

        output
    }
```

</details>
