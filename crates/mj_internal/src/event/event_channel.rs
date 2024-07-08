use crate::event::{Listener, MiraiEventTrait};
use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::Consumer;
use jbuchong::GetInstanceTrait;
use mj_base::MIRAI_PREFIX;

//需要由Env构造。
pub struct EventChannel {
    pub(crate) instance: Instance,
}

impl EventChannel {
    //默认是global的。
    pub fn global() -> EventChannel {
        let instance = Jvm::attach_thread()
            .unwrap()
            .static_class((MIRAI_PREFIX.to_string() + "event.GlobalEventChannel$INSTANCE").as_str())
            .unwrap();
        EventChannel { instance }
    }
    pub fn subscribe<E: MiraiEventTrait>(
        &self,
        on_event: impl Fn(E) -> () + 'static,
    ) -> Listener<E> {
        let class_type = E::get_class_type();
        let consumer = Consumer::new(on_event);
        let listener = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "subscribe",
                &[
                    InvocationArg::try_from(class_type).unwrap(),
                    InvocationArg::try_from(consumer.get_instance().unwrap()).unwrap(),
                ],
            )
            .unwrap();
        Listener {
            instance: listener,
            consumer,
        }
    }
    pub fn subscribe_always<E: MiraiEventTrait>(
        &self,
        on_event: impl Fn(E) -> () + 'static,
    ) -> Listener<E> {
        // TODO
        self.subscribe(on_event)
    }
    pub fn subscribe_once<E: MiraiEventTrait>(
        &self,
        on_event: impl Fn(E) -> () + 'static,
    ) -> Listener<E> {
        // TODO
        self.subscribe(on_event)
    }
    pub fn exception_handler(&self) -> Self {
        todo!("exception_handler")
    }
    pub fn filter(&self) -> Self {
        todo!("filter")
    }
}
