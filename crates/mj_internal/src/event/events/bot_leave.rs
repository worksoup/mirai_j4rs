use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotLeaveEventTrait, BotPassiveEventTrait,
    GroupEventTrait, GroupMemberInfoChangeEventTrait, GroupOperableEventTrait, MiraiEventTrait,
};
use j4rs::Instance;
use mj_macro::{java_type, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotLeaveEvent")]
pub struct BotLeaveEvent {
    instance: Instance,
}
impl BotLeaveEventTrait for BotLeaveEvent {}
impl BotEventTrait for BotLeaveEvent {}
impl BotPassiveEventTrait for BotLeaveEvent {}
impl MiraiEventTrait for BotLeaveEvent {}
impl GroupEventTrait for BotLeaveEvent {}
impl BaseGroupMemberInfoChangeEventTrait for BotLeaveEvent {}
impl GroupMemberInfoChangeEventTrait for BotLeaveEvent {}
#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotLeaveEvent$Active")]
pub struct Active {
    instance: Instance,
}
impl BotLeaveEventTrait for Active {}
impl BotEventTrait for Active {}
impl BotPassiveEventTrait for Active {}
impl MiraiEventTrait for Active {}
impl GroupEventTrait for Active {}
impl BaseGroupMemberInfoChangeEventTrait for Active {}
impl GroupMemberInfoChangeEventTrait for Active {}
#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotLeaveEvent$Disband")]
pub struct Disband {
    instance: Instance,
}
impl GroupOperableEventTrait for Disband {}
impl BotLeaveEventTrait for Disband {}
impl BotEventTrait for Disband {}
impl BotPassiveEventTrait for Disband {}
impl MiraiEventTrait for Disband {}
impl GroupEventTrait for Disband {}
impl BaseGroupMemberInfoChangeEventTrait for Disband {}
impl GroupMemberInfoChangeEventTrait for Disband {}
#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotLeaveEvent$Kick")]
pub struct Kick {
    instance: Instance,
}
impl GroupOperableEventTrait for Kick {}
impl BotLeaveEventTrait for Kick {}
impl BotEventTrait for Kick {}
impl BotPassiveEventTrait for Kick {}
impl MiraiEventTrait for Kick {}
impl GroupEventTrait for Kick {}
impl BaseGroupMemberInfoChangeEventTrait for Kick {}
impl GroupMemberInfoChangeEventTrait for Kick {}
