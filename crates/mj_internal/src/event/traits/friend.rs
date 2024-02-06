use crate::event::{BotEventTrait, UserEventTrait};

pub trait FriendEventTrait: BotEventTrait + UserEventTrait {}
