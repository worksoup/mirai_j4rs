use crate::env::{FromInstance, GetEnvTrait};
use crate::utils::internal::data_wrapper::DataWrapper;
use crate::utils::internal::instance_from_i8_16;
use j4rs::{Instance, InvocationArg, Jvm};
use std::{marker::PhantomData, mem::transmute, pin::Pin};

pub struct KtFunc0<'a, R, F>
    where
        F: Fn() -> R,
        R: GetEnvTrait + FromInstance,
{
    instance: Instance,
    internal_closure_raw: [i8; 16],
    _f: PhantomData<&'a F>,
}

impl<'a, R, F> GetEnvTrait for KtFunc0<'a, R, F>
    where
        F: Fn() -> R,
        R: GetEnvTrait + FromInstance,
{
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}

impl<'a, R, F> KtFunc0<'a, R, F>
    where
        F: Fn() -> R,
        R: GetEnvTrait + FromInstance,
{
    #[inline]
    fn internal_closure_as_i8_16(f: &'a F) -> [i8; 16] {
        let call_from_java = |_: DataWrapper<()>| -> Instance { f().get_instance() };
        let call_from_java_raw: *mut dyn Fn(DataWrapper<()>) -> Instance =
            Box::into_raw(Box::new(call_from_java));
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    pub fn new(closure: &'a F) -> Pin<Box<KtFunc0<R, F>>> {
        let jvm = Jvm::attach_thread().unwrap();
        let internal_closure_raw = Self::internal_closure_as_i8_16(closure);
        let lumia_function = instance_from_i8_16::<"rt.lea.LumiaFunction">(internal_closure_raw);
        let instance = jvm
            .create_instance(
                "rt.lea.LumiaKtFunc0",
                &[InvocationArg::try_from(lumia_function).unwrap()],
            )
            .unwrap();
        Box::pin(KtFunc0 {
            instance,
            internal_closure_raw,
            _f: PhantomData::default(),
        })
    }
    pub fn invoke(&self) -> R {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm.invoke(&self.instance, "invoke", &[]).unwrap();
        R::from_instance(result)
    }
}

impl<'a, R, F> Drop for KtFunc0<'a, R, F>
    where
        F: Fn() -> R,
        R: GetEnvTrait + FromInstance, {
    fn drop(&mut self) {
        let kt_func_0: *mut dyn Fn(DataWrapper<Instance>) -> Instance =
            unsafe { transmute(self.internal_closure_raw) };
        let boxed = unsafe { Box::from_raw(kt_func_0) };
        drop(boxed)
    }
}
