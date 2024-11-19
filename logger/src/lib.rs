use std::{
    ffi::c_void,
    sync::atomic::{AtomicBool, Ordering},
};

use windows::Win32::{
    Foundation::{CloseHandle, HMODULE},
    System::{
        LibraryLoader::FreeLibraryAndExitThread,
        SystemServices::DLL_PROCESS_ATTACH,
        Threading::{CreateThread, THREAD_CREATION_FLAGS},
    },
};

mod hooks;
pub mod memory;
pub mod mono;
pub mod utils;

pub static EXIT_REQUESTED: AtomicBool = AtomicBool::new(false);

#[no_mangle]
pub unsafe extern "system" fn entry(dll_module: *mut c_void) -> u32 {
    hooks::load();

    while !EXIT_REQUESTED.load(Ordering::Relaxed) {}

    hooks::unload();

    FreeLibraryAndExitThread(std::mem::transmute::<*mut c_void, HMODULE>(dll_module), 0);
}

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn DllMain(
    dll_module: HMODULE,
    call_reason: u32,
    _reserved: *mut c_void,
) -> u32 {
    if call_reason == DLL_PROCESS_ATTACH {
        let handle = CreateThread(
            None,
            0usize,
            Some(entry),
            Some(std::mem::transmute(dll_module)),
            THREAD_CREATION_FLAGS(0),
            None,
        )
        .expect("Failed to create a thread");

        CloseHandle(handle).expect("Failed to close thread handle");
    }

    1
}
