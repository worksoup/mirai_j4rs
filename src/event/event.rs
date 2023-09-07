use contact_derive::GetInstanceDerive;
use j4rs::{prelude::*, InvocationArg};
use j4rs::{Instance, Jvm};
use j4rs_derive::*;
use std::mem::transmute;

use crate::env::GetClassTypeTrait;
use crate::event::event_trait::MiraiEventTrait;

//需要由Env构造。
pub struct EventChannel<E>
    where
        E: MiraiEventTrait,
{
    pub(crate) jvm: Jvm,
    pub(crate) instance: Instance,
    pub(crate) _unused: Option<E>,
}

#[call_from_java("rt.lea.Lumia.onEvent")]
fn apply_on_event(on_event_ptr: Instance, event: Instance) {
    let on_event_raw: [i8; 16] = Jvm::attach_thread().unwrap().to_rust(on_event_ptr).unwrap();
    println!("rust side 2");
    println!("{:?}", on_event_raw);
    let on_event: *mut dyn Fn(AbstractEvent) -> () = unsafe { transmute(on_event_raw) };
    unsafe {
        let _ = (*on_event)(AbstractEvent::from_instance(event));
    };
}

impl<'a, E> EventChannel<E>
    where
        E: MiraiEventTrait,
{
    fn subscribe_internal(
        &self,
        call_from_java_raw_list: [i8; 16],
    ) -> (Instance, Instance, [i8; 16]) {
        println!("rust side 1");
        println!("{:?}", call_from_java_raw_list);
        let mut on_event_ptr = Vec::new();
        for i in call_from_java_raw_list {
            on_event_ptr.push(InvocationArg::try_from(i).unwrap());
        }
        let class_type = E::get_class_type();
        let on_event_ptr = self
            .jvm
            .create_java_array("java.lang.Byte", &on_event_ptr)
            .unwrap();
        let consumer = Jvm::attach_thread()
            .unwrap()
            .create_instance(
                "rt.lea.Lumia",
                &[InvocationArg::try_from(on_event_ptr).unwrap()],
            )
            .unwrap();
        (class_type, consumer, call_from_java_raw_list)
    }
    fn subscribe_internal_0_1(on_event: &'a Box<dyn Fn(E) -> ()>) -> [i8; 16] {
        let call_from_java: Box<dyn Fn(AbstractEvent) -> ()> = Box::new(|e: AbstractEvent| {
            let e: E = e.get::<E>();
            on_event(e);
        });
        let call_from_java_raw: *mut dyn Fn(AbstractEvent) = Box::into_raw(call_from_java);
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    fn subscribe_internal_0_2(on_event: Box<dyn FnOnce(E) -> ()>) -> [i8; 16] {
        let call_from_java: Box<dyn FnOnce(AbstractEvent) -> ()> =
            Box::new(move |e: AbstractEvent| {
                let e: E = e.get::<E>();
                on_event(e);
            });
        let call_from_java_raw: *mut dyn FnOnce(AbstractEvent) = Box::into_raw(call_from_java);
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    fn subscribe_internal_1_1(
        &self,
        on_event: &Box<dyn Fn(E) -> ()>,
    ) -> (Instance, Instance, [i8; 16]) {
        let call_from_java_raw_list = Self::subscribe_internal_0_1(on_event);
        self.subscribe_internal(call_from_java_raw_list)
    }
    fn subscribe_internal_1_2(
        &self,
        on_event: Box<dyn FnOnce(E) -> ()>,
    ) -> (Instance, Instance, [i8; 16]) {
        let call_from_java_raw_list = Self::subscribe_internal_0_2(on_event);
        self.subscribe_internal(call_from_java_raw_list)
    }
    pub fn subscribe(&'a self, on_event: &'a Box<dyn Fn(E) -> ()>) -> Listener<E> {
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
    pub fn subscribe_always(&'a self, on_event: &'a Box<dyn Fn(E) -> ()>) -> Listener<E> {
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
    pub fn subscribe_once(&self, on_event: Box<dyn FnOnce(E) -> ()>) -> Listener<E> {
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
        todo!()
    }
    pub fn filter(&self) -> Self {
        todo!()
    }
}


pub enum OnEvent<'a, E> {
    Fn(&'a Box<dyn Fn(E)>),
    // 此处需要值，确保引用有效，值不会被 drop.
    FnOnce, // 此处不需要值，因为值已经移动到下方 Listener 中 call_from_java 这个指针所代表的值里了。
}

pub struct Listener<'a, E> {
    instance: Instance,
    call_from_java: [i8; 16],
    _on_event: OnEvent<'a, E>,
}

impl<E> Listener<'_, E> {
    pub fn cancel(self) {
        todo!()
    }
    pub fn complete(self) -> bool {
        let call_from_java: *mut dyn Fn(AbstractEvent) -> () =
            unsafe { transmute(self.call_from_java) };
        let call_from_java = unsafe { Box::from_raw(call_from_java) };
        drop(call_from_java);
        let jvm = Jvm::attach_thread().unwrap();
        let b = jvm.invoke(&self.instance, "complete", &[]).unwrap();
        jvm.to_rust(b).unwrap()
    }
}

#[derive(GetInstanceDerive)]
pub struct AbstractEvent {
    instance: Instance,
}

impl AbstractEvent {
    pub fn get<E>(&self) -> E
        where
            E: MiraiEventTrait,
    {
        E::from_instance(
            Jvm::attach_thread()
                .unwrap()
                .clone_instance(&self.instance)
                .unwrap(),
        )
    }
}

impl GetClassTypeTrait for AbstractEvent {
    fn get_class_type() -> Instance {
        panic!("本 api 不应当使用。")
    }
}

impl MiraiEventTrait for AbstractEvent {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }

    fn cancel(&self) {
        todo!()
    }

    fn intercept(&self) {
        todo!()
    }

    fn is_canceled(&self) -> bool {
        todo!()
    }

    fn is_intercepted(&self) -> bool {
        todo!()
    }

    fn broadcast(&self) {
        todo!()
    }
}
