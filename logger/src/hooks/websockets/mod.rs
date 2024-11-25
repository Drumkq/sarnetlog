pub mod send_message;
pub mod write_bool;
pub mod write_float;
pub mod write_int16;
pub mod write_int8;
pub mod write_string;
pub mod write_uint32;

use std::sync::Mutex;

static WS_MESSAGE: Mutex<String> = Mutex::new(String::new());
