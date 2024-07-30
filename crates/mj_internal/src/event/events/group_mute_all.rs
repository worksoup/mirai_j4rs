use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait,
    BroadcastControllableTrait, GroupEventTrait, GroupMemberInfoChangeEventTrait,
    GroupOperableEventTrait, GroupSettingsChangeEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct GroupMuteAllEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B:BotBackend> GroupSettingsChangeEventTrait<B, bool> for GroupMuteAllEvent<B> {}
impl<B:BotBackend> GroupOperableEventTrait<B> for GroupMuteAllEvent<B> {}
impl<B:BotBackend> GroupMemberInfoChangeEventTrait<B> for GroupMuteAllEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for GroupMuteAllEvent<B> {}
impl<B:BotBackend> GroupEventTrait<B> for GroupMuteAllEvent<B> {}
impl<B:BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for GroupMuteAllEvent<B> {}
impl<B:BotBackend> BroadcastControllableTrait<B> for GroupMuteAllEvent<B> {}
impl<B:BotBackend> BotPassiveEventTrait<B> for GroupMuteAllEvent<B> {}
