use patterns::{JSON_SERIALIZE, MONOSTRING_CREATE};

use crate::{memory::Scanner, utils::ModuleInfo};

pub mod json_serialize;
pub mod monostring_create;
mod patterns;

pub unsafe fn load() {
    let scanner = Scanner::new(ModuleInfo::new("GameAssembly.dll"));

    monostring_create::hook(scanner.find(MONOSTRING_CREATE).unwrap());
    json_serialize::hook(scanner.find(JSON_SERIALIZE).unwrap().cast_mut());

    minhook::MinHook::enable_all_hooks().unwrap();
}

pub fn unload() {}
