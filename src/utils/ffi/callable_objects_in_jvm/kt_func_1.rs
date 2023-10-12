use crate::env::{FromInstance, GetEnvTrait};
use crate::utils::internal::data_wrapper::DataWrapper;
use crate::utils::internal::instance_from_i8_16;
use j4rs::{Instance, InvocationArg, Jvm};
use std::default::Default;
use std::{marker::PhantomData, mem::transmute, pin::Pin};

pub struct KtFunc1<'a, T, R, F>
    where
        T: FromInstance,
        F: Fn(T) -> R,
        R: GetEnvTrait + FromInstance,
{
    instance: Instance,
    internal_closure_raw: [i8; 16],
    _t: PhantomData<T>,
    _f: PhantomData<&'a F>,
}

impl<T, R, F> GetEnvTrait for KtFunc1<'_, T, R, F>
    where
        T: FromInstance,
        F: Fn(T) -> R,
        R: GetEnvTrait + FromInstance,
{
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}

impl<'a, T, R, F> KtFunc1<'a, T, R, F>
    where
        T: FromInstance,
        F: Fn(T) -> R,
        R: GetEnvTrait + FromInstance,
{
    #[inline]
    fn internal_closure_as_i8_16(f: &'a F) -> [i8; 16] {
        let call_from_java = |value: DataWrapper<Instance>| -> Instance {
            let value = value.get::<T>();
            f(value).get_instance()
        };
        let call_from_java_raw: *mut dyn Fn(DataWrapper<Instance>) -> Instance =
            Box::into_raw(Box::new(call_from_java));
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    pub fn new(closure: &'a F) -> Pin<Box<KtFunc1<'a, T, R, F>>> {
        let jvm = Jvm::attach_thread().unwrap();
        let internal_closure_raw = Self::internal_closure_as_i8_16(closure);
        let lumia_function = instance_from_i8_16::<"rt.lea.LumiaFunction">(internal_closure_raw);
        let instance = jvm
            .create_instance(
                "rt.lea.LumiaKtFunc1",
                &[InvocationArg::try_from(lumia_function).unwrap()],
            )
            .unwrap();
        Box::pin(KtFunc1 {
            instance,
            internal_closure_raw,
            _t: PhantomData::default(),
            _f: PhantomData::default(),
        })
    }
    pub fn invoke(&self, val1: InvocationArg) -> R {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm
            .invoke(&self.instance, "invoke", &[val1])
            .unwrap();
        R::from_instance(result)
    }
}

impl<'a, T, R, F> Drop for KtFunc1<'a, T, R, F>
    where
        T: FromInstance,
        F: Fn(T) -> R,
        R: GetEnvTrait + FromInstance,
{
    fn drop(&mut self) {
        let kt_func_1: *mut dyn Fn(DataWrapper<Instance>) -> Instance =
            unsafe { transmute(self.internal_closure_raw) };
        let boxed = unsafe { Box::from_raw(kt_func_1) };
        drop(boxed)
    }
}
