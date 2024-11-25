use std::ffi::c_void;

use crate::mono::{MethodInfo, NetBuffer};

use super::WS_MESSAGE;

type WriteInt8 = fn(&NetBuffer, i8, *const MethodInfo);
static mut WRITE_INT8_FN: Option<WriteInt8> = None;

pub unsafe fn hook(target: *mut c_void) {
    WRITE_INT8_FN = Some(std::mem::transmute(
        minhook::MinHook::create_hook(target, write_int8_hook as _)
            .expect("WriteUInt32 hook failed"),
    ));
}

fn write_int8_hook(this: &NetBuffer, value: i8, mi: *const MethodInfo) {
    {
        let mut msg = WS_MESSAGE.lock().unwrap();
        msg.push_str(&format!("\t{}\n", value.to_string()));
    }

    (unsafe { WRITE_INT8_FN.unwrap() })(this, value, mi)
}
