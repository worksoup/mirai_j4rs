use j4rs::{prelude::*, Instance, InvocationArg, Jvm};
use j4rs_derive::*;
use mjbase::data_wrapper::DataWrapper;
use mjbase::env::FromInstance;
use mjbase::utils::instance_from_i8_16;
use mjmacro::GetInstanceDerive;
use std::{marker::PhantomData, mem::transmute};

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

pub struct Consumer<'a, T: FromInstance> {
    instance: Instance,
    internal_closure_raw: [i8; 16],
    _t: PhantomData<T>,
    __a: PhantomData<&'a ()>,
}

#[derive(GetInstanceDerive)]
pub struct ConsumerRaw {
    instance: Instance,
    internal_closure_raw: [i8; 16],
}

impl ConsumerRaw {
    fn get_internal_closure_raw(&self) -> *mut dyn Fn(DataWrapper<Instance>) -> () {
        unsafe { transmute(self.internal_closure_raw) }
    }
    pub fn drop_internal_closure_raw(&self) {
        let _boxed = unsafe { Box::from_raw(self.get_internal_closure_raw()) };
    }
}

impl<'a, T: FromInstance> Consumer<'a, T> {
    #[inline]
    fn internal_closure_as_i8_16<F: Fn(T) -> ()>(f: &'a F) -> [i8; 16] {
        let call_from_java = |value: DataWrapper<Instance>| {
            let value = value.get::<T>();
            f(value);
        };
        let call_from_java_raw: *mut dyn Fn(DataWrapper<Instance>) =
            Box::into_raw(Box::new(call_from_java));
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    pub fn new<F: Fn(T) -> ()>(closure: &'a F) -> Consumer<T> {
        let internal_closure_raw = Self::internal_closure_as_i8_16(closure);
        println!("closure_to_consumer");
        println!("{:?}", internal_closure_raw);
        let jvm = Jvm::attach_thread().unwrap();
        let instance = instance_from_i8_16::<"rt.lea.LumiaConsumer">(internal_closure_raw);
        Consumer {
            instance,
            internal_closure_raw,
            _t: PhantomData::default(),
            __a: PhantomData::default(),
        }
    }
    pub fn accept(&self, arg: InvocationArg) {
        let jvm = Jvm::attach_thread().unwrap();
        let _ = jvm.invoke(&self.instance, "accept", &[arg]).unwrap();
    }
    pub fn to_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
    fn get_internal_closure_raw(&self) -> *mut dyn Fn(DataWrapper<Instance>) -> () {
        unsafe { transmute(self.internal_closure_raw) }
    }
    pub(super) fn drop_internal_closure_raw(&self) {
        let _boxed = unsafe { Box::from_raw(self.get_internal_closure_raw()) };
    }
    pub fn drop_and_to_raw(self) -> ConsumerRaw {
        let instance = self.instance;
        let internal_closure_raw = self.internal_closure_raw;
        ConsumerRaw {
            instance,
            internal_closure_raw,
        }
    }
}
