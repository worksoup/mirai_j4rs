use j4rs::errors::J4RsError;
use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::contact::NormalMember;
use crate::event::{
    BotEventTrait, BotPassiveEventTrait, GroupAwareMessageTrait, MessageEventTrait,
    OtherClientEventTrait, TempMessageEventTrait, UserMessageEventTrait,
};

#[mj_event]
pub struct GroupTempMessageEvent {
    instance: Instance,
}

impl GroupAwareMessageTrait<NormalMember, NormalMember> for GroupTempMessageEvent {}
impl TempMessageEventTrait<NormalMember> for GroupTempMessageEvent {}
impl UserMessageEventTrait<NormalMember, NormalMember> for GroupTempMessageEvent {}

impl MessageEventTrait<NormalMember, NormalMember> for GroupTempMessageEvent {}
impl BotEventTrait for GroupTempMessageEvent {}
impl BotPassiveEventTrait for GroupTempMessageEvent {}
impl OtherClientEventTrait for GroupTempMessageEvent {}
