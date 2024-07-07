use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BotActiveEventTrait, BotEventTrait, CancellableEventTrait};

#[mj_event]
pub struct BeforeShortVideoUploadEvent {
    instance: Instance,
}

impl BotEventTrait for BeforeShortVideoUploadEvent {}

impl BotActiveEventTrait for BeforeShortVideoUploadEvent {}
impl CancellableEventTrait for BeforeShortVideoUploadEvent {}
