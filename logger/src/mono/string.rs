use std::ffi::c_char;

use crate::hooks;

#[repr(C)]
pub struct MonoString {
    pad_0000: [c_char; 16],
    pub length: i32,
    pub first_char: u16, // char16_t
}

impl MonoString {
    pub fn new(s: &str) -> &Self {
        let mut a = s.encode_utf16().collect::<Vec<_>>();
        if !s.ends_with('\0') {
            a.push(0);
        }
        
        return unsafe { &*hooks::monostring_create::invoke(a.as_slice().as_ptr()) };
    }

    pub fn to_string(this: *const Self) -> String {
        let utf16 = unsafe { std::slice::from_raw_parts(&(*this).first_char, (*this).length as _) };
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