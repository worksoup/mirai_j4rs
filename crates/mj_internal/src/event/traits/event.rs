use j4rs::{InvocationArg, Jvm};

use jbuchong::{AsInstanceTrait, TryFromInstanceTrait, GetClassTypeTrait, GetInstanceTrait};

pub trait MiraiEventTrait
where
    Self: GetInstanceTrait + GetClassTypeTrait + TryFromInstanceTrait + AsInstanceTrait,
{
    fn cancel(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.invoke(&self.as_instance(), "cancel", InvocationArg::empty())
            .unwrap();
    }
    fn intercept(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.invoke(&self.as_instance(), "intercept", InvocationArg::empty())
            .unwrap();
    }
    fn is_canceled(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.as_instance(), "isCanceled", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    fn is_intercepted(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.as_instance(), "isIntercepted", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    /// 广播一个事件。
    fn broadcast(&self) {
        todo!("参见 EventKt")
    }
}
