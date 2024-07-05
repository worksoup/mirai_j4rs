use j4rs::{Instance, InvocationArg, Jvm};

use mj_macro::mj_all;

use crate::message::message_trait::{
    CodableMessageTrait, MessageContentTrait, MessageHashCodeTrait, MessageTrait,
    SingleMessageTrait,
};

#[mj_all("message.data.AtAll")]
pub struct AtAll {
    instance: Instance,
}

impl AtAll {
    pub fn new() -> AtAll {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static("rt.lea.LumiaUtils", "getAtAll", InvocationArg::empty())
            .unwrap();
        AtAll { instance }
    }
    pub fn get_display() -> String {
        "@全体成员".into()
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
