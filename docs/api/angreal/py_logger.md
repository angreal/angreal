# py_logger


A unified logger to bridge python and rust

## Functions

### `host_log`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">host_log</span>(record:  Bound &lt; &#x27;_ , PyAny &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::py_logger::host_log](../rust/angreal/py_logger.md#fn-host_log)

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
