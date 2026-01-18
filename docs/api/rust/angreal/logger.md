# logger


Logging for the core application

## Functions

### `fn init_logger`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn init_logger () -> Handle
```

initializes the angreal logger instance

<details>
<summary>Source</summary>

```rust
pub fn init_logger() -> Handle {
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();
    let config = Config::builder()
        .appender(Appender::builder().build("stderr", Box::new(stderr)))
        .build(
            Root::builder()
                .appender("stderr")
                .build(log::LevelFilter::Warn),
        )
        .unwrap();

    log4rs::init_config(config).unwrap()
}
```

</details>



### `fn update_verbosity`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn update_verbosity (log_hndl : & Handle , verbosity : u8)
```

updates the verbosity of the logger after initialization

<details>
<summary>Source</summary>

```rust
pub fn update_verbosity(log_hndl: &Handle, verbosity: u8) {
    let level_trace = match verbosity {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        3.. => log::LevelFilter::Trace,
    };

    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();
    let config = Config::builder()
        .appender(Appender::builder().build("stderr", Box::new(stderr)))
        .build(Root::builder().appender("stderr").build(level_trace))
        .unwrap();

    log_hndl.set_config(config);
}
```

</details>
