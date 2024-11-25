use std::{ffi::c_void, ptr::null};

use crate::mono::{MethodInfo, MonoString, NetOutgoingMessage};

type NetOutgoingMessageToString<'a> =
    fn(this: &NetOutgoingMessage, method: *const MethodInfo) -> &'a MonoString;
static mut NET_OUTGOING_MESSAGE_TO_STRING_FN: Option<NetOutgoingMessageToString> = None;

pub(super) unsafe fn hook(target: *const c_void) {
    NET_OUTGOING_MESSAGE_TO_STRING_FN = Some(std::mem::transmute(target));
}

pub fn invoke<'a>(msg: &'a NetOutgoingMessage) -> &'a MonoString {
    unsafe { (NET_OUTGOING_MESSAGE_TO_STRING_FN.unwrap())(msg, null()) }
}
