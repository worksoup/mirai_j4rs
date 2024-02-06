use crate::event::{BotEventTrait, BotPassiveEventTrait, OtherClientEventTrait};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct OtherClientOnlineEvent {
    instance: Instance,
}

impl OtherClientEventTrait for OtherClientOnlineEvent {}
impl BotPassiveEventTrait for OtherClientOnlineEvent {}

impl BotEventTrait for OtherClientOnlineEvent {}
