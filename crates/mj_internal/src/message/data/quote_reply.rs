use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{FromInstanceTrait, GetClassTypeTrait, GetInstanceTrait};
use mj_helper_macro::mj_all;

use crate::message::{
    data::{message_chain::MessageChain, message_source::MessageSource},
    message_trait::{
        ConstrainSingleTrait, MessageHashCodeTrait, MessageMetaDataTrait, MessageTrait,
        SingleMessageTrait,
    },
};
use crate::utils::backend::BotBackend;

#[mj_all("message.data.QuoteReply")]
pub struct QuoteReply<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> MessageTrait<B> for QuoteReply<B> {}
impl<B: BotBackend> MessageMetaDataTrait<B> for QuoteReply<B> {}

impl<B: BotBackend> SingleMessageTrait<B> for QuoteReply<B> {}

impl<B: BotBackend> ConstrainSingleTrait<B> for QuoteReply<B> {}

impl<B: BotBackend> MessageHashCodeTrait for QuoteReply<B> {}

impl<B: BotBackend> QuoteReply<B> {
    pub fn get_source(&self) -> MessageSource<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getSource", InvocationArg::empty())
            .unwrap();
        MessageSource::from_instance(instance)
    }
}

impl<B: BotBackend> From<MessageChain<B>> for QuoteReply<B> {
    fn from(source_message: MessageChain<B>) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                <Self as GetClassTypeTrait>::get_type_name(),
                &[InvocationArg::try_from(source_message.get_instance()).unwrap()],
            )
            .unwrap();
        Self {
            instance,
            _backend: B::default(),
        }
    }
}
