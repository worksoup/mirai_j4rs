use std::intrinsics::transmute;

use j4rs::{Instance, Jvm};

use mj_base::data_wrapper::DataWrapper;

pub enum OnEvent<'a, E> {
    Fn(&'a Box<dyn Fn(E)>),
    // 此处需要值，确保引用有效，值不会被 drop.
    FnOnce, // 此处不需要值，因为值已经移动到下方 Listener 中 call_from_java 这个指针所代表的值里了。
}

pub struct Listener<'a, E> {
    pub(crate) instance: Instance,
    pub(crate) call_from_java: [i8; 16],
    pub(crate) _on_event: OnEvent<'a, E>,
}

impl<E> Listener<'_, E> {
    // 这个函数暂不实现。
    pub fn cancel(self) {
        todo!("低优先级：cancel")
    }
    pub fn complete(self) -> bool {
        let call_from_java: *mut dyn Fn(DataWrapper<Instance>) -> () =
            unsafe { transmute(self.call_from_java) };
        let call_from_java = unsafe { Box::from_raw(call_from_java) };
        drop(call_from_java);
        let jvm = Jvm::attach_thread().unwrap();
        let b = jvm.invoke(&self.instance, "complete", &[]).unwrap();
        jvm.to_rust(b).unwrap()
    }
}
