use j4rs::Instance;

use mj_macro::mj_event;

use crate::event::{
    BotEventTrait, BroadcastControllableTrait, FriendEventTrait, FriendInfoChangeEventTrait,
    UserEventTrait,
};

#[mj_event]
pub struct FriendRemarkChangeEvent {
    instance: Instance,
}

impl FriendEventTrait for FriendRemarkChangeEvent {}
impl FriendInfoChangeEventTrait for FriendRemarkChangeEvent {}
impl BroadcastControllableTrait for FriendRemarkChangeEvent {}

impl BotEventTrait for FriendRemarkChangeEvent {}
impl UserEventTrait for FriendRemarkChangeEvent {}
