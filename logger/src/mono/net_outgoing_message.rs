use crate::hooks::net_outgoing_message;

pub struct NetOutgoingMessage;

impl NetOutgoingMessage {
    pub fn to_string(this: &Self) -> String {
        net_outgoing_message::invoke(this).into()
    }
}
