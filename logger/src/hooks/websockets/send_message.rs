use std::ffi::c_void;

use colored::Colorize;

use crate::{hooks::websockets::WS_MESSAGE, mono::NetOutgoingMessage, utils::filesystem::{write_to_file, OUTPUT_WS_FILE_NAME}};

type WebsocketSendMessage<'a> = fn(&NetOutgoingMessage, u8, i32, *const c_void);

static mut WEBSOCKET_SEND_FN: Option<WebsocketSendMessage> = None;

pub unsafe fn hook(target: *mut c_void) {
    WEBSOCKET_SEND_FN = Some(std::mem::transmute(
        minhook::MinHook::create_hook(target, websocket_send_message_hook as _)
            .expect("WebSocketSnedMessage hook failed"),
    ));
}

fn websocket_send_message_hook(this: &NetOutgoingMessage, a2: u8, a3: i32, method: *const c_void) {
    {
        let msg = NetOutgoingMessage::to_string(this);
        let mut ws_msg = WS_MESSAGE.lock().unwrap();
        let ws_formatted_msg = format!("{{\n{}\n}}", ws_msg.clone());
        
        println!(
            "{}\n{}\n{}\n",
            "[packet sent]".bright_black(),
            msg.bright_black(),
            ws_formatted_msg.bright_cyan().bold()
        );

        write_to_file(OUTPUT_WS_FILE_NAME, &ws_formatted_msg);

        ws_msg.clear();
    }

    (unsafe { WEBSOCKET_SEND_FN.unwrap() })(this, a2, a3, method)
}
