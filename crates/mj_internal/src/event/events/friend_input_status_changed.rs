use crate::event::{BotEventTrait, FriendEventTrait, MiraiEventTrait, UserEventTrait};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.FriendInputStatusChangedEvent")]
pub struct FriendInputStatusChangedEvent {
    instance: Instance,
}
impl MiraiEventTrait for FriendInputStatusChangedEvent {}

impl BotEventTrait for FriendInputStatusChangedEvent {}

impl UserEventTrait for FriendInputStatusChangedEvent {}

impl FriendEventTrait for FriendInputStatusChangedEvent {}
