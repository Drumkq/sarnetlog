use std::{ffi::c_void, sync::atomic::{AtomicI64, Ordering}};

use colored::Colorize;

use crate::{mono::MonoString, utils::filesystem::{write_to_file, OUTPUT_JSONS_FILE_NAME}};

type JsonSerializeObject<'a> = fn(*const c_void, *const c_void, *const c_void) -> &'a MonoString;
static mut JSON_SERIALIZE_OBJECT_FN: Option<JsonSerializeObject> = None;
static SERIALIZATION_I: AtomicI64 = AtomicI64::new(1);

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
) -> *const MonoString {
    let res = unsafe { &*(JSON_SERIALIZE_OBJECT_FN.unwrap())(this, json, method) };
    let s: String = res.into();

    let js: serde_json::Value = serde_json::from_str(s.as_str()).unwrap();
    let pretty_output = serde_json::to_string_pretty(&js).unwrap();

    println!(
        "{}: {}",
        "[outgoing json]".bright_black().bold(),
        pretty_output.bright_yellow().bold()
    );

    let formatted = format!("Iteration {}\n{}", SERIALIZATION_I.load(Ordering::Relaxed), pretty_output);
    write_to_file(OUTPUT_JSONS_FILE_NAME, &formatted);

    SERIALIZATION_I.store(SERIALIZATION_I.load(Ordering::Relaxed) + 1, Ordering::Relaxed);

    res
}
