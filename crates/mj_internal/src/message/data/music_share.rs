use j4rs::Instance;

use mj_helper_macro::mj_all;

use crate::message::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MessageContentTrait, MessageTrait,
    SingleMessageTrait,
};
use crate::utils::backend::BotBackend;

#[mj_all("message.data.MusicShare")]
pub struct MusicShare<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> MessageTrait<B> for MusicShare<B> {}

impl<B: BotBackend> SingleMessageTrait<B> for MusicShare<B> {}

impl<B: BotBackend> MessageContentTrait<B> for MusicShare<B> {}

impl<B: BotBackend> ConstrainSingleTrait<B> for MusicShare<B> {}

impl<B: BotBackend> CodableMessageTrait<B> for MusicShare<B> {}
