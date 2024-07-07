use crate::utils::raw_pointer_to_instance;
use crate::RawPointer;
use j4rs::errors::J4RsError;
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::GetInstanceTrait;
use mj_base::{data_wrapper::DataWrapper, env::TryFromInstanceTrait};
use mj_macro::GetInstanceDerive;
use std::marker::PhantomData;

#[derive(GetInstanceDerive)]
pub struct Consumer<T> {
    instance: Instance,
    internal_closure_raw: RawPointer,
    _t: PhantomData<T>,
}

impl<T> Consumer<T> {
    unsafe fn get_internal_closure_raw(
        &self,
    ) -> *mut dyn Fn(DataWrapper<Instance>) -> Result<(), J4RsError> {
        unsafe { std::mem::transmute(self.internal_closure_raw) }
    }
    pub fn drop(self) {
        let _ = unsafe { Box::from_raw(self.get_internal_closure_raw()) };
    }
    pub fn accept(&self, arg: InvocationArg) -> Result<(), J4RsError> {
        Jvm::attach_thread()?.invoke(&self.get_instance()?, "accept", &[arg])?;
        Ok(())
    }
}
impl<T> Consumer<T>
where
    T: TryFromInstanceTrait,
{
    #[inline]
    fn internal_closure_as_raw_pointer<F>(f: F) -> RawPointer
    where
        F: Fn(T) + 'static,
    {
        let call_from_java: Box<dyn Fn(DataWrapper<Instance>) -> Result<(), J4RsError>> = Box::new(
            move |value: DataWrapper<Instance>| -> Result<(), J4RsError> {
                Ok(f(value.get::<T>()?))
            },
        );
        let call_from_java_raw = Box::into_raw(call_from_java);
        unsafe { std::mem::transmute(call_from_java_raw) }
    }
    pub fn new<F>(closure: F) -> Consumer<T>
    where
        F: Fn(T) + 'static,
    {
        let internal_closure_raw = Self::internal_closure_as_raw_pointer(closure);
        println!("closure_to_function\n{:?}", internal_closure_raw);
        let instance = raw_pointer_to_instance::<"rt.lea.function.LumiaConsumer">(internal_closure_raw);
        Consumer {
            instance,
            internal_closure_raw,
            _t: PhantomData::default(),
        }
    }
}
