use std::{marker::PhantomData, mem::transmute};

use j4rs::{Instance, InvocationArg, Jvm};

use mj_base::{
    data_wrapper::DataWrapper,
    env::{FromInstanceTrait, GetInstanceTrait},
    utils::instance_from_i8_16,
};
use mj_macro::GetInstanceDerive;

pub struct KtFunc1<'a, T: FromInstanceTrait, R: GetInstanceTrait + FromInstanceTrait> {
    instance: Instance,
    internal_closure_raw: [i8; 16],
    _t: PhantomData<T>,
    _r: PhantomData<R>,
    __a: PhantomData<&'a ()>,
}

#[derive(GetInstanceDerive)]
pub struct KtFunc1Raw {
    instance: Instance,
    internal_closure_raw: [i8; 16],
}

impl KtFunc1Raw {
    fn get_internal_closure_raw(&self) -> *mut dyn Fn(DataWrapper<Instance>) -> Instance {
        unsafe { transmute(self.internal_closure_raw) }
    }
    pub fn drop_internal_closure_raw(&self) {
        let _boxed = unsafe { Box::from_raw(self.get_internal_closure_raw()) };
    }
}

impl<'a, T, R> KtFunc1<'a, T, R>
where
    T: FromInstanceTrait,
    R: GetInstanceTrait + FromInstanceTrait,
{
    #[inline]
    fn internal_closure_as_i8_16<F: Fn(T) -> R>(f: &'a F) -> [i8; 16] {
        let call_from_java = |value: DataWrapper<Instance>| -> Instance {
            let value = value.get::<T>();
            f(value).get_instance()
        };
        let call_from_java_raw: *mut dyn Fn(DataWrapper<Instance>) -> Instance =
            Box::into_raw(Box::new(call_from_java));
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    pub fn new<F: Fn(T) -> R>(closure: &'a F) -> KtFunc1<'a, T, R> {
        let jvm = Jvm::attach_thread().unwrap();
        let internal_closure_raw = Self::internal_closure_as_i8_16(closure);
        let lumia_function = instance_from_i8_16::<"rt.lea.LumiaFunction">(internal_closure_raw);
        let instance = jvm
            .create_instance(
                "rt.lea.LumiaKtFunc1",
                &[InvocationArg::try_from(lumia_function).unwrap()],
            )
            .unwrap();
        KtFunc1 {
            instance,
            internal_closure_raw,
            _t: PhantomData::default(),
            _r: PhantomData::default(),
            __a: PhantomData::default(),
        }
    }
    pub fn to_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
    pub fn invoke(&self, val1: InvocationArg) -> R {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm.invoke(&self.instance, "invoke", &[val1]).unwrap();
        R::from_instance(result)
    }
    pub fn drop_and_to_raw(self) -> KtFunc1Raw {
        let instance = self.instance;
        let internal_closure_raw = self.internal_closure_raw;
        KtFunc1Raw {
            instance,
            internal_closure_raw,
        }
    }
}
