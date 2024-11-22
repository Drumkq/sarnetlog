use std::ffi::c_char;

use crate::hooks;

/// Copy and Clone are not allowed
#[repr(C)]
pub struct MonoString {
    pad_0000: [c_char; 16],
    pub length: i32,
    pub first_char: u16,
}

impl MonoString {
    /// # Panics
    /// When mono string ctor not found or not initialized
    pub fn new(s: &str) -> &Self {
        let mut utf16 = s.encode_utf16().collect::<Vec<_>>();

        // All C# strings contains null terminator, but in Rust are not null-terminated
        // so we need to append \0 to utf16 encoded string
        if s.ends_with('\0') {
            utf16.push(0);
        }

        return hooks::monostring_create::invoke(utf16.as_ptr());
    }

    /// # Safety
    /// The “this” parameter must point to a valid C# String class
    pub fn to_string(this: &Self) -> String {
        let utf16 = unsafe { std::slice::from_raw_parts(&this.first_char, this.length as _) };
        let res = String::from_utf16(utf16);

        match res {
            Ok(s) => s,
            Err(_) => String::new(),
        }
    }
}

impl<'a> Into<&'a MonoString> for &'a str {
    fn into(self) -> &'a MonoString {
        MonoString::new(self)
    }
}

impl From<&MonoString> for String {
    fn from(value: &MonoString) -> Self {
        MonoString::to_string(value)
    }
}
