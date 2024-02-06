use crate::contact::NormalMember;
use crate::event::{
    BotEventTrait, BotPassiveEventTrait, GroupAwareMessageTrait, MessageEventTrait,
    MiraiEventTrait, OtherClientEventTrait, TempMessageEventTrait, UserMessageEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("")]
pub struct GroupTempMessageEvent {
    instance: Instance,
}

impl GroupAwareMessageTrait<NormalMember, NormalMember> for GroupTempMessageEvent {}
impl TempMessageEventTrait<NormalMember> for GroupTempMessageEvent {}
impl UserMessageEventTrait<NormalMember, NormalMember> for GroupTempMessageEvent {}

impl MessageEventTrait<NormalMember, NormalMember> for GroupTempMessageEvent {}
impl MiraiEventTrait for GroupTempMessageEvent {}
impl BotEventTrait for GroupTempMessageEvent {}
impl BotPassiveEventTrait for GroupTempMessageEvent {}
impl OtherClientEventTrait for GroupTempMessageEvent {}
