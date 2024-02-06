use crate::event::{
    BotEventTrait, FriendEventTrait, FriendInfoChangeEventTrait, MiraiEventTrait, UserEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.FriendNickChangedEvent")]
pub struct FriendNickChangedEvent {
    instance: Instance,
}
impl MiraiEventTrait for FriendNickChangedEvent {}

impl BotEventTrait for FriendNickChangedEvent {}

impl UserEventTrait for FriendNickChangedEvent {}

impl FriendEventTrait for FriendNickChangedEvent {}
impl FriendInfoChangeEventTrait for FriendNickChangedEvent {}
