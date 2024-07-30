use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BotEventTrait, BotPassiveEventTrait};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct BotGroupPermissionChangeEvent <B: BotBackend>{
    instance: Instance,
    _backend: B,
}

impl <B: BotBackend>BotEventTrait<B> for BotGroupPermissionChangeEvent<B> {}

impl<B: BotBackend> BotPassiveEventTrait<B> for BotGroupPermissionChangeEvent<B> {}
