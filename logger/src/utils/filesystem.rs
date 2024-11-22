use std::{
    env::current_dir,
    fs::{create_dir, File, OpenOptions},
    io::Write,
    path::PathBuf,
};

use chrono::Utc;

pub const OUTPUT_JSONS_FILE_NAME: &str = "output_jsons.txt";

pub fn get_logs_path() -> PathBuf {
    let current_dir = current_dir()
        .expect("Failed to get current directory")
        .join("sarnetlog");
    let _ = create_dir(current_dir.clone());

    current_dir
}

pub fn init_files() {
    let current_dir = get_logs_path();

    let _ = File::create(current_dir.clone().join(OUTPUT_JSONS_FILE_NAME));
}

pub fn write_to_file(path: &str, s: &str) {
    let file_path = get_logs_path().join(path);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .truncate(false)
        .open(file_path)
        .unwrap();

    let formatted_output = format!("[{}]\n{}\n\n", Utc::now(), s);
    let _ = file.write(formatted_output.as_bytes());
}
