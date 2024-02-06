use crate::event::{
    BotEventTrait, BroadcastControllableTrait, FriendEventTrait, FriendInfoChangeEventTrait,
    MiraiEventTrait, UserEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.FriendRemarkChangeEvent")]
pub struct FriendRemarkChangeEvent {
    instance: Instance,
}

impl FriendEventTrait for FriendRemarkChangeEvent {}
impl FriendInfoChangeEventTrait for FriendRemarkChangeEvent {}
impl BroadcastControllableTrait for FriendRemarkChangeEvent {}

impl MiraiEventTrait for FriendRemarkChangeEvent {}
impl BotEventTrait for FriendRemarkChangeEvent {}
impl UserEventTrait for FriendRemarkChangeEvent {}
