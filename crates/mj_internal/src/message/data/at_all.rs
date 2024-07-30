use crate::message::message_trait::{
    CodableMessageTrait, MessageContentTrait, MessageHashCodeTrait, MessageTrait,
    SingleMessageTrait,
};
use crate::utils::backend::BotBackend;
use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::FromInstanceTrait;
use mj_helper_macro::mj_all;

#[mj_all("message.data.AtAll")]
pub struct AtAll<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> AtAll<B> {
    pub fn new() -> AtAll<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static("rt.lea.LumiaUtils", "getAtAll", InvocationArg::empty())
            .unwrap();
        AtAll::from_instance(instance)
    }
    pub fn get_display() -> String {
        "@全体成员".into()
    }
}
impl<B: BotBackend> Default for AtAll<B> {
    fn default() -> Self {
        Self::new()
    }
}
impl<B: BotBackend> MessageTrait<B> for AtAll<B> {
    fn to_content(&self) -> String {
        "@全体成员".to_string()
    }
    fn to_string(&self) -> String {
        "[mirai:at all]".to_string()
    }
}

impl<B: BotBackend> CodableMessageTrait<B> for AtAll<B> {
    fn to_code(&self) -> String {
        MessageTrait::<B>::to_string(self)
    }
}

impl<B: BotBackend> SingleMessageTrait<B> for AtAll<B> {}

impl<B: BotBackend> MessageContentTrait<B> for AtAll<B> {}

impl<B: BotBackend> MessageHashCodeTrait for AtAll<B> {
    /// "@全体成员".hashCode()
    fn hash_code(&self) -> i32 {
        700264627
    }
}
