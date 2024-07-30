use j4rs::Instance;

use mj_helper_macro::mj_all;

use crate::message::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MessageContentTrait, MessageTrait,
    SingleMessageTrait,
};
use crate::utils::backend::BotBackend;

// TODO: 低优先级
#[mj_all("message.data.VipFace")]
pub struct VipFace<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> MessageTrait<B> for VipFace<B> {}

impl<B: BotBackend> SingleMessageTrait<B> for VipFace<B> {}

impl<B: BotBackend> MessageContentTrait<B> for VipFace<B> {}

impl<B: BotBackend> ConstrainSingleTrait<B> for VipFace<B> {}

impl<B: BotBackend> CodableMessageTrait<B> for VipFace<B> {}
