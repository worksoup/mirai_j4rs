use j4rs::Instance;

use mj_helper_macro::mj_all;

use crate::contact::{ContactOrBotTrait, ContactTrait, SendMessageSupportedTrait};
use crate::utils::backend::BotBackend;

#[mj_all("contact.OtherClient")]
pub struct OtherClient<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> ContactOrBotTrait<B> for OtherClient<B> {}

impl<B: BotBackend> ContactTrait<B> for OtherClient<B> {}

impl<B: BotBackend> SendMessageSupportedTrait<B> for OtherClient<B> {}
