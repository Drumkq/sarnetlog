use std::ffi::c_void;

use windows::{
    core::PCWSTR,
    Win32::System::{
        LibraryLoader::GetModuleHandleW,
        ProcessStatus::{GetModuleInformation, MODULEINFO},
        Threading::GetCurrentProcess,
    },
};

pub struct ModuleInfo {
    pub base: *const c_void,
    pub entry: *const c_void,
    pub size: usize,
}

impl ModuleInfo {
    /// # Safety
    /// Safe, but if the module name is correct
    pub fn new(module: &str) -> Self {
        let module_str = module
            .encode_utf16()
            .chain([0u16])
            .collect::<Vec<u16>>()
            .as_ptr();

        let hmod = unsafe { GetModuleHandleW(PCWSTR(module_str)).unwrap() };
        let mut mi = MODULEINFO::default();

        unsafe {
            GetModuleInformation(GetCurrentProcess(), hmod, &mut mi, size_of_val(&mi) as u32)
                .unwrap()
        };

        let MODULEINFO {
            EntryPoint,
            SizeOfImage,
            lpBaseOfDll,
        } = mi;

        ModuleInfo {
            base: lpBaseOfDll,
            entry: EntryPoint,
            size: SizeOfImage as usize,
        }
    }
}
