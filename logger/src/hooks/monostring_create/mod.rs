use std::{ffi::c_void, ptr::null};

use crate::mono::MonoString;

type MonoStringCreate = fn(*const c_void, *const u16, *const c_void) -> MonoString;
static mut MONO_STRING_CREATE_FN: Option<MonoStringCreate> = None;

/// # Safety:
/// Safe if **addr** points to a valid function
pub(super) fn init(addr: *const c_void) {
    unsafe {
        MONO_STRING_CREATE_FN = Some(std::mem::transmute(addr));
    };
}

pub fn invoke(ptr: *const u16) -> MonoString {
    unsafe { (MONO_STRING_CREATE_FN.unwrap())(null(), ptr, null()) }
}
