use j4rs::{Instance, InvocationArg, Jvm};
use j4rs::errors::J4RsError;
use mj_helper_macro::mj_all;

use crate::message::message_trait::{
    CodableMessageTrait, MessageContentTrait, MessageHashCodeTrait, MessageTrait, RichMessageTrait,
    SingleMessageTrait,
};

#[mj_all("message.data.LightApp")]
pub struct LightApp {
    instance: Instance,
}

impl LightApp {
    pub fn get_content(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getContent", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
}

impl MessageHashCodeTrait for LightApp {}

impl MessageTrait for LightApp {}

impl SingleMessageTrait for LightApp {}

impl MessageContentTrait for LightApp {}

impl RichMessageTrait for LightApp {}

impl CodableMessageTrait for LightApp {}
