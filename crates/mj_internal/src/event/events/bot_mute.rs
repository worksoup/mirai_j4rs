use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait, GroupEventTrait,
    GroupMemberInfoChangeEventTrait, MiraiEventTrait,
};
use j4rs::Instance;
use mj_macro::{java_type, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotMuteEvent")]
pub struct BotMuteEvent {
    instance: Instance,
}

impl BotEventTrait for BotMuteEvent {}

impl GroupEventTrait for BotMuteEvent {}

impl BotPassiveEventTrait for BotMuteEvent {}

impl MiraiEventTrait for BotMuteEvent {}

impl BaseGroupMemberInfoChangeEventTrait for BotMuteEvent {}

impl GroupMemberInfoChangeEventTrait for BotMuteEvent {}
