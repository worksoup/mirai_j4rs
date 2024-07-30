use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait,
    BroadcastControllableTrait, GroupEventTrait, GroupMemberInfoChangeEventTrait,
    GroupSettingsChangeEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct GroupAllowConfessTalkEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> GroupSettingsChangeEventTrait<B, bool> for GroupAllowConfessTalkEvent<B> {}
impl<B: BotBackend> GroupMemberInfoChangeEventTrait<B> for GroupAllowConfessTalkEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for GroupAllowConfessTalkEvent<B> {}
impl<B: BotBackend> GroupEventTrait<B> for GroupAllowConfessTalkEvent<B> {}
impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for GroupAllowConfessTalkEvent<B> {}

impl<B:BotBackend> BroadcastControllableTrait<B> for GroupAllowConfessTalkEvent<B> {}
impl<B: BotBackend> BotPassiveEventTrait<B> for GroupAllowConfessTalkEvent<B> {}
