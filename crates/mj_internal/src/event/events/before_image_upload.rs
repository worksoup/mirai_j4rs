use crate::event::{BotActiveEventTrait, BotEventTrait, CancellableEventTrait};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct BeforeImageUploadEvent {
    instance: Instance,
}

impl BotEventTrait for BeforeImageUploadEvent {}

impl BotActiveEventTrait for BeforeImageUploadEvent {}
impl CancellableEventTrait for BeforeImageUploadEvent {}
