use crate::contact::Friend;
use crate::event::{
    BotEventTrait, BotPassiveEventTrait, FriendEventTrait, MessageEventTrait,
    OtherClientEventTrait, UserEventTrait, UserMessageEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct FriendMessageEvent {
    instance: Instance,
}

impl FriendMessageEvent {}

impl MessageEventTrait<Friend, Friend> for FriendMessageEvent {}
impl OtherClientEventTrait for FriendMessageEvent {}

impl UserMessageEventTrait<Friend, Friend> for FriendMessageEvent {}
impl UserEventTrait for FriendMessageEvent {}

impl FriendEventTrait for FriendMessageEvent {}

impl BotEventTrait for FriendMessageEvent {}

impl BotPassiveEventTrait for FriendMessageEvent {}
