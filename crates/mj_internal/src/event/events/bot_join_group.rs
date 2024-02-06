use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait, GroupEventTrait,
    GroupMemberInfoChangeEventTrait, MiraiEventTrait,
};
use j4rs::Instance;
use mj_macro::{java_type, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotJoinGroupEvent")]
pub struct BotJoinGroupEvent {
    instance: Instance,
}
impl BotEventTrait for BotJoinGroupEvent {}
impl BotPassiveEventTrait for BotJoinGroupEvent {}
impl MiraiEventTrait for BotJoinGroupEvent {}
impl GroupEventTrait for BotJoinGroupEvent {}
impl BaseGroupMemberInfoChangeEventTrait for BotJoinGroupEvent {}
impl GroupMemberInfoChangeEventTrait for BotJoinGroupEvent {}
