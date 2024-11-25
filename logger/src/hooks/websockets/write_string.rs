use std::ffi::c_void;

use crate::mono::{MethodInfo, MonoString, NetBuffer};

use super::WS_MESSAGE;

type WriteString = fn(&NetBuffer, &MonoString, *const MethodInfo);
static mut WRITE_STRING_FN: Option<WriteString> = None;

pub unsafe fn hook(target: *mut c_void) {
    WRITE_STRING_FN = Some(std::mem::transmute(
        minhook::MinHook::create_hook(target, write_string_hook as _)
            .expect("WriteUInt32 hook failed"),
    ));
}

fn write_string_hook(this: &NetBuffer, value: &MonoString, mi: *const MethodInfo) {
    {
        let mut msg = WS_MESSAGE.lock().unwrap();
        msg.push_str(&format!("\t{}\n", MonoString::to_string(value)));
    }

    (unsafe { WRITE_STRING_FN.unwrap() })(this, value, mi)
}
