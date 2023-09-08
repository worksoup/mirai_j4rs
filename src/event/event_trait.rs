use crate::contact::bot::Bot;
use crate::env::{GetClassTypeTrait, GetEnvTrait};
use j4rs::{Instance, Jvm};

pub trait MiraiEventTrait
    where
        Self: GetEnvTrait + GetClassTypeTrait,
{
    fn from_instance(instance: Instance) -> Self;
    fn cancel(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.invoke(&self.get_instance(), "cancel", &[]).unwrap();
    }
    fn intercept(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.invoke(&self.get_instance(), "intercept", &[]).unwrap();
    }
    fn is_canceled(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.get_instance(), "isCanceled", &[]).unwrap())
            .unwrap()
    }
    fn is_intercepted(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.get_instance(), "isIntercepted", &[]).unwrap())
            .unwrap()
    }
    // TODO: 这个函数哪来的？为什么在最初的版本中？
    fn broadcast(&self) { todo!("什么也不做，也请先不要调用此函数") }
}

pub trait BotEventTrait
    where
        Self: MiraiEventTrait,
{
    fn get_bot(&self) -> Bot;
}

pub trait BotOfflineEventTrait {}
