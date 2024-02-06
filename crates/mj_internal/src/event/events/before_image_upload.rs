use crate::event::{BotActiveEventTrait, BotEventTrait, CancellableEventTrait, MiraiEventTrait};
use j4rs::Instance;
use mj_macro::{java_type, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BeforeImageUploadEvent")]
pub struct BeforeImageUploadEvent {
    instance: Instance,
}

impl BotEventTrait for BeforeImageUploadEvent {}

impl BotActiveEventTrait for BeforeImageUploadEvent {}
impl MiraiEventTrait for BeforeImageUploadEvent {}
impl CancellableEventTrait for BeforeImageUploadEvent {}
