use crate::contact::Friend;
use crate::event::{BotEventTrait, BotPassiveEventTrait, FriendMessageEvent, MessageEventTrait, MiraiEventTrait, OtherClientEventTrait};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.FriendMessageSyncEvent")]
pub struct FriendMessageSyncEvent {
    instance: Instance,
}
impl MiraiEventTrait for FriendMessageSyncEvent {}
impl OtherClientEventTrait for FriendMessageSyncEvent {}
impl MessageEventTrait<Friend, Friend> for FriendMessageSyncEvent {}

impl BotEventTrait for FriendMessageSyncEvent {}

impl BotPassiveEventTrait for FriendMessageSyncEvent {}
