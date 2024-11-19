use crate::hooks;

#[repr(C)]
pub struct MonoString {
    pad_0000: [u8; 16],
    length: i32,
    first_char: u16, // char16_t
}

impl MonoString {
    pub fn new(ptr: *const u16) -> Self {
        hooks::monostring_create::invoke(ptr)
    }
}

impl Into<String> for MonoString {
    fn into(self) -> String {
        let sl = unsafe { std::slice::from_raw_parts(&self.first_char, self.length as usize) };

        let res = String::from_utf16(&sl);
        match res {
            Ok(val) => val,
            Err(_) => String::new(),
        }
    }
}
