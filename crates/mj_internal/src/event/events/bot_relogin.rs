use crate::event::{BotActiveEventTrait, BotEventTrait, MiraiEventTrait};
use j4rs::{Instance, Jvm};
use mj_macro::{java_type, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotReloginEvent")]
pub struct BotReloginEvent {
    instance: Instance,
}

impl BotReloginEvent {}

impl BotEventTrait for BotReloginEvent {}

impl BotActiveEventTrait for BotReloginEvent {}

impl MiraiEventTrait for BotReloginEvent {}
