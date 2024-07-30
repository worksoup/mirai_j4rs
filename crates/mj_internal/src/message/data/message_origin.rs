use crate::message::{
    ConstrainSingleTrait, MessageMetaDataTrait, MessageTrait, SingleMessageTrait,
};
use crate::utils::backend::BotBackend;
use j4rs::Instance;
use mj_helper_macro::mj_all;

#[mj_all("message.data.MessageOrigin")]
pub struct MessageOrigin <B: BotBackend>{
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> MessageMetaDataTrait<B> for MessageOrigin<B> {}
impl<B: BotBackend> ConstrainSingleTrait<B> for MessageOrigin<B> {}
impl<B: BotBackend> SingleMessageTrait<B> for MessageOrigin<B> {}
impl<B: BotBackend> MessageTrait<B> for MessageOrigin<B> {}
