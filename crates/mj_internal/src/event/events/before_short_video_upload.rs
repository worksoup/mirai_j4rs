use crate::event::{BotActiveEventTrait, BotEventTrait, CancellableEventTrait, MiraiEventTrait};
use j4rs::Instance;
use mj_macro::{java_type, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BeforeShortVideoUploadEvent")]
pub struct BeforeShortVideoUploadEvent {
    instance: Instance,
}

impl BotEventTrait for BeforeShortVideoUploadEvent {}

impl BotActiveEventTrait for BeforeShortVideoUploadEvent {}
impl MiraiEventTrait for BeforeShortVideoUploadEvent {}
impl CancellableEventTrait for BeforeShortVideoUploadEvent {}
