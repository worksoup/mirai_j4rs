use crate::event::{BotActiveEventTrait, BotEventTrait};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct BotReloginEvent {
    instance: Instance,
}

impl BotReloginEvent {}

impl BotEventTrait for BotReloginEvent {}

impl BotActiveEventTrait for BotReloginEvent {}
