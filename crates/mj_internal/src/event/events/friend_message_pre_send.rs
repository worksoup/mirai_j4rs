use crate::event::{
    BotActiveEventTrait, BotEventTrait, CancellableEventTrait, MessagePreSendEventTrait,
    MiraiEventTrait, UserMessagePreSendEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.FriendMessagePreSendEvent")]
pub struct FriendMessagePreSendEvent {
    instance: Instance,
}
impl MiraiEventTrait for FriendMessagePreSendEvent {}

impl MessagePreSendEventTrait for FriendMessagePreSendEvent {}

impl BotEventTrait for FriendMessagePreSendEvent {}

impl BotActiveEventTrait for FriendMessagePreSendEvent {}

impl CancellableEventTrait for FriendMessagePreSendEvent {}

impl UserMessagePreSendEventTrait for FriendMessagePreSendEvent {}
