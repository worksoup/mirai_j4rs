use crate::message::message_trait::{ConstrainSingleTrait, MessageTrait, SingleMessageTrait};
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::{FromInstance, GetInstanceTrait};
use mj_macro::{GetInstanceDerive, java_type};

// TODO
#[derive(GetInstanceDerive)]
#[java_type("net.mamoe.mirai.message.data.MessageSource")]
pub struct MessageSource {
    instance: Instance,
}

impl MessageTrait for MessageSource {}

impl SingleMessageTrait for MessageSource {}

impl ConstrainSingleTrait for MessageSource {}

impl FromInstance for MessageSource {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }
}

impl Clone for MessageSource {
    fn clone(&self) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                "net.mamoe.mirai.message.data.QuoteReply",
                &[InvocationArg::try_from(self.get_instance()).unwrap()],
            )
            .unwrap();
        Self { instance }
    }
}
