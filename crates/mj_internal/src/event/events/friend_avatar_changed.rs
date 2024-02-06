use crate::event::{BotEventTrait, FriendEventTrait, MiraiEventTrait, UserEventTrait};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.FriendAvatarChangedEvent")]
pub struct FriendAvatarChangedEvent {
    instance: Instance,
}
impl BotEventTrait for FriendAvatarChangedEvent {}

impl UserEventTrait for FriendAvatarChangedEvent {}

impl FriendEventTrait for FriendAvatarChangedEvent {}

impl MiraiEventTrait for FriendAvatarChangedEvent {}
