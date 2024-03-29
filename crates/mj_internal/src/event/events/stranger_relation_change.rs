use j4rs::Instance;

use mj_macro::mj_event;

use crate::event::{BotEventTrait, StrangerEventTrait, UserEventTrait};

#[mj_event]
pub struct StrangerRelationChangeEvent {
    instance: Instance,
}
impl StrangerEventTrait for StrangerRelationChangeEvent {}
impl BotEventTrait for StrangerRelationChangeEvent {}
impl UserEventTrait for StrangerRelationChangeEvent {}
