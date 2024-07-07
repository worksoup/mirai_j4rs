use crate::utils::raw_pointer_to_instance;
use crate::RawPointer;
use j4rs::errors::J4RsError;
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::{
    data_wrapper::DataWrapper,
    env::{TryFromInstanceTrait, GetInstanceTrait},
};
use mj_macro::GetInstanceDerive;
use std::marker::PhantomData;

#[derive(GetInstanceDerive)]
pub struct Function<T, R> {
    instance: Instance,
    internal_closure_raw: RawPointer,
    _t: PhantomData<T>,
    _r: PhantomData<R>,
}
impl<T, R> Function<T, R> {
    unsafe fn get_internal_closure_raw(
        &self,
    ) -> *mut dyn Fn(DataWrapper<Instance>) -> Result<Instance, J4RsError> {
        unsafe { std::mem::transmute(self.internal_closure_raw) }
    }
    pub fn drop(self) {
        let _ = unsafe { Box::from_raw(self.get_internal_closure_raw()) };
    }
}
impl<T, R> Function<T, R>
where
    R: TryFromInstanceTrait,
{
    pub fn apply(&self, arg: InvocationArg) -> Result<R, J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let result = jvm.invoke(&self.get_instance()?, "apply", &[arg])?;
        R::try_from_instance(result)
    }
}
impl<T, R> Function<T, R>
where
    T: TryFromInstanceTrait,
    R: GetInstanceTrait,
{
    #[inline]
    fn internal_closure_as_raw_pointer<F>(f: F) -> RawPointer
    where
        F: Fn(T) -> R + 'static,
    {
        let call_from_java: Box<dyn Fn(DataWrapper<Instance>) -> Result<Instance, J4RsError>> =
            Box::new(
                move |value: DataWrapper<Instance>| -> Result<Instance, J4RsError> {
                    f(value.get::<T>()?).get_instance()
                },
            );
        let call_from_java_raw = Box::into_raw(call_from_java);
        unsafe { std::mem::transmute(call_from_java_raw) }
    }
    pub fn new<F>(closure: F) -> Function<T, R>
    where
        F: Fn(T) -> R + 'static,
    {
        let internal_closure_raw = Self::internal_closure_as_raw_pointer(closure);
        println!("closure_to_function\n{:?}", internal_closure_raw);
        let instance = raw_pointer_to_instance::<"rt.lea.function.LumiaFunction">(internal_closure_raw);
        Function {
            instance,
            internal_closure_raw,
            _t: PhantomData,
            _r: PhantomData,
        }
    }
}
