use crate::utils::raw_pointer_to_instance;
use crate::RawPointer;
use j4rs::errors::J4RsError;
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::{TryFromInstanceTrait, GetInstanceTrait};
use mj_macro::GetInstanceDerive;
use std::marker::PhantomData;

#[derive(GetInstanceDerive)]
pub struct Supplier<R> {
    instance: Instance,
    internal_closure_raw: RawPointer,
    _r: PhantomData<R>,
}
impl<R> Supplier<R> {
    unsafe fn get_internal_closure_raw(&self) -> *mut dyn Fn() -> Result<Instance, J4RsError> {
        unsafe { std::mem::transmute(self.internal_closure_raw) }
    }
    pub fn drop(self) {
        let _ = unsafe { Box::from_raw(self.get_internal_closure_raw()) };
    }
}
impl<R> Supplier<R>
where
    R: TryFromInstanceTrait,
{
    pub fn get(&self) -> Result<R, J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let result = jvm.invoke(&self.get_instance()?, "get", InvocationArg::empty())?;
        R::try_from_instance(result)
    }
}
impl<R> Supplier<R>
where
    R: GetInstanceTrait,
{
    #[inline]
    fn internal_closure_as_raw_pointer<F>(f: F) -> RawPointer
    where
        F: Fn() -> R + 'static,
    {
        let call_from_java: Box<dyn Fn() -> Result<Instance, J4RsError>> =
            Box::new(move || -> Result<Instance, J4RsError> { f().get_instance() });
        let call_from_java_raw = Box::into_raw(call_from_java);
        unsafe { std::mem::transmute(call_from_java_raw) }
    }
    pub fn new<F>(closure: F) -> Supplier<R>
    where
        F: Fn() -> R + 'static,
    {
        let internal_closure_raw = Self::internal_closure_as_raw_pointer(closure);
        let instance = raw_pointer_to_instance::<"rt.lea.function.LumiaSupplier">(internal_closure_raw);
        Supplier {
            instance,
            internal_closure_raw,
            _r: PhantomData,
        }
    }
}
