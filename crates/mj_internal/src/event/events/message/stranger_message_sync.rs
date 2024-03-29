use j4rs::Instance;

use mj_macro::mj_event;

use crate::contact::Stranger;
use crate::event::{
    BotEventTrait, BotPassiveEventTrait, MessageEventTrait, MessageSyncEventTrait,
    OtherClientEventTrait, StrangerEventTrait, UserEventTrait,
};

#[mj_event]
pub struct StrangerMessageSyncEvent {
    instance: Instance,
}

impl BotEventTrait for StrangerMessageSyncEvent {}

impl UserEventTrait for StrangerMessageSyncEvent {}

impl StrangerEventTrait for StrangerMessageSyncEvent {}

impl MessageEventTrait<Stranger, Stranger> for StrangerMessageSyncEvent {}

impl OtherClientEventTrait for StrangerMessageSyncEvent {}

impl MessageSyncEventTrait<Stranger, Stranger> for StrangerMessageSyncEvent {}
impl BotPassiveEventTrait for StrangerMessageSyncEvent {}
