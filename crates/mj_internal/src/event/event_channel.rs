use std::intrinsics::transmute;

use j4rs::{Instance, InvocationArg, Jvm};

use mj_base::data_wrapper::DataWrapper;
use mj_base::MIRAI_PREFIX;

use crate::event::{Listener, MiraiEventTrait, OnEvent};

//需要由Env构造。
pub struct EventChannel {
    pub(crate) jvm: Jvm,
    pub(crate) instance: Instance,
}

impl EventChannel {
    //默认是global的。
    pub fn global() -> EventChannel {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = Jvm::attach_thread()
            .unwrap()
            .static_class((MIRAI_PREFIX.to_string() + "event.GlobalEventChannel$INSTANCE").as_str())
            .unwrap();
        EventChannel { jvm, instance }
    }
    fn subscribe_internal<E: MiraiEventTrait>(
        &self,
        call_from_java_raw_list: &[i8; 16],
    ) -> Instance {
        println!("rust side 1");
        println!("{:?}", call_from_java_raw_list);
        let mut on_event_ptr = Vec::new();
        for i in call_from_java_raw_list {
            on_event_ptr.push(InvocationArg::try_from(i).unwrap());
        }
        let on_event_ptr = self
            .jvm
            .create_java_array("java.lang.Byte", &on_event_ptr)
            .unwrap();
        let consumer = Jvm::attach_thread()
            .unwrap()
            .create_instance(
                "rt.lea.LumiaConsumer",
                &[InvocationArg::try_from(on_event_ptr).unwrap()],
            )
            .unwrap();
        consumer
    }
    fn subscribe_internal_0_1<'a, E: MiraiEventTrait>(
        on_event: &'a Box<dyn Fn(E) -> ()>,
    ) -> [i8; 16] {
        let call_from_java: Box<dyn Fn(DataWrapper<Instance>) -> ()> =
            Box::new(|e: DataWrapper<Instance>| {
                let e: E = e.get::<E>();
                on_event(e);
            });
        let call_from_java_raw: *mut dyn Fn(DataWrapper<Instance>) = Box::into_raw(call_from_java);
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    fn subscribe_internal_0_2<E: MiraiEventTrait>(on_event: Box<dyn FnOnce(E) -> ()>) -> [i8; 16] {
        let call_from_java: Box<dyn FnOnce(DataWrapper<Instance>) -> ()> =
            Box::new(move |e: DataWrapper<Instance>| {
                let e: E = e.get::<E>();
                on_event(e);
            });
        let call_from_java_raw: *mut dyn FnOnce(DataWrapper<Instance>) =
            Box::into_raw(call_from_java);
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    fn subscribe_internal_1_1<E: MiraiEventTrait>(
        &self,
        on_event: &Box<dyn Fn(E) -> ()>,
    ) -> (Instance, Instance, [i8; 16]) {
        let call_from_java_raw_list = Self::subscribe_internal_0_1(on_event);
        (
            E::get_class_type(),
            self.subscribe_internal::<E>(&call_from_java_raw_list),
            call_from_java_raw_list,
        )
    }
    fn subscribe_internal_1_2<E: MiraiEventTrait>(
        &self,
        on_event: Box<dyn FnOnce(E) -> ()>,
    ) -> (Instance, Instance, [i8; 16]) {
        let call_from_java_raw_list = Self::subscribe_internal_0_2(on_event);
        (
            E::get_class_type(),
            self.subscribe_internal::<E>(&call_from_java_raw_list),
            call_from_java_raw_list,
        )
    }
    pub fn subscribe<'a, E: MiraiEventTrait>(
        &'a self,
        on_event: &'a Box<dyn Fn(E) -> ()>,
    ) -> Listener<E> {
        let (class_type, consumer, call_from_java) = self.subscribe_internal_1_1(on_event);
        let listener = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "subscribe",
                &[
                    InvocationArg::try_from(class_type).unwrap(),
                    InvocationArg::try_from(consumer).unwrap(),
                ],
            )
            .unwrap();
        Listener {
            instance: listener,
            call_from_java,
            _on_event: OnEvent::Fn(on_event),
        }
    }
    pub fn subscribe_always<'a, E: MiraiEventTrait>(
        &'a self,
        on_event: &'a Box<dyn Fn(E) -> ()>,
    ) -> Listener<E> {
        let (class_type, consumer, call_from_java) = self.subscribe_internal_1_1(on_event);
        let listener = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "subscribeAlways",
                &[
                    InvocationArg::try_from(class_type).unwrap(),
                    InvocationArg::try_from(consumer).unwrap(),
                ],
            )
            .unwrap();
        Listener {
            instance: listener,
            call_from_java,
            _on_event: OnEvent::Fn(on_event),
        }
    }
    pub fn subscribe_once<E: MiraiEventTrait>(
        &self,
        on_event: Box<dyn FnOnce(E) -> ()>,
    ) -> Listener<E> {
        let (class_type, consumer, call_from_java) = self.subscribe_internal_1_2(on_event);
        let listener = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "subscribeOnce",
                &[
                    InvocationArg::try_from(class_type).unwrap(),
                    InvocationArg::try_from(consumer).unwrap(),
                ],
            )
            .unwrap();
        Listener {
            instance: listener,
            call_from_java,
            _on_event: OnEvent::FnOnce,
        }
    }
    pub fn exception_handler(&self) -> Self {
        todo!("exception_handler")
    }
    pub fn filter(&self) -> Self {
        todo!("filter")
    }
}
