use j4rs::{prelude::*, Instance, Jvm};
use j4rs_derive::*;
use mj_base::{
    data_wrapper::DataWrapper,
    env::{FromInstance, GetInstanceTrait},
    utils::instance_from_i8_16,
};
use mj_macro::GetInstanceDerive;
use std::{marker::PhantomData, mem::transmute};

#[call_from_java("rt.lea.LumiaKtFunc0.nativeInvoke")]
fn lumia_kt_func_0_invoke(kt_func_0_raw_as_i8_16: Instance) -> Result<Instance, String> {
    let kt_func_0_raw: [i8; 16] = Jvm::attach_thread()
        .unwrap()
        .to_rust(kt_func_0_raw_as_i8_16)
        .unwrap();
    println!(
        "lumia_kt_func_0_invoke, in {}, {}:{}",
        file! {},
        line!(),
        column!()
    );
    println!("kt_func_0_raw: {:?}", kt_func_0_raw);
    let kt_func_0: *mut dyn Fn() -> Instance = unsafe { transmute(kt_func_0_raw) };
    let value = unsafe { (*kt_func_0)() };
    Ok(value)
}

pub struct KtFunc0<'a, R: GetInstanceTrait + FromInstance> {
    instance: Instance,
    internal_closure_raw: [i8; 16],
    __a: PhantomData<&'a ()>,
    _r: PhantomData<R>,
}

#[derive(GetInstanceDerive)]
pub struct KtFunc0Raw {
    instance: Instance,
    internal_closure_raw: [i8; 16],
}

impl KtFunc0Raw {
    fn get_internal_closure_raw(&self) -> *mut dyn Fn(DataWrapper<Instance>) -> Instance {
        unsafe { transmute(self.internal_closure_raw) }
    }
    pub fn drop_internal_closure_raw(&self) {
        let _boxed = unsafe { Box::from_raw(self.get_internal_closure_raw()) };
    }
}

impl<'a, R: GetInstanceTrait + FromInstance> KtFunc0<'a, R> {
    #[inline]
    fn internal_closure_as_i8_16<F: Fn() -> R>(f: &'a F) -> [i8; 16] {
        let call_from_java = || -> Instance { f().get_instance() };
        let call_from_java_raw: *mut dyn Fn() -> Instance = Box::into_raw(Box::new(call_from_java));
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    pub fn new<F: Fn() -> R>(closure: &'a F) -> KtFunc0<R> {
        let internal_closure_raw = Self::internal_closure_as_i8_16(closure);
        println!("closure_to_kt_func_0\n{:?}", internal_closure_raw);
        let instance = instance_from_i8_16::<"rt.lea.LumiaKtFunc0">(internal_closure_raw);
        KtFunc0 {
            instance,
            internal_closure_raw,
            __a: PhantomData::default(),
            _r: PhantomData::default(),
        }
    }
    pub fn to_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
    pub fn invoke(&self) -> R {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm.invoke(&self.instance, "invoke", &[]).unwrap();
        R::from_instance(result)
    }
    fn get_internal_closure_raw(&self) -> *mut dyn Fn(DataWrapper<Instance>) -> Instance {
        unsafe { transmute(self.internal_closure_raw) }
    }
    pub(super) fn drop_internal_closure_raw(&self) {
        let _boxed = unsafe { Box::from_raw(self.get_internal_closure_raw()) };
    }
    pub fn drop_and_to_raw(self) -> KtFunc0Raw {
        let instance = self.instance;
        let internal_closure_raw = self.internal_closure_raw;
        KtFunc0Raw {
            instance,
            internal_closure_raw,
        }
    }
}
