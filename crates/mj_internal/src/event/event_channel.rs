use crate::event::{Listener, MiraiEventTrait};
use crate::utils::backend::BotBackend;
use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{java_all, GetInstanceTrait};
use jbuchong::{Consumer, FromInstanceTrait};
use mj_base::MIRAI_PREFIX;

//需要由Env构造。
#[java_all]
pub struct EventChannel<B: BotBackend> {
    pub(crate) instance: Instance,
    _backend: B,
}

impl<B: BotBackend> EventChannel<B> {
    //默认是global的。
    pub fn global() -> Self {
        let instance = Jvm::attach_thread()
            .unwrap()
            .static_class((MIRAI_PREFIX.to_string() + "event.GlobalEventChannel$INSTANCE").as_str())
            .unwrap();
        Self::from_instance(instance)
    }
    pub fn subscribe<E: MiraiEventTrait<B>>(&self, on_event: impl Fn(E) + 'static) -> Listener<E> {
        let class_type = E::get_class_type();
        let consumer = Consumer::new(on_event);
        let listener = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "subscribe",
                &[
                    InvocationArg::from(class_type),
                    InvocationArg::from(consumer.get_instance().unwrap()),
                ],
            )
            .unwrap();
        Listener {
            instance: listener,
            consumer,
        }
    }
    pub fn subscribe_always<E: MiraiEventTrait<B>>(
        &self,
        on_event: impl Fn(E) + 'static,
    ) -> Listener<E> {
        // TODO
        let class_type = E::get_class_type();
        let consumer = Consumer::new(on_event);
        let listener = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "subscribeAlways",
                &[
                    InvocationArg::from(class_type),
                    InvocationArg::from(consumer.get_instance().unwrap()),
                ],
            )
            .unwrap();
        Listener {
            instance: listener,
            consumer,
        }
    }
    pub fn subscribe_once<E: MiraiEventTrait<B>>(
        &self,
        on_event: impl Fn(E) + 'static,
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
