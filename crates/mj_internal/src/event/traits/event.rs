use j4rs::InvocationArg;

use crate::utils::backend::BotBackend;
use jbuchong::{AsInstanceTrait, GetClassTypeTrait, GetInstanceTrait, TryFromInstanceTrait};
use mj_helper_macro::java_fn;

pub trait MiraiEventTrait<B: BotBackend>
where
    Self: GetInstanceTrait + GetClassTypeTrait + TryFromInstanceTrait + AsInstanceTrait,
{
    #[java_fn]
    fn cancel(&self) {}
    #[java_fn]
    fn intercept(&self) {}
    #[java_fn]
    fn is_canceled(&self) -> bool {}
    #[java_fn]
    fn is_intercepted(&self) -> bool {}
    /// 广播一个事件。
    fn broadcast(&self) {
        todo!("参见 EventKt")
    }
}
