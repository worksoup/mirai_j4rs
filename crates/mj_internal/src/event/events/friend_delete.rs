use crate::event::{
    BotEventTrait, FriendEventTrait, FriendInfoChangeEventTrait, MiraiEventTrait, UserEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.FriendDeleteEvent")]
pub struct FriendDeleteEvent {
    instance: Instance,
}

impl BotEventTrait for FriendDeleteEvent {}

impl UserEventTrait for FriendDeleteEvent {}

impl FriendEventTrait for FriendDeleteEvent {}

impl MiraiEventTrait for FriendDeleteEvent {}

impl FriendInfoChangeEventTrait for FriendDeleteEvent {}
