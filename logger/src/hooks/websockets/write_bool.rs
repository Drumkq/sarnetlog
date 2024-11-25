use std::ffi::c_void;

use crate::mono::{MethodInfo, NetBuffer};

use super::WS_MESSAGE;

type WriteBool = fn(&NetBuffer, bool, *const MethodInfo);
static mut WRITE_BOOL_FN: Option<WriteBool> = None;

pub unsafe fn hook(target: *mut c_void) {
    WRITE_BOOL_FN = Some(std::mem::transmute(
        minhook::MinHook::create_hook(target, write_bool_hook as _)
            .expect("WriteUInt32 hook failed"),
    ));
}

fn write_bool_hook(this: &NetBuffer, value: bool, mi: *const MethodInfo) {
    {
        let mut msg = WS_MESSAGE.lock().unwrap();
        msg.push_str(&format!("\t{}\n", value.to_string()));
    }

    (unsafe { WRITE_BOOL_FN.unwrap() })(this, value, mi)
}
