use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BotEventTrait, StrangerEventTrait, UserEventTrait};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct StrangerAddEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B:BotBackend> BotEventTrait<B> for StrangerAddEvent<B> {}

impl<B:BotBackend> UserEventTrait<B> for StrangerAddEvent<B> {}

impl<B:BotBackend> StrangerEventTrait<B> for StrangerAddEvent<B> {}
