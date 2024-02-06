use j4rs::Instance;

use mj_macro::mj_event;

use crate::contact::Stranger;
use crate::event::{
    BotEventTrait, BotPassiveEventTrait, MessageEventTrait, OtherClientEventTrait,
    StrangerEventTrait, UserEventTrait, UserMessageEventTrait,
};

#[mj_event]
pub struct StrangerMessageEvent {
    instance: Instance,
}

impl MessageEventTrait<Stranger, Stranger> for StrangerMessageEvent {}

impl UserMessageEventTrait<Stranger, Stranger> for StrangerMessageEvent {}
impl BotPassiveEventTrait for StrangerMessageEvent {}
impl OtherClientEventTrait for StrangerMessageEvent {}

impl BotEventTrait for StrangerMessageEvent {}

impl UserEventTrait for StrangerMessageEvent {}

impl StrangerEventTrait for StrangerMessageEvent {}
