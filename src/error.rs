use std::{
    error::Error as StdError,
    fmt::{self, Write},
    io::{stdout, Error as IoError},
    panic::PanicInfo,
    result::Result as StdResult,
};

use backtrace::Backtrace;
use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};

use crate::fs::{save_log, LogType};

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConfigError(String),
    OtherError(String),
    IoError(IoError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OtherError(s) | Self::ConfigError(s) => f.write_str(s),
            Self::IoError(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        match self {
            Self::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::IoError(e)
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Self::OtherError(msg.to_string())
    }
}

// This was heavily inspired by https://crates.io/crates/human-panic
pub fn handle_panic(info: &PanicInfo) {
    disable_raw_mode().expect("Unable to disable raw mode.");
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)
        .expect("Unable to restore screen.");

    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let repo = env!("CARGO_PKG_REPOSITORY");

    let os_info = os_info::get().to_string();

    let trace = get_trace();
    let panic_message = info
        .message()
        .map(|m| format!("{}", m))
        .map_or_else(|| "Unknown".to_string(), |msg| msg);

    let mut buffer = String::new();

    let _ = write!(
        buffer,
        "Version: {}\nSystem: {}\nCause: {}\n\nTrace: \n{}",
        version, os_info, panic_message, trace
    );

    let path = match save_log(LogType::Crash, buffer) {
        Ok(path) => format!("A trace log has been saved to '{}'.", path),
        Err(why) => format!("Error creating log file: {}", why),
    };

    println!(
        "Oh, no! It seems like {} has crashed, this is kind of \
         embarrassing.\n\n{}\n\nDue to the nature of {}, we respect your \
         privacy and as such\ncrash reports are never sent automatically. If \
         you would like\nto help up diagnose this issue, please submit an \
         issue at \n{}/issues/new?assignees=L3afMe&labels=&\
         template=panic-report.md&title=[BUG]{}\n\nThe report contains some \
         basic information about your system\nlike the OS and arch type, this \
         can help with diagnosing what\nwent wrong, if you don't want this to \
         be sent feel free to\nremove it before submitting.\n",
        name,
        path,
        name,
        repo,
        urlencoding::encode(&panic_message),
    );
}

fn get_trace() -> String {
    // https://github.com/rust-cli/human-panic/blob/master/src/report.rs#L47-L51
    const SKIP_FRAMES_NUM: usize = 8;
    const HEX_WIDTH: usize = std::mem::size_of::<usize>() + 2;

    let mut trace = String::new();

    for (idx, frame) in Backtrace::new()
        .frames()
        .iter()
        .skip(SKIP_FRAMES_NUM)
        .enumerate()
    {
        let newline = if idx == 0 { "" } else { "\n" };
        let ip = frame.ip();
        let _ = write!(trace, "{}{:3}: {:3$?}", newline, idx, ip, HEX_WIDTH);

        let symbols = frame.symbols();
        if symbols.is_empty() {
            let _ = write!(trace, " - <unresolved>");
            continue;
        }

        for (idx, symbol) in symbols.iter().enumerate() {
            if idx != 0 {
                let _ = write!(trace, "\n    ");
            }

            let name = symbol.name().map_or_else(
                || "<unknown>".to_string(),
                |symbol| symbol.to_string(),
            );
            let _ = write!(trace, " - {}", name);

            if let (Some(file), Some(line), Some(col)) =
                (symbol.filename(), symbol.lineno(), symbol.colno())
            {
                let _ = write!(
                    trace,
                    "\n    at {}:{}:{}",
                    file.display(),
                    line,
                    col,
                );
            }
        }
    }

    trace
}
