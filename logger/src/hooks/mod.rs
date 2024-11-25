use patterns::{
    JSON_SERIALIZE, MONOSTRING_CREATE, NETOUTGOINGMESSAGE_TO_STRING, WEBSOCKET_SEND, WS_WRITE_BOOL,
    WS_WRITE_FLOAT, WS_WRITE_INT16, WS_WRITE_INT8, WS_WRITE_STRING, WS_WRITE_UINT32,
};

use crate::{
    memory::Scanner,
    utils::{get_absolute_address, ModuleInfo},
};

mod json_serialize;
mod patterns;
mod websockets;

pub mod monostring_create;
pub mod net_outgoing_message;

pub unsafe fn load() {
    let mi = ModuleInfo::new("GameAssembly.dll");
    let scanner = Scanner::new(mi);

    monostring_create::hook(scanner.find(MONOSTRING_CREATE).unwrap());
    net_outgoing_message::hook(scanner.find(NETOUTGOINGMESSAGE_TO_STRING).unwrap());
    json_serialize::hook(scanner.find(JSON_SERIALIZE).unwrap().cast_mut());

    websockets::send_message::hook(
        get_absolute_address(scanner.find(WEBSOCKET_SEND).unwrap(), 1).cast_mut(),
    );
    websockets::write_uint32::hook(scanner.find(WS_WRITE_UINT32).unwrap().cast_mut());
    websockets::write_int8::hook(
        get_absolute_address(scanner.find(WS_WRITE_INT8).unwrap(), 1).cast_mut(),
    );
    websockets::write_bool::hook(
        get_absolute_address(scanner.find(WS_WRITE_BOOL).unwrap(), 1).cast_mut(),
    );
    websockets::write_float::hook(
        get_absolute_address(scanner.find(WS_WRITE_FLOAT).unwrap(), 1).cast_mut(),
    );
    websockets::write_int16::hook(
        get_absolute_address(scanner.find(WS_WRITE_INT16).unwrap(), 1).cast_mut(),
    );
    websockets::write_string::hook(
        get_absolute_address(scanner.find(WS_WRITE_STRING).unwrap(), 1).cast_mut(),
    );

    minhook::MinHook::enable_all_hooks().unwrap();
}

pub fn unload() {}
