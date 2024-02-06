use j4rs::Instance;

use mj_macro::mj_event;

use crate::event::{BotEventTrait, BotPassiveEventTrait};

#[mj_event]
pub struct BotGroupPermissionChangeEvent {
    instance: Instance,
}

impl BotEventTrait for BotGroupPermissionChangeEvent {}

impl BotPassiveEventTrait for BotGroupPermissionChangeEvent {}
