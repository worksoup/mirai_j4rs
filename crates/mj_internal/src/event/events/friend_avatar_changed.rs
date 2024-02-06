use j4rs::Instance;

use mj_macro::mj_event;

use crate::event::{BotEventTrait, FriendEventTrait, UserEventTrait};

#[mj_event]
pub struct FriendAvatarChangedEvent {
    instance: Instance,
}
impl BotEventTrait for FriendAvatarChangedEvent {}

impl UserEventTrait for FriendAvatarChangedEvent {}

impl FriendEventTrait for FriendAvatarChangedEvent {}
