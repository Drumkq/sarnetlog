use std::ffi::c_void;

use crate::mono::{MethodInfo, NetBuffer};

use super::WS_MESSAGE;

type WriteInt16 = fn(&NetBuffer, i16, *const MethodInfo);
static mut WRITE_INT16_FN: Option<WriteInt16> = None;

pub unsafe fn hook(target: *mut c_void) {
    WRITE_INT16_FN = Some(std::mem::transmute(
        minhook::MinHook::create_hook(target, write_int16_hook as _)
            .expect("WriteUInt32 hook failed"),
    ));
}

fn write_int16_hook(this: &NetBuffer, value: i16, mi: *const MethodInfo) {
    {
        let mut msg = WS_MESSAGE.lock().unwrap();
        msg.push_str(&format!("\t{}\n", value.to_string()));
    }

    (unsafe { WRITE_INT16_FN.unwrap() })(this, value, mi)
}
