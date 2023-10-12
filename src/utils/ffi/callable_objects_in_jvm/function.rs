use crate::env::{FromInstance, GetEnvTrait};
use crate::utils::internal::data_wrapper::DataWrapper;
use crate::utils::internal::instance_from_i8_16;
use j4rs::{prelude::*, Instance, InvocationArg, Jvm};
use j4rs_derive::*;
use std::{marker::PhantomData, mem::transmute, pin::Pin};

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
    let function: *mut dyn Fn(DataWrapper<Instance>) -> Instance =
        unsafe { transmute(function_raw) };
    let value = unsafe { (*function)(DataWrapper::from_instance(val1)) };
    Ok(value)
}

pub struct Function<'a, T, F, R>
    where
        T: FromInstance,
        F: Fn(T) -> R,
        R: GetEnvTrait + FromInstance,
{
    instance: Instance,
    internal_closure_raw: [i8; 16],
    _t: PhantomData<T>,
    _r: PhantomData<R>,
    _f: PhantomData<&'a F>,
}

impl<'a, T, F, R> GetEnvTrait for Function<'a, T, F, R>
    where
        T: FromInstance,
        F: Fn(T) -> R,
        R: GetEnvTrait + FromInstance,
{
    fn get_instance(&self) -> Instance {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.clone_instance(&self.instance).unwrap()
    }
}

impl<'a, T, F, R> Function<'a, T, F, R>
    where
        T: FromInstance,
        F: Fn(T) -> R,
        R: GetEnvTrait + FromInstance,
{
    #[inline]
    fn internal_closure_as_i8_16(f: &'a F) -> [i8; 16] {
        let call_from_java = |value: DataWrapper<Instance>| -> Instance {
            let value = value.get::<T>();
            let value = f(value);
            value.get_instance()
        };
        let call_from_java_raw: *mut dyn Fn(DataWrapper<Instance>) -> Instance =
            Box::into_raw(Box::new(call_from_java));
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    pub fn new(closure: &'a F) -> Pin<Box<Function<'a, T, F, R>>> {
        let internal_closure_raw = Self::internal_closure_as_i8_16(closure);
        println!("closure_to_function\n{:?}", internal_closure_raw);
        let instance = instance_from_i8_16::<"rt.lea.LumiaFunction">(internal_closure_raw);
        Box::pin(Function {
            instance,
            internal_closure_raw,
            _t: PhantomData::default(),
            _r: PhantomData::default(),
            _f: PhantomData::default(),
        })
    }
    pub fn apply(&self, arg: InvocationArg) -> R {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm
            .invoke(&self.instance, "apply", &[arg])
            .unwrap();
        R::from_instance(result)
    }
}

impl<'a, T, F, R> Drop for Function<'a, T, F, R>
    where
        T: FromInstance,
        F: Fn(T) -> R,
        R: GetEnvTrait + FromInstance, {
    fn drop(&mut self) {
        let function: *mut dyn Fn(DataWrapper<Instance>) -> Instance =
            unsafe { transmute(self.internal_closure_raw) };
        let boxed = unsafe { Box::from_raw(function) };
        drop(boxed)
    }
}
