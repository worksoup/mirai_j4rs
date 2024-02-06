use crate::event::{BotActiveEventTrait, BotEventTrait};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct ImageUploadEvent {
    instance: Instance,
}
impl BotEventTrait for ImageUploadEvent {}
impl BotActiveEventTrait for ImageUploadEvent {}