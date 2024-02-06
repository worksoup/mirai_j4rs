use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait,
    BroadcastControllableTrait, GroupEventTrait, GroupMemberInfoChangeEventTrait,
    GroupOperableEventTrait, GroupSettingsChangeEventTrait, MiraiEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.GroupEntranceAnnouncementChangeEvent")]
pub struct GroupEntranceAnnouncementChangeEvent {
    instance: Instance,
}

impl GroupSettingsChangeEventTrait<String> for GroupEntranceAnnouncementChangeEvent {}
impl GroupOperableEventTrait for GroupEntranceAnnouncementChangeEvent {}
impl GroupMemberInfoChangeEventTrait for GroupEntranceAnnouncementChangeEvent {}

impl MiraiEventTrait for GroupEntranceAnnouncementChangeEvent {}
impl BotEventTrait for GroupEntranceAnnouncementChangeEvent {}
impl GroupEventTrait for GroupEntranceAnnouncementChangeEvent {}
impl BaseGroupMemberInfoChangeEventTrait for GroupEntranceAnnouncementChangeEvent {}
impl BroadcastControllableTrait for GroupEntranceAnnouncementChangeEvent {}
impl BotPassiveEventTrait for GroupEntranceAnnouncementChangeEvent {}
