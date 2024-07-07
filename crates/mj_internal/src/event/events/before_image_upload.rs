use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BotActiveEventTrait, BotEventTrait, CancellableEventTrait};

#[mj_event]
pub struct BeforeImageUploadEvent {
    instance: Instance,
}

impl BotEventTrait for BeforeImageUploadEvent {}

impl BotActiveEventTrait for BeforeImageUploadEvent {}
impl CancellableEventTrait for BeforeImageUploadEvent {}
