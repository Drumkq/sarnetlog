use std::process::exit;

use colored::Colorize;
use windows::{core::PCSTR, Win32::System::LibraryLoader::LoadLibraryA};

fn main() {
    print!("\x1B[2J\x1B[1;1H"); // clear the screen

    let dll_path = "logger.dll\0";
    let os_dll_path = PCSTR::from_raw(dll_path.as_ptr());

    if let Err(e) = std::fs::read_dir(dll_path.to_string()) {
        println!("{} {}", "[fatal]".bright_black(), e.to_string().red().bold());
        exit(-1);
    }

    unsafe {
        if let Err(e) = LoadLibraryA(os_dll_path) {
            println!("{} {}", "[fatal]".bright_black(), e.message().red().bold());
            println!("{}", "successfully injected".green().bold());
        } else {
        }
    }
}
