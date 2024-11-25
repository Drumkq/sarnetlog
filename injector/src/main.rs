use std::{
    env::current_dir, ffi::CString, process::exit, ptr::{null, null_mut}
};

use colored::Colorize;
use windows::{
    core::{PCSTR, PCWSTR},
    Win32::{
        Foundation::CloseHandle,
        System::{
            Diagnostics::{
                Debug::WriteProcessMemory,
                ToolHelp::{
                    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
                    TH32CS_SNAPPROCESS,
                },
            },
            LibraryLoader::{GetModuleHandleA, GetModuleHandleW, GetProcAddress},
            Memory::{VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE},
            Threading::{CreateRemoteThread, OpenProcess, PROCESS_ALL_ACCESS},
        },
    },
};

fn find_game_process() -> u32 {
    unsafe {
        // Safe:
        // Fails only if the specified process is a 64-bit process and the
        // caller is a 32-bit process
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).unwrap();

        let mut process_entry: PROCESSENTRY32W = Default::default();
        process_entry.dwSize = size_of::<PROCESSENTRY32W>() as _;

        let mut proc = Process32FirstW(snapshot, &mut process_entry);
        while proc.is_ok() {
            let mut exe_name = process_entry
                .szExeFile
                .into_iter()
                .rev()
                .skip_while(|&byte| byte == 0)
                .collect::<Vec<u16>>();
            exe_name.reverse();
            exe_name.push(0);

            let process_name = String::from_utf16_lossy(&exe_name);
            if process_name == "Super Animal Royale.exe\0" {
                CloseHandle(snapshot).unwrap(); // Safe. The handle is open, so we can close it safely

                return process_entry.th32ProcessID;
            }

            proc = Process32NextW(snapshot, &mut process_entry);
        }

        CloseHandle(snapshot).unwrap(); // Safe. The handle is open, so we can close it safely

        0
    }
}

fn main() {
    print!("\x1B[2J\x1B[1;1H"); // clear the screen

    println!(
        "{} {}",
        "[info]".bright_black(),
        "looking up for the process".bright_black().bold()
    );

    // Ugly contruction, but I don't know
    // how to better implement getting the process pid
    let pid = (|| -> u32 {
        let mut pid = 0;
        while pid == 0 {
            pid = find_game_process();
        }

        pid
    })();

    println!(
        "{} {} {}",
        "[info]".bright_black(),
        "process found with id:".bright_black().bold(),
        pid
    );

    let process_handle = unsafe {
        let option = OpenProcess(PROCESS_ALL_ACCESS, false, pid);
        if let Err(e) = option {
            println!("{}", e.message().red().bold());
            return;
        }

        option.unwrap()
    };

    let dll_dir = current_dir().unwrap().join("logger.dll");
    let dll_path = dll_dir.to_str().unwrap();

    if let Err(_) = std::fs::exists(dll_dir.to_path_buf()) {
        println!(
            "{} {}",
            "[fatal]".bright_black(),
            "put logger.dll and injector in the same directory"
                .red()
                .bold()
        );
        exit(-1);
    }

    unsafe {
        let kernel32_mod = GetModuleHandleA(PCSTR::from_raw("kernel32\0".as_ptr()))
        .expect(&"failed to get kernel32.dll".red().bold());
        let load_lib = GetProcAddress(kernel32_mod, PCSTR::from_raw("LoadLibraryA\0".as_ptr()))
            .expect(&"failed to get LoadLibraryA".red().bold());

        let stub = VirtualAllocEx(
            process_handle,
            Some(null()),
            dll_path.len(),
            MEM_RESERVE | MEM_COMMIT,
            PAGE_EXECUTE_READWRITE,
        );
        WriteProcessMemory(
            process_handle,
            stub,
            dll_path.as_ptr().cast(),
            dll_path.len(),
            Some(null_mut()),
        )
        .expect(&"failed to write process memory".red().bold());
        CreateRemoteThread(
            process_handle,
            Some(null()),
            0,
            Some(std::mem::transmute(load_lib)),
            Some(stub.cast()),
            0,
            Some(null_mut()),
        )
        .expect(&"failed to create remote thread".red().bold());

        // Safe
        CloseHandle(process_handle).unwrap();
    }

    println!("{}", "injected".green().bold());
}
