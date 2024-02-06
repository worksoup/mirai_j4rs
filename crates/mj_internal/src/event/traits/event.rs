use j4rs::Jvm;
use mj_base::env::{FromInstance, GetClassTypeTrait, GetInstanceTrait};

pub trait MiraiEventTrait
    where
        Self: GetInstanceTrait + GetClassTypeTrait + FromInstance,
{
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
        jvm.to_rust(
            jvm.invoke(&self.get_instance(), "isIntercepted", &[])
                .unwrap(),
        )
            .unwrap()
    }
    /// 广播一个事件。
    fn broadcast(&self) {
        todo!("参见 EventKt")
    }
}