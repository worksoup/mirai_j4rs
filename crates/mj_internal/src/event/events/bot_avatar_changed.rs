use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::BotEventTrait;
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct BotAvatarChangedEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}
impl <B: BotBackend> BotEventTrait<B> for BotAvatarChangedEvent<B> {}
