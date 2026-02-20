use std::fmt;

pub enum LogLevel {
    Trace, Info, Warn, Error, Fatal,
}

pub fn log(message: fmt::Arguments<'_>, level: LogLevel, file: &'static str, line: u32) {
    let level_str;
    match level {
        LogLevel::Trace => {
            level_str = "\x1b[44;30m  TRC  \x1b[0m";
        },
        LogLevel::Info => {
            level_str = "\x1b[42;30m  INF  \x1b[0m";
        },
        LogLevel::Warn => {
            level_str = "\x1b[43;30m  WRN  \x1b[0m";
        },
        LogLevel::Error => {
            level_str = "\x1b[41;30m  ERR  \x1b[0m";
        },
        LogLevel::Fatal => {
            level_str = "\x1b[1;41;33m FATAL \x1b[0m";
        },
    };

    let time = chrono::prelude::Utc::now();
    let file = format!("{}:{}", file.split('/').last().unwrap_or("???.rs"), line);
    
    std::print!("\x1b[1;37;100m{time}\x1b[22;3;97m {file:^15} {level_str}│ {message}\n");
}
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{
        $crate::log(format_args!($($arg)*), $crate::LogLevel::Trace, file!(), line!());
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::log(format_args!($($arg)*), $crate::LogLevel::Info, file!(), line!());
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        $crate::log(format_args!($($arg)*), $crate::LogLevel::Warn, file!(), line!());
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::log(format_args!($($arg)*), $crate::LogLevel::Error, file!(), line!());
    }};
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {{
        $crate::log(format_args!($($arg)*), $crate::LogLevel::Fatal, file!(), line!());
    }};
}
