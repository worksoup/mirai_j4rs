use crate::env::FromInstance;
use crate::utils::internal::i8_16_to_bytes_16;
use contact_derive::GetInstanceDerive;
use j4rs::{prelude::*, Instance, InvocationArg, Jvm};
use j4rs_derive::*;
use std::cmp::Ordering;
use std::marker::PhantomData;
use std::mem::transmute;
use std::pin::Pin;

#[derive(GetInstanceDerive)]
pub struct InstanceWrapper {
    instance: Instance,
}

impl InstanceWrapper {
    pub fn get<E>(&self) -> E
        where
            E: FromInstance,
    {
        E::from_instance(
            Jvm::attach_thread()
                .unwrap()
                .clone_instance(&self.instance)
                .unwrap(),
        )
    }
}

impl FromInstance for InstanceWrapper {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }
}

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
    let consumer: *mut dyn Fn(InstanceWrapper) -> () = unsafe { transmute(consumer_raw) };
    unsafe {
        let _ = (*consumer)(InstanceWrapper::from_instance(arg));
    };
}

pub(crate) struct Consumer<T, F>
    where
        T: FromInstance,
        F: Fn(T) -> (),
{
    closure: F,
    instance: Option<Instance>,
    _unused: PhantomData<T>,
}

impl<T, F: Fn(T) -> ()> Consumer<T, F>
    where
        T: FromInstance,
{
    pub fn new(closure: F) -> Pin<Box<Consumer<T, F>>> {
        let mut consumer: Consumer<T, F> = Consumer {
            closure,
            instance: None,
            _unused: Default::default(),
        };
        let closure_ref = &consumer.closure;
        let call_from_java = Box::new(|value: InstanceWrapper| {
            let value = value.get::<T>();
            closure_ref(value);
        });
        let call_from_java_raw: *mut dyn Fn(InstanceWrapper) = Box::into_raw(call_from_java);
        let call_from_java_raw_as_i8_16 = unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) };
        println!("closure_to_consumer");
        println!("{:?}", call_from_java_raw_as_i8_16);
        let jvm = Jvm::attach_thread().unwrap();
        let call_from_java_raw_as_java_bytes =
            i8_16_to_bytes_16::<T>(&jvm, call_from_java_raw_as_i8_16);
        let instance = jvm
            .create_instance(
                "rt.lea.LumiaConsumer",
                &[InvocationArg::try_from(call_from_java_raw_as_java_bytes).unwrap()],
            )
            .unwrap();
        consumer.instance = Some(instance);
        Box::pin(consumer)
    }
    pub fn accept(&self, arg: InvocationArg) {
        let jvm = Jvm::attach_thread().unwrap();
        let _ = jvm
            .invoke(&self.instance.as_ref().unwrap(), "accept", &[arg])
            .unwrap();
    }
}

#[call_from_java("rt.lea.LumiaConsumer.nativeCompare")]
fn lumia_comparator_compare(
    comparator_as_i8_16: Instance,
    val1: Instance,
    val2: Instance,
) -> Result<Instance, String> {
    let comparator_raw: [i8; 16] = Jvm::attach_thread()
        .unwrap()
        .to_rust(comparator_as_i8_16)
        .unwrap();
    println!(
        "lumia_comparator_compare, in {}, {}:{}",
        file! {},
        line!(),
        column!()
    );
    println!("comparator_raw: {:?}", comparator_raw);
    let consumer: *mut dyn Fn(InstanceWrapper, InstanceWrapper) -> Ordering =
        unsafe { transmute(comparator_raw) };
    let ordering = unsafe {
        (*consumer)(
            InstanceWrapper::from_instance(val1),
            InstanceWrapper::from_instance(val2),
        )
    };
    let ordering = match ordering {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    };
    let ordering = InvocationArg::try_from(ordering).map_err(|error| format!("{}", error))?;
    Instance::try_from(ordering).map_err(|error| format!("{}", error))
}

#[call_from_java("rt.lea.LumiaFunction.nativeApply")]
fn lumia_function_apply(
    function_raw_as_i8_16: Instance,
    val1: Instance,
) -> Result<Instance, String> {
    let function_raw: [i8; 16] = Jvm::attach_thread()
        .unwrap()
        .to_rust(function_raw_as_i8_16)
        .unwrap();
    println!(
        "lumia_function_apply, in {}, {}:{}",
        file! {},
        line!(),
        column!()
    );
    println!("function_raw: {:?}", function_raw);
    let function: *mut dyn Fn(InstanceWrapper) -> Instance = unsafe { transmute(function_raw) };
    let value = unsafe { (*function)(InstanceWrapper::from_instance(val1)) };
    let value = InvocationArg::try_from(value).map_err(|error| format!("{}", error))?;
    Instance::try_from(value).map_err(|error| format!("{}", error))
}

#[call_from_java("rt.lea.LumiaPredicate.nativeTest")]
fn lumia_predicate_test(
    predicate_raw_as_i8_16: Instance,
    item: Instance,
) -> Result<Instance, String> {
    let predicate_raw: [i8; 16] = Jvm::attach_thread()
        .unwrap()
        .to_rust(predicate_raw_as_i8_16)
        .unwrap();
    println!(
        "lumia_predicate_test, in {}, {}:{}",
        file! {},
        line!(),
        column!()
    );
    println!("predicate_raw: {:?}", predicate_raw);
    let function: *mut dyn Fn(InstanceWrapper) -> bool = unsafe { transmute(predicate_raw) };
    let value = unsafe { (*function)(InstanceWrapper::from_instance(item)) };
    let value = InvocationArg::try_from(value).map_err(|error| format!("{}", error))?;
    Instance::try_from(value).map_err(|error| format!("{}", error))
}
