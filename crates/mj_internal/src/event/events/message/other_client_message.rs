use crate::contact::{OtherClient, User};
use crate::event::{BotEventTrait, BotPassiveEventTrait, MessageEventTrait, OtherClientEventTrait};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct OtherClientMessageEvent {
    instance: Instance,
}
impl MessageEventTrait<User, OtherClient> for OtherClientMessageEvent {}
impl BotPassiveEventTrait for OtherClientMessageEvent {}

impl BotEventTrait for OtherClientMessageEvent {}
impl OtherClientEventTrait for OtherClientMessageEvent {}
