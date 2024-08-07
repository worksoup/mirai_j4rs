use std::marker::PhantomData;

use crate::utils::backend::BotBackend;
use crate::{
    contact::{ContactTrait, UserOrBotTrait},
    message::message_trait::MessageHashCodeTrait,
};
use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{java_all, AsInstanceTrait, GetInstanceTrait, TryFromInstanceTrait};
use mj_helper_macro::java_fn;

pub trait NudgeTrait<B: BotBackend, UserOrBot: UserOrBotTrait<B>>:
    GetInstanceTrait + MessageHashCodeTrait + TryFromInstanceTrait + AsInstanceTrait
{
    #[java_fn]
    fn get_target(&self) -> UserOrBot {}
    // TODO: 该函数不符合 Mirai 定义的位置。到时候用 rust 标准库里的特征看看能不能实现一下。
    fn to_string(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(self.as_instance(), "toString", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
    // TODO: 该函数不符合 Mirai 定义的位置。
    fn equals(&self) -> bool {
        todo!("低优先级。")
    }
    fn send_to(&self, receiver: impl ContactTrait<B>) -> bool {
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
#[java_all]
pub struct Nudge<B: BotBackend, UserOrBot: UserOrBotTrait<B>> {
    instance: Instance,
    _backend: B,
    _u: PhantomData<UserOrBot>,
}
impl<B: BotBackend, UserOrBot: UserOrBotTrait<B>> MessageHashCodeTrait for Nudge<B, UserOrBot> {}
