use crate::event::{BotEventTrait, BotPassiveEventTrait};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct BotGroupPermissionChangeEvent {
    instance: Instance,
}

impl BotEventTrait for BotGroupPermissionChangeEvent {}

impl BotPassiveEventTrait for BotGroupPermissionChangeEvent {}
