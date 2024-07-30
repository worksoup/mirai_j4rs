use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait,
    BroadcastControllableTrait, GroupEventTrait, GroupMemberInfoChangeEventTrait,
    GroupOperableEventTrait, GroupSettingsChangeEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct GroupEntranceAnnouncementChangeEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B:BotBackend> GroupSettingsChangeEventTrait<B, String> for GroupEntranceAnnouncementChangeEvent<B> {}
impl<B:BotBackend> GroupOperableEventTrait<B> for GroupEntranceAnnouncementChangeEvent<B> {}
impl<B:BotBackend> GroupMemberInfoChangeEventTrait<B> for GroupEntranceAnnouncementChangeEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for GroupEntranceAnnouncementChangeEvent<B> {}
impl<B:BotBackend> GroupEventTrait<B> for GroupEntranceAnnouncementChangeEvent<B> {}
impl<B:BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for GroupEntranceAnnouncementChangeEvent<B> {}
impl<B:BotBackend> BroadcastControllableTrait<B> for GroupEntranceAnnouncementChangeEvent<B> {}
impl<B:BotBackend> BotPassiveEventTrait<B> for GroupEntranceAnnouncementChangeEvent<B> {}
