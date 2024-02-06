use crate::event::{BaseGroupMemberInfoChangeEventTrait, BotEventTrait, MiraiEventTrait};
use j4rs::Instance;
use mj_macro::{java_type, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotInvitedJoinGroupRequestEvent")]
pub struct BotInvitedJoinGroupRequestEvent {
    instance: Instance,
}
impl BotEventTrait for BotInvitedJoinGroupRequestEvent {}
impl MiraiEventTrait for BotInvitedJoinGroupRequestEvent {}
impl BaseGroupMemberInfoChangeEventTrait for BotInvitedJoinGroupRequestEvent {}
