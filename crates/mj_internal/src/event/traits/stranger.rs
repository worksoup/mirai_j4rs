use crate::event::{BotEventTrait, UserEventTrait};
use crate::utils::backend::BotBackend;

pub trait StrangerEventTrait<B: BotBackend>: BotEventTrait<B> + UserEventTrait<B> {}
