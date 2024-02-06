use crate::contact::Friend;
use crate::event::{
    BotActiveEventTrait, BotEventTrait, MessagePostSendEventTrait, MiraiEventTrait,
    UserMessagePostSendEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.FriendMessagePostSendEvent")]
pub struct FriendMessagePostSendEvent {
    instance: Instance,
}
impl MiraiEventTrait for FriendMessagePostSendEvent {}
impl MessagePostSendEventTrait<Friend> for FriendMessagePostSendEvent {}

impl BotEventTrait for FriendMessagePostSendEvent {}

impl BotActiveEventTrait for FriendMessagePostSendEvent {}

impl UserMessagePostSendEventTrait<Friend> for FriendMessagePostSendEvent {}
