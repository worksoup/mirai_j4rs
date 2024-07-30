use crate::event::BotEventTrait;
use crate::utils::backend::BotBackend;

pub trait UserEventTrait<B: BotBackend>: BotEventTrait<B> {}
