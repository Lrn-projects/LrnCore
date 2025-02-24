mod logs;

fn main() {
    println!("Here's the Core crates for all Lrn project! Here's some sample of features:");
    logs::error_log("Error log sample");
    logs::info_log("Info log sample");
    logs::error_log_with_code(
        "Error log with code sample",
        "Enter the code you want, like a error returned from a function",
    );
    logs::lrn_log("Lrn", "Lrn log sample");
}
