use crate::event::{BotEventTrait, UserEventTrait};
use crate::utils::backend::BotBackend;

pub trait FriendEventTrait<B: BotBackend>: BotEventTrait<B> + UserEventTrait<B> {}
