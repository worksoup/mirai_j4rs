use crate::env::{FromInstance, GetEnvTrait};
use crate::utils::internal::data_wrapper::DataWrapper;
use crate::utils::internal::instance_from_i8_16;
use j4rs::{prelude::*, Instance, InvocationArg, Jvm};
use j4rs_derive::*;
use std::{marker::PhantomData, mem::transmute, pin::Pin};

#[call_from_java("rt.lea.LumiaConsumer.nativeAccept")]
fn lumia_consumer_accept(consumer_as_i8_16: Instance, arg: Instance) {
    let consumer_raw: [i8; 16] = Jvm::attach_thread()
        .unwrap()
        .to_rust(consumer_as_i8_16)
        .unwrap();
    println!(
        "lumia_consumer_accept, in {}, {}:{}",
        file! {},
        line!(),
        column!()
    );
    println!("consumer_raw: {:?}", consumer_raw);
    let consumer: *mut dyn Fn(DataWrapper<Instance>) -> () = unsafe { transmute(consumer_raw) };
    unsafe {
        let _ = (*consumer)(DataWrapper::from_instance(arg));
    };
}

pub struct Consumer<'a, T, F>
    where
        T: FromInstance,
        F: Fn(T) -> (),
{
    instance: Instance,
    internal_closure_raw: [i8; 16],
    _t: PhantomData<T>,
    _f: PhantomData<&'a F>,
}

impl<'a, T, F> GetEnvTrait for Consumer<'a, T, F>
    where
        T: FromInstance,
        F: Fn(T) -> (),
{
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}

impl<'a, T, F: Fn(T) -> ()> Consumer<'a, T, F>
    where
        T: FromInstance,
{
    #[inline]
    fn internal_closure_as_i8_16(f: &'a F) -> [i8; 16] {
        let call_from_java = |value: DataWrapper<Instance>| {
            let value = value.get::<T>();
            f(value);
        };
        let call_from_java_raw: *mut dyn Fn(DataWrapper<Instance>) =
            Box::into_raw(Box::new(call_from_java));
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    pub fn new(closure: &'a F) -> Pin<Box<Consumer<T, F>>> {
        let internal_closure_raw = Self::internal_closure_as_i8_16(closure);
        println!("closure_to_consumer");
        println!("{:?}", internal_closure_raw);
        let jvm = Jvm::attach_thread().unwrap();
        let instance = instance_from_i8_16::<"rt.lea.LumiaConsumer">(internal_closure_raw);
        Box::pin(Consumer {
            instance,
            internal_closure_raw,
            _t: PhantomData::default(),
            _f: PhantomData::default(),
        })
    }
    pub fn accept(&self, arg: InvocationArg) {
        let jvm = Jvm::attach_thread().unwrap();
        let _ = jvm.invoke(&self.instance, "accept", &[arg]).unwrap();
    }
}

impl<'a, T, F: Fn(T) -> ()> Drop for Consumer<'a, T, F>
    where
        T: FromInstance,
{
    fn drop(&mut self) {
        let consumer: *mut dyn Fn(DataWrapper<Instance>) -> () =
            unsafe { transmute(self.internal_closure_raw) };
        let boxed = unsafe { Box::from_raw(consumer) };
        drop(boxed)
    }
}
