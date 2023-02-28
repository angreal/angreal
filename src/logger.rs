//! Logging for the core application
//!
use log4rs::append::console::ConsoleAppender;
use log4rs::append::console::Target;
use log4rs::config::{Appender, Config, Root};
use log4rs::Handle;

/// initializes the angreal logger instance
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

/// updates the verbosity of the logger after initialization
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
