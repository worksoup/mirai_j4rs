use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BotActiveEventTrait, BotEventTrait};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct BotReloginEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> BotReloginEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for BotReloginEvent<B> {}

impl<B: BotBackend> BotActiveEventTrait<B> for BotReloginEvent<B> {}
