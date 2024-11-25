use std::ffi::c_void;

use crate::mono::{MethodInfo, NetBuffer};

use super::WS_MESSAGE;

type WriteUInt32Fn = fn(&NetBuffer, u32, *const MethodInfo);
static mut WRITE_UINT32_FN: Option<WriteUInt32Fn> = None;

pub unsafe fn hook(target: *mut c_void) {
    WRITE_UINT32_FN = Some(std::mem::transmute(
        minhook::MinHook::create_hook(target, write_uint32_hook as _)
            .expect("WriteUInt32 hook failed"),
    ));
}

fn write_uint32_hook(this: &NetBuffer, value: u32, mi: *const MethodInfo) {
    {
        let mut msg = WS_MESSAGE.lock().unwrap();
        msg.push_str(&format!("\t{}\n", value.to_string()));
    }

    (unsafe { WRITE_UINT32_FN.unwrap() })(this, value, mi)
}
