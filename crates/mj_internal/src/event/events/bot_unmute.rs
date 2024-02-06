use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait, GroupEventTrait,
    GroupMemberInfoChangeEventTrait, MiraiEventTrait,
};
use j4rs::Instance;
use mj_macro::{java_type, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotUnmuteEvent")]
pub struct BotUnmuteEvent {
    instance: Instance,
}

impl BotEventTrait for BotUnmuteEvent {}

impl GroupEventTrait for BotUnmuteEvent {}

impl BotPassiveEventTrait for BotUnmuteEvent {}

impl MiraiEventTrait for BotUnmuteEvent {}

impl BaseGroupMemberInfoChangeEventTrait for BotUnmuteEvent {}

impl GroupMemberInfoChangeEventTrait for BotUnmuteEvent {}
