use chrono::Utc;
use colored::{ColoredString, Colorize};

//TODO
// add a struct or enum to select a color

/// The `info_log` function prints an informational message with a green "[INFO]" tag.
///
/// Arguments:
///
/// * `msg`: The `msg` parameter is a reference to a string slice (`&str`) which contains the message to
/// be logged.
pub fn info_log(msg: &str) {
    let info = "[INFO]".truecolor(0, 255, 0);

    println!("{} {}", info, msg);
}

/// The function `time_info_log` logs a message with a timestamp and an info tag in Rust.
///
/// Arguments:
///
/// * `msg`: The `time_info_log` function takes a message `msg` as a parameter, which is a reference to
/// a string (`&str`). This message will be logged along with the current timestamp and an information
/// tag.
pub fn time_info_log(msg: &str) {
    let info = "[INFO]".truecolor(0, 255, 0);
    let now = Utc::now().with_timezone(&chrono::Local);

    println!(
        "{} - {} {}",
        now.format("%Y-%m-%d %H:%M:%S").to_string(),
        info,
        msg
    );
}

/// The function `error_log` prints an error message with a red "[ERROR]" tag.
///
/// Arguments:
///
/// * `msg`: The `msg` parameter is a reference to a string slice (`&str`) which contains the error
/// message that you want to log.
pub fn error_log(msg: &str) {
    let info = "[ERROR]".truecolor(255, 0, 0);

    eprintln!("{} {}", info, msg);
}

/// The function `time_error_log` logs an error message with a timestamp in Rust.
///
/// Arguments:
///
/// * `msg`: The `msg` parameter in the `time_error_log` function is a reference to a string slice
/// (`&str`) that represents the error message to be logged.
pub fn time_error_log(msg: &str) {
    let info = "[ERROR]".truecolor(255, 0, 0);
    let now = Utc::now().with_timezone(&chrono::Local);

    eprintln!(
        "{} - {} {}",
        now.format("%Y-%m-%d %H:%M:%S").to_string(),
        info,
        msg
    );
}

/// The function `error_log_with_code` prints an error message with a specified error code in
/// red color.
///
/// Arguments:
///
/// * `msg`: The `msg` parameter is a reference to a string that represents the message or description
/// of the error that occurred.
/// * `error`: The `error` parameter in the `error_log_with_code` function is a string that represents
/// the specific error message or description associated with the error being logged.
pub fn error_log_with_code(msg: &str, error: &str) {
    let info = "[ERROR]".truecolor(255, 0, 0);
    eprintln!("{} {} {}", info, msg, error);
}

/// The function `lrn_log` prints a message with a colored logger tag.
///
/// Arguments:
///
/// * `logger`: The `logger` parameter is a string that represents the name or identifier of the logger
/// or logging component that is generating the log message.
/// * `msg`: The `msg` parameter in the `lrn_log` function is a string that represents the message that
/// you want to log. It could be any information or data that you want to display in the log along with
/// the logger name.
pub fn lrn_log(logger: &str, msg: &str) {
    let log: ColoredString = format!("[{}]", logger).truecolor(138, 43, 226);
    println!("{} {}", log, msg);
}

/// The function `time_lrn_log` logs a message with a timestamp and colored logger name in Rust.
///
/// Arguments:
///
/// * `logger`: The `logger` parameter is a string that represents the name or identifier of the logger
/// or logging component that is generating the log message.
/// * `msg`: The `msg` parameter in the `time_lrn_log` function is a string that represents the message
/// or information that you want to log. It is the actual content that you want to be included in the
/// log message along with the logger name and timestamp.
pub fn time_lrn_log(logger: &str, msg: &str) {
    let log: ColoredString = format!("[{}]", logger).truecolor(138, 43, 226);
    let now = Utc::now().with_timezone(&chrono::Local);

    println!(
        "{} - {} {}",
        now.format("%Y-%m-%d %H:%M:%S").to_string(),
        log,
        msg
    );
}

/// The function `warning_log` prints a warning message with a colored "[WARNING]" tag.
///
/// Arguments:
///
/// * `msg`: The `msg` parameter is a reference to a string slice (`&str`) that contains the message to
/// be logged as a warning.
pub fn warning_log(msg: &str) {
    let info = "[WARNING]".truecolor(255, 165, 0);

    println!("{} {}", info, msg);
}
