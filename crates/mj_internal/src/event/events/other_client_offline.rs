use j4rs::Instance;

use mj_macro::mj_event;

use crate::event::{BotEventTrait, BotPassiveEventTrait, OtherClientEventTrait};

#[mj_event]
pub struct OtherClientOfflineEvent {
    instance: Instance,
}

impl OtherClientEventTrait for OtherClientOfflineEvent {}
impl BotPassiveEventTrait for OtherClientOfflineEvent {}

impl BotEventTrait for OtherClientOfflineEvent {}
