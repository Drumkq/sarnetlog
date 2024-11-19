use patterns::{JSON_SERIALIZE, MONOSTRING_CREATE};

use crate::{memory::Scanner, utils::ModuleInfo};

pub mod json_serialize;
pub mod monostring_create;
mod patterns;

pub unsafe fn load() {
    let scanner = Scanner::new(ModuleInfo::new("GameAssembly.dll"));

    monostring_create::init(scanner.find(MONOSTRING_CREATE).unwrap());
    json_serialize::hook(scanner.find(JSON_SERIALIZE).unwrap().cast_mut());
}

pub fn unload() {}
