use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BotEventTrait, StrangerEventTrait, UserEventTrait};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct StrangerRelationChangeEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}
impl<B:BotBackend> StrangerEventTrait<B> for StrangerRelationChangeEvent<B> {}
impl<B:BotBackend> BotEventTrait<B> for StrangerRelationChangeEvent<B> {}
impl<B:BotBackend> UserEventTrait<B> for StrangerRelationChangeEvent<B> {}
