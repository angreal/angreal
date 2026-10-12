---
title: "angreal::py_logger"
description: "A unified logger to bridge python and rust"
editUrl: false
sidebar:
  order: 27
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

A unified logger to bridge python and rust

## Functions

### <span id="fn.register"></span><code style="font-size: 0.85em">fn register</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn register()
```

registers the rust logging interface with the python logging interface.

<details>
<summary>Source</summary>

```rust
pub fn register() {
    Python::attach(|py| {
        // Extend the `logging` module to interact with log
        setup_logging(py)
    })
    .unwrap();
}
```

</details>

### <span id="fn.host_log"></span><code style="font-size: 0.85em">fn host_​log</code>

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>

```rust
fn host_log(record: &Bound<'_, PyAny>) -> PyResult<()>
```

Consume a Python `logging.LogRecord` and emit a Rust `Log` instead.

<details>
<summary>Source</summary>

```rust
fn host_log(record: &Bound<'_, PyAny>) -> PyResult<()> {
    let level = record.getattr("levelno")?;
    let message = record.getattr("getMessage")?.call0()?.to_string();
    let pathname = record.getattr("pathname")?.to_string();
    let lineno = record
        .getattr("lineno")?
        .to_string()
        .parse::<u32>()
        .unwrap();
    let _logger_name = record.getattr("name")?.to_string();

    // error
    let error_metadata = if level.ge(40u8)? {
        MetadataBuilder::new()
            .target("angreal")
            .level(Level::Error)
            .build()
    } else if level.ge(30u8)? {
        MetadataBuilder::new()
            .target("angreal")
            .level(Level::Warn)
            .build()
    } else if level.ge(20u8)? {
        MetadataBuilder::new()
            .target("angreal")
            .level(Level::Info)
            .build()
    } else if level.ge(10u8)? {
        MetadataBuilder::new()
            .target("angreal")
            .level(Level::Debug)
            .build()
    } else {
        MetadataBuilder::new()
            .target("angreal")
            .level(Level::Trace)
            .build()
    };

    logger().log(
        &Record::builder()
            .metadata(error_metadata)
            .args(format_args!("{}", &message))
            .line(Some(lineno))
            .file(Some("angreal task"))
            .module_path(Some(&pathname))
            .build(),
    );

    Ok(())
}
```

</details>

### <span id="fn.setup_logging"></span><code style="font-size: 0.85em">fn setup_​logging</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn setup_logging(py: Python) -> PyResult<()>
```

Modifies the Python `logging` module to deliver its log messages to the host `tracing::Subscriber` by default. To achieve this goal, the following changes are made to the module:

- A new builtin function `logging.host_log` transcodes `logging.LogRecord`s to `tracing::Event`s. This function
  is not exported in `logging.__all__`, as it is not intended to be called directly.
- A new class `logging.HostHandler` provides a `logging.Handler` that delivers all records to `host_log`.
- `logging.basicConfig` is changed to use `logging.HostHandler` by default.

Since any call like `logging.warn(...)` sets up logging via `logging.basicConfig`, all log messages are now
delivered to `crate::host_log`, which will send them to `tracing::event!`.

<details>
<summary>Source</summary>

```rust
pub fn setup_logging(py: Python) -> PyResult<()> {
    let logging = py.import("logging")?;

    logging.setattr("host_log", wrap_pyfunction!(host_log, &logging)?)?;

    py.run(
        c"
class HostHandler(Handler):
	def __init__(self, level=0):
		super().__init__(level=level)

	def emit(self, record):
		host_log(record)

oldBasicConfig = basicConfig
def basicConfig(*pargs, **kwargs):
	if 'handlers' not in kwargs:
		kwargs['handlers'] = [HostHandler()]
	return oldBasicConfig(*pargs, **kwargs)
",
        Some(&logging.dict()),
        None,
    )?;

    let all = logging.index()?;
    all.append("HostHandler")?;

    Ok(())
}
```

</details>
