use crate::message::message_trait::{
    CodableMessageTrait, MessageContentTrait, MessageHashCodeTrait, MessageTrait, RichMessageTrait,
    SingleMessageTrait,
};
use j4rs::{Instance, Jvm};
use mjmacro::{FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, FromInstanceDerive)]
pub struct LightApp {
    instance: Instance,
}

impl LightApp {
    pub fn get_content(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getContent", &[]).unwrap())
            .unwrap()
    }
}

impl MessageHashCodeTrait for LightApp {}

impl MessageTrait for LightApp {}

impl SingleMessageTrait for LightApp {}

impl MessageContentTrait for LightApp {}

impl RichMessageTrait for LightApp {}

impl CodableMessageTrait for LightApp {}
