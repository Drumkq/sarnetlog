use windows::{core::PCWSTR, Win32::System::Console::{
    AllocConsole, FreeConsole, GetConsoleMode, GetStdHandle, SetConsoleMode, SetConsoleTitleW, CONSOLE_MODE, ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_OUTPUT_HANDLE
}};

/// Safe
unsafe fn set_colors() {
    if cfg!(debug_assertions) {
        let out_handle = GetStdHandle(STD_OUTPUT_HANDLE).unwrap();
        let mut console_mode = CONSOLE_MODE(0);
        GetConsoleMode(out_handle, &mut console_mode).unwrap();
        console_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
        SetConsoleMode(out_handle, console_mode).unwrap();
    }
}

/// # Safety
/// Safe as long as no more than 1 console call is attempted
pub unsafe fn init() {
    if cfg!(debug_assertions) {
        AllocConsole().unwrap();
        SetConsoleTitleW(PCWSTR::from_raw("🔥 sarnetlog | by drumkq\0".encode_utf16().collect::<Vec<_>>().as_slice().as_ptr())).unwrap();
        set_colors();
    }
}

/// # Safety
/// Safe
pub unsafe fn destroy() {
    if cfg!(debug_assertions) {
        FreeConsole().unwrap();
    }
}
