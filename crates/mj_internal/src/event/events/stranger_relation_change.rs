use crate::event::{BotEventTrait, MiraiEventTrait, StrangerEventTrait, UserEventTrait};
use j4rs::Instance;
use mj_macro::{java_type, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.StrangerRelationChangeEvent")]
pub struct StrangerRelationChangeEvent {
    instance: Instance,
}
impl StrangerEventTrait for StrangerRelationChangeEvent {}
impl BotEventTrait for StrangerRelationChangeEvent {}
impl UserEventTrait for StrangerRelationChangeEvent {}
impl MiraiEventTrait for StrangerRelationChangeEvent {}
