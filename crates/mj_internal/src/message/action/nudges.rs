use std::marker::PhantomData;

use j4rs::{Instance, InvocationArg, Jvm};

use mj_base::env::{AsInstanceTrait, FromInstanceTrait, GetInstanceTrait};
use mj_macro::{AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

use crate::{
    contact::{ContactTrait, UserOrBotTrait},
    message::message_trait::MessageHashCodeTrait,
};

pub trait NudgeTrait<UserOrBot: UserOrBotTrait>:
    GetInstanceTrait + MessageHashCodeTrait + FromInstanceTrait + AsInstanceTrait
{
    fn get_target(&self) -> UserOrBot {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(self.as_instance(), "getTarget", &[]).unwrap();
        UserOrBot::from_instance(instance)
    }
    // TODO: 该函数不符合 Mirai 定义的位置。到时候用 rust 标准库里的特征看看能不能实现一下。
    fn to_string(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(self.as_instance(), "toString", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
    // TODO: 该函数不符合 Mirai 定义的位置。
    fn equals(&self) -> bool {
        todo!("低优先级。")
    }
    fn send_to(&self, receiver: impl ContactTrait) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(
                self.as_instance(),
                "sendTo",
                &[InvocationArg::try_from(receiver.get_instance()).unwrap()],
            )
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
}
#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
pub struct Nudge<UserOrBot: UserOrBotTrait> {
    instance: Instance,
    _u: PhantomData<UserOrBot>,
}
impl<UserOrBot: UserOrBotTrait> MessageHashCodeTrait for Nudge<UserOrBot> {}
