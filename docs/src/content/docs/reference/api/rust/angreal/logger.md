---
title: "angreal::logger"
description: "Logging for the core application"
sidebar:
  order: 29
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>


Logging for the core application

## Functions

### <span id="fn.init_logger"></span>`fn init_logger`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


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



### <span id="fn.update_verbosity"></span>`fn update_verbosity`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


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
