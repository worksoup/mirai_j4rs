use crate::message::data::message_chain::MessageChain;
use crate::message::data::message_source::MessageSource;
use crate::message::message_trait::{
    ConstrainSingleTrait, MessageHashCodeTrait, MessageMetaDataTrait, MessageTrait,
    SingleMessageTrait,
};
use j4rs::{Instance, InvocationArg, Jvm};
use mjbase::env::{FromInstance, GetEnvTrait};
use mjmacro::GetInstanceDerive;

#[derive(GetInstanceDerive)]
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
        let instance = jvm.invoke(&self.instance, "getSource", &[]).unwrap();
        MessageSource::from_instance(instance)
    }
}

impl From<MessageChain> for QuoteReply {
    fn from(source_message: MessageChain) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                "net.mamoe.mirai.message.data.QuoteReply",
                &[InvocationArg::try_from(source_message.get_instance()).unwrap()],
            )
            .unwrap();
        Self { instance }
    }
}

impl FromInstance for QuoteReply {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }
}
