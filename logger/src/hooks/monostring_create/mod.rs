use std::{ffi::c_void, ptr::null};

use crate::mono::MonoString;

type MonoStringCreate<'a> = fn(*const c_void, *const u16, *const c_void) -> &'a MonoString;
static mut MONO_STRING_CREATE_FN: Option<MonoStringCreate> = None;

pub(super) fn hook(addr: *const c_void) {
    unsafe {
        MONO_STRING_CREATE_FN = Some(std::mem::transmute(addr));
    };
}

pub fn invoke<'a>(ptr: *const u16) -> &'a MonoString {
    if unsafe { MONO_STRING_CREATE_FN.is_none() } {
        panic!("MONO_STRING_CHAR_CTOR not initialized");
    }

    (unsafe { MONO_STRING_CREATE_FN.unwrap() })(null(), ptr, null())
}
