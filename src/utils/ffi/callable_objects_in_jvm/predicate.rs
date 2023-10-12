use crate::env::{FromInstance, GetEnvTrait};
use crate::utils::internal::data_wrapper::DataWrapper;
use crate::utils::internal::instance_from_i8_16;
use j4rs::{errors::J4RsError, prelude::*, Instance, InvocationArg, Jvm};
use j4rs_derive::*;
use std::{marker::PhantomData, mem::transmute, pin::Pin};

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
    let function: *mut dyn Fn(DataWrapper<Instance>) -> Result<InvocationArg, J4RsError> =
        unsafe { transmute(predicate_raw) };
    let value = unsafe { (*function)(DataWrapper::from_instance(item)) }
        .map_err(|error| format!("{}", error))?;
    Instance::try_from(value).map_err(|error| format!("{}", error))
}

pub struct Predicate<'a, T, F>
where
    T: FromInstance,
    F: Fn(T) -> bool,
{
    instance: Instance,
    internal_closure_raw: [i8; 16],
    _t: PhantomData<T>,
    _f: PhantomData<&'a F>,
}

impl<'a, T, F> GetEnvTrait for Predicate<'a, T, F>
where
    T: FromInstance,
    F: Fn(T) -> bool,
{
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}

impl<'a, T, F> Predicate<'a, T, F>
where
    T: FromInstance,
    F: Fn(T) -> bool,
{
    #[inline]
    fn internal_closure_as_i8_16(f: &'a F) -> [i8; 16] {
        let call_from_java = |value: DataWrapper<Instance>| -> Result<InvocationArg, J4RsError> {
            let value = value.get::<T>();
            let value = f(value);
            InvocationArg::try_from(value)
        };
        let call_from_java_raw: *mut dyn Fn(
            DataWrapper<Instance>,
        ) -> Result<InvocationArg, J4RsError> = Box::into_raw(Box::new(call_from_java));
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    pub fn new(closure: &'a F) -> Pin<Box<Predicate<'a, T, F>>> {
        let internal_closure_raw = Self::internal_closure_as_i8_16(closure);
        println!("closure_to_predicate\n{:?}", internal_closure_raw);
        let instance = instance_from_i8_16::<"rt.lea.LumiaPredicate">(internal_closure_raw);
        Box::pin(Predicate {
            instance,
            internal_closure_raw,
            _t: PhantomData::default(),
            _f: PhantomData::default(),
        })
    }
    pub fn test(&self, arg: InvocationArg) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm.invoke(&self.instance, "test", &[arg]).unwrap();
        jvm.to_rust(result).unwrap()
    }
}

impl<'a, T, F> Drop for Predicate<'a, T, F>
where
    T: FromInstance,
    F: Fn(T) -> bool,
{
    fn drop(&mut self) {
        let predicate: *mut dyn Fn(DataWrapper<Instance>) -> Result<InvocationArg, J4RsError> =
            unsafe { transmute(self.internal_closure_raw) };
        let boxed = unsafe { Box::from_raw(predicate) };
        drop(boxed)
    }
}
