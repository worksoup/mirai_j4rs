use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::Consumer;

pub enum OnEvent<'a, E> {
    Fn(&'a Box<dyn Fn(E)>),
    // 此处需要值，确保引用有效，值不会被 drop.
    FnOnce, // 此处不需要值，因为值已经移动到下方 Listener 中 call_from_java 这个指针所代表的值里了。
}

pub struct Listener<E> {
    pub(crate) instance: Instance,
    pub(crate) consumer: Consumer<E>,
}

impl<E> Listener<E> {
    // 这个函数暂不实现。
    pub fn cancel(self) {
        todo!("低优先级：cancel")
    }
    pub fn complete(self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let b = jvm
            .invoke(&self.instance, "complete", InvocationArg::empty())
            .unwrap();
        self.consumer.drop();
        let r = jvm.to_rust(b).unwrap();
        r
    }
}
