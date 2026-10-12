---
title: "angreal.py_logger"
description: "A unified logger to bridge python and rust"
sidebar:
  order: 3
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>


A unified logger to bridge python and rust

## Functions

### <span id="fn.host_log"></span>`host_log`

```python
def host_log(record:  Bound < '_ , PyAny >) -> PyResult < () >
```

> **Rust Implementation**: [angreal::py_logger::host_log](../../rust/angreal/py_logger/#fn.host_log)

Consume a Python `logging.LogRecord` and emit a Rust `Log` instead.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `record` | ` Bound < '_ , PyAny >` |  |


<details>
<summary>Source</summary>

```python
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
