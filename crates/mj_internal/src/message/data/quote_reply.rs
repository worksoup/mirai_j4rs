use j4rs::{Instance, InvocationArg, Jvm};

use mj_base::env::{FromInstanceTrait, GetClassTypeTrait, GetInstanceTrait};
use mj_macro::mj_all;

use crate::message::{
    data::{message_chain::MessageChain, message_source::MessageSource},
    message_trait::{
        ConstrainSingleTrait, MessageHashCodeTrait, MessageMetaDataTrait, MessageTrait,
        SingleMessageTrait,
    },
};

#[mj_all("message.data.QuoteReply")]
pub struct QuoteReply {
    instance: Instance,
}

impl MessageTrait for QuoteReply {}
impl MessageMetaDataTrait for QuoteReply {}

impl SingleMessageTrait for QuoteReply {}

impl ConstrainSingleTrait for QuoteReply {}

impl MessageHashCodeTrait for QuoteReply {}

impl QuoteReply {
    pub fn get_source(&self) -> MessageSource {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getSource", InvocationArg::empty())
            .unwrap();
        MessageSource::from_instance(instance)
    }
}

impl From<MessageChain> for QuoteReply {
    fn from(source_message: MessageChain) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                <Self as GetClassTypeTrait>::get_type_name().as_str(),
                &[InvocationArg::try_from(source_message.get_instance()).unwrap()],
            )
            .unwrap();
        Self { instance }
    }
}
