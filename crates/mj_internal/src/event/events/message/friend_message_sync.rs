use j4rs::Instance;

use mj_macro::mj_event;

use crate::contact::Friend;
use crate::event::{BotEventTrait, BotPassiveEventTrait, MessageEventTrait, OtherClientEventTrait};

#[mj_event]
pub struct FriendMessageSyncEvent {
    instance: Instance,
}

impl OtherClientEventTrait for FriendMessageSyncEvent {}
impl MessageEventTrait<Friend, Friend> for FriendMessageSyncEvent {}

impl BotEventTrait for FriendMessageSyncEvent {}

impl BotPassiveEventTrait for FriendMessageSyncEvent {}
