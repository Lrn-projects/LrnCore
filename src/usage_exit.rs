use std::process::exit;

/// The function `command_usage` prints the provided usage information and exits the program.
///
/// Arguments:
///
/// * `usage`: The `command_usage` function takes a string parameter `usage` which represents the usage
/// instructions for a command. This function prints out the usage instructions and then exits the
/// program with a status code of 0.
pub fn command_usage(usage: &str) {
    println!("{}", usage);
    exit(0);
}

/// The function `command_and_exit_with_code` prints a usage message and exits with a specified exit
/// code in Rust.
///
/// Arguments:
///
/// * `usage`: The `usage` parameter is a string that contains information about how to use a command or
/// program. It typically includes details about the available options, arguments, and usage examples.
/// * `code`: The `code` parameter in the function `command_and_exit_with_code` is of type `i32`, which
/// represents a 32-bit signed integer. This code is used to specify the exit status code that the
/// program will return when it exits.
pub fn command_and_exit_with_code(usage: &str, code: i32) {
    println!("{}", usage);
    exit(code);
}

pub fn usage_and_exit(msg: &str, usage: &str) {
    if msg != "" {
        eprintln!("{}", msg);
    }

    println!("{}", usage);

    exit(0);
}
