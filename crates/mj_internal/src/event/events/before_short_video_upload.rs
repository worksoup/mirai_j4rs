use crate::event::{BotActiveEventTrait, BotEventTrait, CancellableEventTrait};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct BeforeShortVideoUploadEvent {
    instance: Instance,
}

impl BotEventTrait for BeforeShortVideoUploadEvent {}

impl BotActiveEventTrait for BeforeShortVideoUploadEvent {}
impl CancellableEventTrait for BeforeShortVideoUploadEvent {}
