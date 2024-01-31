use j4rs::{prelude::*, Instance, InvocationArg, Jvm};
use j4rs_derive::*;
use mjbase::data_wrapper::DataWrapper;
use mjbase::env::{FromInstance, GetEnvTrait};
use mjbase::utils::instance_from_i8_16;
use mjmacro::GetInstanceDerive;
use std::{marker::PhantomData, mem::transmute};

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

pub struct Function<'a, T: FromInstance, R: GetEnvTrait + FromInstance> {
    instance: Instance,
    internal_closure_raw: [i8; 16],
    __a: PhantomData<&'a ()>,
    _t: PhantomData<T>,
    _r: PhantomData<R>,
}

#[derive(GetInstanceDerive)]
pub struct FunctionRaw {
    instance: Instance,
    internal_closure_raw: [i8; 16],
}

impl FunctionRaw {
    fn get_internal_closure_raw(&self) -> *mut dyn Fn(DataWrapper<Instance>) -> Instance {
        unsafe { transmute(self.internal_closure_raw) }
    }
    pub fn drop_internal_closure_raw(&self) {
        let _boxed = unsafe { Box::from_raw(self.get_internal_closure_raw()) };
    }
}

impl<'a, T: FromInstance, R: GetEnvTrait + FromInstance> Function<'a, T, R> {
    #[inline]
    fn internal_closure_as_i8_16<F>(f: &'a F) -> [i8; 16]
    where
        F: Fn(T) -> R,
    {
        let call_from_java_raw: *mut dyn Fn(DataWrapper<Instance>) -> Instance =
            Box::into_raw(Box::new(|value: DataWrapper<Instance>| -> Instance {
                f(value.get::<T>()).get_instance()
            }));
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    pub fn new<F>(closure: &'a F) -> Function<'a, T, R>
    where
        F: Fn(T) -> R,
    {
        let internal_closure_raw = Self::internal_closure_as_i8_16(closure);
        println!("closure_to_function\n{:?}", internal_closure_raw);
        let instance = instance_from_i8_16::<"rt.lea.LumiaFunction">(internal_closure_raw);
        Function {
            instance,
            internal_closure_raw,
            __a: PhantomData::default(),
            _t: PhantomData::default(),
            _r: PhantomData::default(),
        }
    }
    pub fn apply(&self, arg: InvocationArg) -> R {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm.invoke(&self.instance, "apply", &[arg]).unwrap();
        R::from_instance(result)
    }
    pub fn to_instance(&self) -> Instance {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.clone_instance(&self.instance).unwrap()
    }
    fn get_internal_closure_raw(&self) -> *mut dyn Fn(DataWrapper<Instance>) -> Instance {
        unsafe { transmute(self.internal_closure_raw) }
    }
    pub(super) fn drop_internal_closure_raw(&self) {
        let _boxed = unsafe { Box::from_raw(self.get_internal_closure_raw()) };
    }
    pub fn drop_and_to_raw(self) -> FunctionRaw {
        let instance = self.instance;
        let internal_closure_raw = self.internal_closure_raw;
        FunctionRaw {
            instance,
            internal_closure_raw,
        }
    }
}
