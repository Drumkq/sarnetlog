use std::ffi::c_void;

use crate::mono::{MethodInfo, NetBuffer};

use super::WS_MESSAGE;

type WriteFloat = fn(&NetBuffer, f32, *const MethodInfo);
static mut WRITE_FLOAT_FN: Option<WriteFloat> = None;

pub unsafe fn hook(target: *mut c_void) {
    WRITE_FLOAT_FN = Some(std::mem::transmute(
        minhook::MinHook::create_hook(target, write_float_hook as _)
            .expect("WriteUInt32 hook failed"),
    ));
}

fn write_float_hook(this: &NetBuffer, value: f32, mi: *const MethodInfo) {
    {
        let mut msg = WS_MESSAGE.lock().unwrap();
        msg.push_str(&format!("\t{}\n", value.to_string()));
    }

    (unsafe { WRITE_FLOAT_FN.unwrap() })(this, value, mi)
}
