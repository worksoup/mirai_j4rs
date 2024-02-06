use crate::event::{BotActiveEventTrait, BotEventTrait, MiraiEventTrait};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.ImageUploadEvent")]
pub struct ImageUploadEvent {
    instance: Instance,
}
impl BotEventTrait for ImageUploadEvent {}
impl BotActiveEventTrait for ImageUploadEvent {}

impl MiraiEventTrait for ImageUploadEvent {}
