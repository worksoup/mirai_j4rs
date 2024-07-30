use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait, GroupEventTrait,
    GroupMemberInfoChangeEventTrait, GroupOperableEventTrait,
};
use crate::utils::backend::BotBackend;

pub trait BotLeaveEventTrait<B: BotBackend>:
    BotEventTrait<B> + GroupMemberInfoChangeEventTrait<B>
{
}

#[mj_event("event.events.BotLeaveEvent")]
pub struct BotLeaveEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}
impl<B: BotBackend> BotLeaveEventTrait<B> for BotLeaveEvent<B> {}
impl<B: BotBackend> BotEventTrait<B> for BotLeaveEvent<B> {}
impl<B: BotBackend> BotPassiveEventTrait<B> for BotLeaveEvent<B> {}
impl<B: BotBackend> GroupEventTrait<B> for BotLeaveEvent<B> {}
impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for BotLeaveEvent<B> {}
impl<B: BotBackend> GroupMemberInfoChangeEventTrait<B> for BotLeaveEvent<B> {}

#[mj_event("event.events.BotLeaveEvent$Active")]
pub struct Active<B: BotBackend> {
    instance: Instance,
    _backend: B,
}
impl<B: BotBackend> BotLeaveEventTrait<B> for Active<B> {}
impl<B: BotBackend> BotEventTrait<B> for Active<B> {}
impl<B: BotBackend> BotPassiveEventTrait<B> for Active<B> {}
impl<B: BotBackend> GroupEventTrait<B> for Active<B> {}
impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for Active<B> {}
impl<B: BotBackend> GroupMemberInfoChangeEventTrait<B> for Active<B> {}

#[mj_event("event.events.BotLeaveEvent$Disband")]
pub struct Disband<B: BotBackend> {
    instance: Instance,
    _backend: B,
}
impl<B: BotBackend> GroupOperableEventTrait<B> for Disband<B> {}
impl<B: BotBackend> BotLeaveEventTrait<B> for Disband<B> {}
impl<B: BotBackend> BotEventTrait<B> for Disband<B> {}
impl<B: BotBackend> BotPassiveEventTrait<B> for Disband<B> {}
impl<B: BotBackend> GroupEventTrait<B> for Disband<B> {}
impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for Disband<B> {}
impl<B: BotBackend> GroupMemberInfoChangeEventTrait<B> for Disband<B> {}

#[mj_event("event.events.BotLeaveEvent$Kick")]
pub struct Kick<B: BotBackend> {
    instance: Instance,
    _backend: B,
}
impl<B: BotBackend> GroupOperableEventTrait<B> for Kick<B> {}
impl<B: BotBackend> BotLeaveEventTrait<B> for Kick<B> {}
impl<B: BotBackend> BotEventTrait<B> for Kick<B> {}
impl<B: BotBackend> BotPassiveEventTrait<B> for Kick<B> {}
impl<B: BotBackend> GroupEventTrait<B> for Kick<B> {}
impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for Kick<B> {}
impl<B: BotBackend> GroupMemberInfoChangeEventTrait<B> for Kick<B> {}
