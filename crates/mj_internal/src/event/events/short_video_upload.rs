use j4rs::Instance;

use mj_macro::mj_event;

use crate::event::{BotActiveEventTrait, BotEventTrait};

#[mj_event]
pub struct ShortVideoUploadEvent {
    instance: Instance,
}
impl BotEventTrait for ShortVideoUploadEvent {}
impl BotActiveEventTrait for ShortVideoUploadEvent {}
