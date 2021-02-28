use chrono::Local;
use colored::*;
use lazy_static::lazy_static;
use log::LevelFilter;
use std::io::Write;
use std::mem;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{io, sync::Mutex};
use termcolor::{ColorChoice, StandardStream};

lazy_static! {
    static ref MODULE_PATH: Mutex<String> = Mutex::new("".to_string());
}

fn set_module_path(module_path: &str) {
    MODULE_PATH.lock().unwrap().push_str(module_path);
}

/// Defines level log
pub fn setup_log(level: LevelFilter, module_path: &str) {
    set_module_path(module_path);

    // If terminal support colors
    StandardStream::stdout(if atty::is(atty::Stream::Stdout) {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    });
    log::set_max_level(level);
    set_max_level(level);
    log::set_logger(&Logger).unwrap();
}

lazy_static! {
    static ref MAX_LEVEL: AtomicUsize = AtomicUsize::new(unsafe { mem::transmute(log::LevelFilter::Warn) });
}

pub fn max_level() -> log::LevelFilter {
    unsafe { mem::transmute(MAX_LEVEL.load(Ordering::Relaxed)) }
}

fn set_max_level(level: log::LevelFilter) {
    MAX_LEVEL.store(unsafe { mem::transmute(level) }, Ordering::Relaxed);
}

/// A simple logger
pub struct Logger;

impl Logger {
    fn get_actual_level(&self, metadata: &log::Metadata<'_>) -> log::Level {
        let mut level = metadata.level();
        if level == log::Level::Debug && (metadata.target() == "tokio_reactor" || metadata.target().starts_with("hyper::proto::")) {
            level = log::Level::Trace;
        }
        level
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        self.get_actual_level(metadata) <= max_level()
    }

    fn log(&self, record: &log::Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let level = self.get_actual_level(record.metadata());
        let (level_name, level_color) = match level {
            log::Level::Error => ("ERROR", Color::Red),
            log::Level::Warn => ("WARN ", Color::Yellow),
            log::Level::Info => ("INFO ", Color::Cyan),
            log::Level::Debug => ("DEBUG", Color::Yellow),
            log::Level::Trace => ("TRACE", Color::Magenta),
        };

        let datetime = Local::now();
        let now = datetime.format("%T").to_string();

        let short_target = record.target().split("::").next().unwrap_or("");
        let msg = format!(
            "{} {} {}{}",
            now.dimmed(),
            format!("  {}  ", level_name).on_color(level_color).black(),
            record.args().to_string().replace("\n", " "),
            {
                if short_target != &MODULE_PATH.lock().unwrap()[..] {
                    format!("  (from {})", short_target)
                } else {
                    "".to_string()
                }
            }
            .dimmed(),
        );
        writeln!(io::stdout(), "{}", msg).ok();
    }

    fn flush(&self) {}
}
