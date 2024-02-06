use crate::event::{BotEventTrait, StrangerEventTrait, UserEventTrait};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct StrangerAddEvent {
    instance: Instance,
}

impl BotEventTrait for StrangerAddEvent {}

impl UserEventTrait for StrangerAddEvent {}

impl StrangerEventTrait for StrangerAddEvent {}
