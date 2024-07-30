use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{GetClassTypeTrait, GetInstanceTrait};
use mj_helper_macro::mj_all;

use crate::message::message_trait::{ConstrainSingleTrait, MessageTrait, SingleMessageTrait};
use crate::utils::backend::BotBackend;

// TODO
#[mj_all("message.data.MessageSource")]
pub struct MessageSource<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> MessageTrait<B> for MessageSource<B> {}

impl<B: BotBackend> SingleMessageTrait<B> for MessageSource<B> {}

impl<B: BotBackend> ConstrainSingleTrait<B> for MessageSource<B> {}

impl<B: BotBackend> Clone for MessageSource<B> {
    fn clone(&self) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                <Self as GetClassTypeTrait>::get_type_name(), // TODO: 之前这里是引用回复的类名，请检查是否出错。
                &[InvocationArg::try_from(self.get_instance()).unwrap()],
            )
            .unwrap();
        Self {
            instance,
            _backend: B::default(),
        }
    }
}
