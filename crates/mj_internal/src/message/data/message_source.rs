use j4rs::{Instance, InvocationArg, Jvm};
use j4rs::errors::J4RsError;
use mj_base::env::{GetClassTypeTrait, GetInstanceTrait};
use mj_helper_macro::mj_all;

use crate::message::message_trait::{ConstrainSingleTrait, MessageTrait, SingleMessageTrait};

// TODO
#[mj_all("message.data.MessageSource")]
pub struct MessageSource {
    instance: Instance,
}

impl MessageTrait for MessageSource {}

impl SingleMessageTrait for MessageSource {}

impl ConstrainSingleTrait for MessageSource {}

impl Clone for MessageSource {
    fn clone(&self) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                <Self as GetClassTypeTrait>::get_type_name().as_str(), // TODO: 之前这里是引用回复的类名，请检查是否出错。
                &[InvocationArg::try_from(self.get_instance()).unwrap()],
            )
            .unwrap();
        Self { instance }
    }
}
