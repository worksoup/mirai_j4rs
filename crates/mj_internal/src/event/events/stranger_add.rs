use j4rs::errors::J4RsError;
use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BotEventTrait, StrangerEventTrait, UserEventTrait};

#[mj_event]
pub struct StrangerAddEvent {
    instance: Instance,
}

impl BotEventTrait for StrangerAddEvent {}

impl UserEventTrait for StrangerAddEvent {}

impl StrangerEventTrait for StrangerAddEvent {}
