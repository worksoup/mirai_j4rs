use j4rs::Instance;

use mj_macro::mj_event;

use crate::contact::Friend;
use crate::event::{
    BotActiveEventTrait, BotEventTrait, MessagePostSendEventTrait, UserMessagePostSendEventTrait,
};

#[mj_event]
pub struct FriendMessagePostSendEvent {
    instance: Instance,
}

impl MessagePostSendEventTrait<Friend> for FriendMessagePostSendEvent {}

impl BotEventTrait for FriendMessagePostSendEvent {}

impl BotActiveEventTrait for FriendMessagePostSendEvent {}

impl UserMessagePostSendEventTrait<Friend> for FriendMessagePostSendEvent {}
