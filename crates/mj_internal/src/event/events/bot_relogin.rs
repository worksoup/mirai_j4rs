use j4rs::Instance;

use mj_macro::mj_event;

use crate::event::{BotActiveEventTrait, BotEventTrait};

#[mj_event]
pub struct BotReloginEvent {
    instance: Instance,
}

impl BotReloginEvent {}

impl BotEventTrait for BotReloginEvent {}

impl BotActiveEventTrait for BotReloginEvent {}
