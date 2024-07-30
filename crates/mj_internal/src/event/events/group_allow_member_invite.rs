use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait,
    BroadcastControllableTrait, GroupEventTrait, GroupMemberInfoChangeEventTrait,
    GroupOperableEventTrait, GroupSettingsChangeEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct GroupAllowMemberInviteEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> GroupSettingsChangeEventTrait<B, bool> for GroupAllowMemberInviteEvent<B> {}
impl<B: BotBackend> GroupOperableEventTrait<B> for GroupAllowMemberInviteEvent<B> {}
impl<B: BotBackend> GroupMemberInfoChangeEventTrait<B> for GroupAllowMemberInviteEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for GroupAllowMemberInviteEvent<B> {}
impl<B: BotBackend> GroupEventTrait<B> for GroupAllowMemberInviteEvent<B> {}
impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for GroupAllowMemberInviteEvent<B> {}
impl<B:BotBackend> BroadcastControllableTrait<B> for GroupAllowMemberInviteEvent<B> {}
impl<B: BotBackend> BotPassiveEventTrait<B> for GroupAllowMemberInviteEvent<B> {}
