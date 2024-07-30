use j4rs::{Instance, InvocationArg, Jvm};
use mj_helper_macro::mj_all;

use crate::message::message_trait::{
    CodableMessageTrait, MessageContentTrait, MessageHashCodeTrait, MessageTrait, RichMessageTrait,
    SingleMessageTrait,
};
use crate::utils::backend::BotBackend;

#[mj_all("message.data.LightApp")]
pub struct LightApp <B: BotBackend>{
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> LightApp<B> {
    pub fn get_content(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getContent", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
}

impl<B: BotBackend> MessageHashCodeTrait for LightApp<B> {}

impl<B: BotBackend> MessageTrait<B> for LightApp<B> {}

impl<B: BotBackend> SingleMessageTrait<B> for LightApp<B> {}

impl<B: BotBackend> MessageContentTrait<B> for LightApp<B> {}

impl<B: BotBackend> RichMessageTrait<B> for LightApp<B> {}

impl<B: BotBackend> CodableMessageTrait<B> for LightApp<B> {}
