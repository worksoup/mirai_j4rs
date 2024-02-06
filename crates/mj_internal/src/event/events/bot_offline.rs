use crate::event::{BotEventTrait, BotOfflineEventTrait, MessageEventTrait, MiraiEventTrait};
use j4rs::Instance;
use mj_macro::{java_type, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

pub trait CauseAwareTrait {}
#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotOfflineEvent$Active")]
pub struct Active {
    instance: Instance,
}

impl BotOfflineEventTrait for Active {}
impl BotEventTrait for Active {}
impl MiraiEventTrait for Active {}
#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotOfflineEvent$Force")]
pub struct Force {
    instance: Instance,
}

impl BotOfflineEventTrait for Force {}
impl BotEventTrait for Force {}
impl MiraiEventTrait for Force {}
#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotOfflineEvent$Dropped")]
pub struct Dropped {
    instance: Instance,
}

impl BotOfflineEventTrait for Dropped {}
impl BotEventTrait for Dropped {}
impl MiraiEventTrait for Dropped {}
#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotOfflineEvent$RequireReconnect")]
pub struct RequireReconnect {
    instance: Instance,
}

impl BotOfflineEventTrait for RequireReconnect {}
impl BotEventTrait for RequireReconnect {}
impl MiraiEventTrait for RequireReconnect {}
#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotOfflineEvent$MsfOffline")]
pub struct MsfOffline {
    instance: Instance,
}

impl BotOfflineEventTrait for MsfOffline {}
impl BotEventTrait for MsfOffline {}
impl MiraiEventTrait for MsfOffline {}
#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotOfflineEvent")]
pub struct BotOfflineEvent {
    instance: Instance,
}

impl BotOfflineEventTrait for BotOfflineEvent {}
impl BotEventTrait for BotOfflineEvent {}
impl MiraiEventTrait for BotOfflineEvent {}
