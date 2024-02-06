use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait,
    BroadcastControllableTrait, GroupEventTrait, GroupMemberInfoChangeEventTrait,
    GroupOperableEventTrait, GroupSettingsChangeEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct GroupEntranceAnnouncementChangeEvent {
    instance: Instance,
}

impl GroupSettingsChangeEventTrait<String> for GroupEntranceAnnouncementChangeEvent {}
impl GroupOperableEventTrait for GroupEntranceAnnouncementChangeEvent {}
impl GroupMemberInfoChangeEventTrait for GroupEntranceAnnouncementChangeEvent {}

impl BotEventTrait for GroupEntranceAnnouncementChangeEvent {}
impl GroupEventTrait for GroupEntranceAnnouncementChangeEvent {}
impl BaseGroupMemberInfoChangeEventTrait for GroupEntranceAnnouncementChangeEvent {}
impl BroadcastControllableTrait for GroupEntranceAnnouncementChangeEvent {}
impl BotPassiveEventTrait for GroupEntranceAnnouncementChangeEvent {}
