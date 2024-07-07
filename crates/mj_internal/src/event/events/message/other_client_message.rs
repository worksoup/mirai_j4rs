use j4rs::errors::J4RsError;
use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::contact::{OtherClient, User};
use crate::event::{BotEventTrait, BotPassiveEventTrait, MessageEventTrait, OtherClientEventTrait};

#[mj_event]
pub struct OtherClientMessageEvent {
    instance: Instance,
}
impl MessageEventTrait<User, OtherClient> for OtherClientMessageEvent {}
impl BotPassiveEventTrait for OtherClientMessageEvent {}

impl BotEventTrait for OtherClientMessageEvent {}
impl OtherClientEventTrait for OtherClientMessageEvent {}
