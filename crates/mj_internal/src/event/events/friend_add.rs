use crate::event::{
    BotEventTrait, FriendEventTrait, FriendInfoChangeEventTrait, MiraiEventTrait, UserEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.FriendAddEvent")]
pub struct FriendAddEvent {
    instance: Instance,
}

impl BotEventTrait for FriendAddEvent {}

impl UserEventTrait for FriendAddEvent {}

impl FriendEventTrait for FriendAddEvent {}

impl MiraiEventTrait for FriendAddEvent {}

impl FriendInfoChangeEventTrait for FriendAddEvent {}
