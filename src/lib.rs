pub mod logs;
pub mod macros;
pub mod path;
pub mod usage_exit;

// fn main() {
//     println!("Here's the Core crates for all Lrn project! Here's some sample of features:");
//     println!("[LOGS] example of the logs module");
//     logs::error_log("Error log sample");
//     logs::info_log("Info log sample");
//     logs::error_log_with_code(
//         "Error log with code sample",
//         "Enter the code you want, like a error returned from a function",
//     );
//     logs::lrn_log("Lrn", "Lrn log sample");

//     // command and exist usage

//     //     usage_exit::command_usage(
//     //         "
//     //     Usage: example command [options]

//     //     Commands:
//     //         run             Run the process
//     //         help            Show this help message

//     //     Options:

//     //         -h, --help      Show command usage
//     //         -v, --version   Show the current version",
//     //     );
//     //     usage_exit::command_and_exit_with_code(
//     //         "
//     // Usage: example command [options]

//     // Commands:
//     //     run             Run the process
//     //     help            Show this help message

//     // Options:

//     //     -h, --help      Show command usage
//     //     -v, --version   Show the current version",
//     //         9,
//     //     );

//     // path
//     println!("print the current path: {}", path::get_current_path());
//     path::change_work_dir("./src");
//     println!(
//         "change working dir... new working dir: {}",
//         path::get_current_path()
//     );

//     println!("check if path exist: {}", path::path_exists("./main.rs"));

//     // macros
//     let _new_vec: Vec<String> = vec_of_strings!("first_string", "second_string");
// }
