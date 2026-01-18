# error_formatter


## Structs

### `struct PythonErrorFormatter`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


Formats Python exception information in a more readable way

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `error` | `PyErr` |  |

#### Methods

##### `new`


```rust
fn new (error : PyErr) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn new(error: PyErr) -> Self {
        Self { error }
    }
```

</details>



##### `format`


```rust
fn format (& self) -> String
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
