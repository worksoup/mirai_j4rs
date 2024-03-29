use j4rs::Instance;

use mj_macro::mj_event;

use crate::event::{BotActiveEventTrait, BotEventTrait};

#[mj_event]
pub struct ImageUploadEvent {
    instance: Instance,
}
impl BotEventTrait for ImageUploadEvent {}
impl BotActiveEventTrait for ImageUploadEvent {}
