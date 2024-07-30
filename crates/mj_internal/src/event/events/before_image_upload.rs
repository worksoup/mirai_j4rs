use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BotActiveEventTrait, BotEventTrait, CancellableEventTrait};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct BeforeImageUploadEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> BotEventTrait<B> for BeforeImageUploadEvent<B> {}

impl<B: BotBackend> BotActiveEventTrait<B> for BeforeImageUploadEvent<B> {}
impl<B: BotBackend> CancellableEventTrait<B> for BeforeImageUploadEvent<B> {}
