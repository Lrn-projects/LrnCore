use std::{env, fs};

pub fn get_current_path() -> String {
    let path = env::current_dir().expect("Failed to get current directory");
    return path.display().to_string();
}

pub fn change_work_dir(dir: &str) {
    env::set_current_dir(&dir).expect("Failed to change directory");
}

pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
