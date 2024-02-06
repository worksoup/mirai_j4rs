use crate::event::{BotEventTrait, MiraiEventTrait};
use j4rs::Instance;
use mj_macro::mj_event;

pub trait BotOfflineEventTrait: BotEventTrait {}
pub trait CauseAwareTrait {}
#[mj_event("net.mamoe.mirai.event.events.BotOfflineEvent$Active")]
pub struct Active {
    instance: Instance,
}

impl BotOfflineEventTrait for Active {}
impl BotEventTrait for Active {}
#[mj_event("net.mamoe.mirai.event.events.BotOfflineEvent$Force")]
pub struct Force {
    instance: Instance,
}

impl BotOfflineEventTrait for Force {}
impl BotEventTrait for Force {}
#[mj_event("net.mamoe.mirai.event.events.BotOfflineEvent$Dropped")]
pub struct Dropped {
    instance: Instance,
}

impl BotOfflineEventTrait for Dropped {}
impl BotEventTrait for Dropped {}
#[mj_event("net.mamoe.mirai.event.events.BotOfflineEvent$RequireReconnect")]
pub struct RequireReconnect {
    instance: Instance,
}

impl BotOfflineEventTrait for RequireReconnect {}
impl BotEventTrait for RequireReconnect {}
#[mj_event("net.mamoe.mirai.event.events.BotOfflineEvent$MsfOffline")]
pub struct MsfOffline {
    instance: Instance,
}

impl BotOfflineEventTrait for MsfOffline {}
impl BotEventTrait for MsfOffline {}
#[mj_event]
pub struct BotOfflineEvent {
    instance: Instance,
}

impl BotOfflineEventTrait for BotOfflineEvent {}
impl BotEventTrait for BotOfflineEvent {}
