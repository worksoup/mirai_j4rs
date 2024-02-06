use crate::event::{BotEventTrait, MiraiEventTrait, StrangerEventTrait, UserEventTrait};
use j4rs::Instance;
use mj_macro::{java_type, mj_all, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

#[mj_all("net.mamoe.mirai.event.events.StrangerRelationChangeEvent")]
pub struct StrangerRelationChangeEvent {
    instance: Instance,
}
impl StrangerEventTrait for StrangerRelationChangeEvent {}
impl BotEventTrait for StrangerRelationChangeEvent {}
impl UserEventTrait for StrangerRelationChangeEvent {}
impl MiraiEventTrait for StrangerRelationChangeEvent {}
