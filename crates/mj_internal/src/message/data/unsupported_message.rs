use j4rs::Instance;
use jbuchong::java_all;

use crate::message::message_trait::{MessageContentTrait, MessageTrait, SingleMessageTrait};
use crate::utils::backend::BotBackend;

#[java_all]
pub struct UnsupportedMessage<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> MessageTrait<B> for UnsupportedMessage<B> {}

impl<B: BotBackend> SingleMessageTrait<B> for UnsupportedMessage<B> {}

impl<B: BotBackend> MessageContentTrait<B> for UnsupportedMessage<B> {}
