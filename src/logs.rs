use colored::{ColoredString, Colorize};

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

/// The function `error_log` prints an error message with a red "[ERROR]" tag.
///
/// Arguments:
///
/// * `msg`: The `msg` parameter is a reference to a string slice (`&str`) which contains the error
/// message that you want to log.
pub fn error_log(msg: &str) {
    let info = "[ERROR]".truecolor(255, 0, 0);
    println!("{} {}", info, msg);
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
    println!("{} {} {}", info, msg, error);
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
    let log: ColoredString = format!("[{}]", logger).truecolor(255, 94, 0);
    println!("{} {}", log, msg);
}
