use crate::event::{BotEventTrait, MiraiEventTrait};
use j4rs::Instance;
use mj_macro::{java_type, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotAvatarChangedEvent")]
pub struct BotAvatarChangedEvent {
    instance: Instance,
}
impl BotEventTrait for BotAvatarChangedEvent {}
impl MiraiEventTrait for BotAvatarChangedEvent {}
