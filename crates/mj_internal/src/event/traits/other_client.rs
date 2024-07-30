use crate::event::BotEventTrait;
use crate::utils::backend::BotBackend;

pub trait OtherClientEventTrait<B: BotBackend>: BotEventTrait<B> {}
