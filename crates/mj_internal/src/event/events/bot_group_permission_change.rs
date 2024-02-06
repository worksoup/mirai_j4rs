use crate::event::{BotEventTrait, BotPassiveEventTrait, MiraiEventTrait};
use j4rs::Instance;
use mj_macro::{java_type, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotGroupPermissionChangeEvent")]
pub struct BotGroupPermissionChangeEvent {
    instance: Instance,
}

impl BotEventTrait for BotGroupPermissionChangeEvent {}

impl BotPassiveEventTrait for BotGroupPermissionChangeEvent {}
impl MiraiEventTrait for BotGroupPermissionChangeEvent {}
