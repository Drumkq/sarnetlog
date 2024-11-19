use std::ffi::c_void;

use crate::mono::MonoString;

type JsonSerializeObject = fn(*const c_void, *const c_void, *const c_void) -> MonoString;
static mut JSON_SERIALIZE_OBJECT_FN: Option<JsonSerializeObject> = None;

pub(super) unsafe fn hook(target: *mut c_void) {
    JSON_SERIALIZE_OBJECT_FN = Some(std::mem::transmute(
        minhook::MinHook::create_hook(target, serialize_object_hook as *mut _)
            .expect("SimpleJsonInstance_SerializeObject hook failed"),
    ));
}

fn serialize_object_hook(
    this: *const c_void,
    json: *const c_void,
    method: *const c_void,
) -> MonoString {
    unsafe { (JSON_SERIALIZE_OBJECT_FN.unwrap())(this, json, method) }
}
