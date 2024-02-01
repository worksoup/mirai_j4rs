use crate::message::message_trait::{
    CodableMessageTrait, MessageContentTrait, MessageHashCodeTrait, MessageTrait,
    SingleMessageTrait,
};
use j4rs::{Instance, Jvm};
use mj_macro::{FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, FromInstanceDerive)]
pub struct AtAll {
    instance: Instance,
}

impl AtAll {
    pub fn new() -> AtAll {
        let instance = Jvm::attach_thread()
            .unwrap()
            .static_class("net.mamoe.mirai.message.data.AtAll$INSTANCE")
            .unwrap();
        AtAll { instance }
    }
}

impl MessageTrait for AtAll {
    fn to_content(&self) -> String {
        "@全体成员".to_string()
    }
    fn to_string(&self) -> String {
        "[mirai:at all]".to_string()
    }
}

impl CodableMessageTrait for AtAll {
    fn to_code(&self) -> String {
        self.to_string()
    }
}

impl SingleMessageTrait for AtAll {}

impl MessageContentTrait for AtAll {}

impl MessageHashCodeTrait for AtAll {
    /// "@全体成员".hashCode()
    fn hash_code(&self) -> i32 {
        700264627
    }
}
