use j4rs::Instance;

use mj_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait, GroupEventTrait,
    GroupMemberInfoChangeEventTrait, GroupOperableEventTrait,
};

pub trait BotLeaveEventTrait: BotEventTrait + GroupMemberInfoChangeEventTrait {}

#[mj_event("event.events.BotLeaveEvent")]
pub struct BotLeaveEvent {
    instance: Instance,
}
impl BotLeaveEventTrait for BotLeaveEvent {}
impl BotEventTrait for BotLeaveEvent {}
impl BotPassiveEventTrait for BotLeaveEvent {}
impl GroupEventTrait for BotLeaveEvent {}
impl BaseGroupMemberInfoChangeEventTrait for BotLeaveEvent {}
impl GroupMemberInfoChangeEventTrait for BotLeaveEvent {}

#[mj_event("event.events.BotLeaveEvent$Active")]
pub struct Active {
    instance: Instance,
}
impl BotLeaveEventTrait for Active {}
impl BotEventTrait for Active {}
impl BotPassiveEventTrait for Active {}
impl GroupEventTrait for Active {}
impl BaseGroupMemberInfoChangeEventTrait for Active {}
impl GroupMemberInfoChangeEventTrait for Active {}

#[mj_event("event.events.BotLeaveEvent$Disband")]
pub struct Disband {
    instance: Instance,
}
impl GroupOperableEventTrait for Disband {}
impl BotLeaveEventTrait for Disband {}
impl BotEventTrait for Disband {}
impl BotPassiveEventTrait for Disband {}
impl GroupEventTrait for Disband {}
impl BaseGroupMemberInfoChangeEventTrait for Disband {}
impl GroupMemberInfoChangeEventTrait for Disband {}

#[mj_event("event.events.BotLeaveEvent$Kick")]
pub struct Kick {
    instance: Instance,
}
impl GroupOperableEventTrait for Kick {}
impl BotLeaveEventTrait for Kick {}
impl BotEventTrait for Kick {}
impl BotPassiveEventTrait for Kick {}
impl GroupEventTrait for Kick {}
impl BaseGroupMemberInfoChangeEventTrait for Kick {}
impl GroupMemberInfoChangeEventTrait for Kick {}
