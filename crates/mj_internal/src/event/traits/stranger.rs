use crate::event::{BotEventTrait, UserEventTrait};

pub trait StrangerEventTrait: BotEventTrait + UserEventTrait {}
