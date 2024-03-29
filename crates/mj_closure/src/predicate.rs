use std::{marker::PhantomData, mem::transmute};

use j4rs::{errors::J4RsError, prelude::*, Instance, InvocationArg, Jvm};
use j4rs_derive::*;

use mj_base::{data_wrapper::DataWrapper, env::FromInstanceTrait, utils::instance_from_i8_16};
use mj_macro::GetInstanceDerive;

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

pub struct Predicate<'a, T>
where
    T: FromInstanceTrait,
{
    instance: Instance,
    internal_closure_raw: [i8; 16],
    _t: PhantomData<T>,
    __a: PhantomData<&'a ()>,
}

#[derive(GetInstanceDerive)]
pub struct PredicateRaw {
    instance: Instance,
    internal_closure_raw: [i8; 16],
}

impl PredicateRaw {
    pub fn get_internal_closure_raw(
        &self,
    ) -> *mut dyn Fn(DataWrapper<Instance>) -> Result<InvocationArg, J4RsError> {
        unsafe { transmute(self.internal_closure_raw) }
    }
    pub fn drop_internal_closure_raw(&self) {
        let _boxed = unsafe { Box::from_raw(self.get_internal_closure_raw()) };
    }
}

impl<'a, T> Predicate<'a, T>
where
    T: FromInstanceTrait,
{
    #[inline]
    fn internal_closure_as_i8_16<F: Fn(T) -> bool>(f: &'a F) -> [i8; 16] {
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
    pub fn new<F: Fn(T) -> bool>(closure: &'a F) -> Predicate<'a, T> {
        let internal_closure_raw = Self::internal_closure_as_i8_16(closure);
        println!("closure_to_predicate\n{:?}", internal_closure_raw);
        let instance = instance_from_i8_16::<"rt.lea.LumiaPredicate">(internal_closure_raw);
        Predicate {
            instance,
            internal_closure_raw,
            _t: PhantomData::default(),
            __a: PhantomData::default(),
        }
    }
    pub fn to_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
    pub fn test(&self, arg: InvocationArg) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm.invoke(&self.instance, "test", &[arg]).unwrap();
        jvm.to_rust(result).unwrap()
    }
    pub fn drop_and_to_raw(self) -> PredicateRaw {
        let instance = self.instance;
        let internal_closure_raw = self.internal_closure_raw;
        PredicateRaw {
            instance,
            internal_closure_raw,
        }
    }
}
