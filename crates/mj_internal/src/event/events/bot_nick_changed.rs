use crate::event::{BotEventTrait, MiraiEventTrait};
use j4rs::Instance;
use mj_macro::{java_type, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotNickChangedEvent")]
pub struct BotNickChangedEvent {
    instance: Instance,
}
impl BotEventTrait for BotNickChangedEvent {}
impl MiraiEventTrait for BotNickChangedEvent {}
