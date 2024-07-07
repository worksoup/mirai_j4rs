use crate::Supplier;
use j4rs::errors::J4RsError;
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::TryFromInstanceTrait;
use mj_base::env::GetInstanceTrait;
use mj_macro::GetInstanceDerive;

#[derive(GetInstanceDerive)]
pub struct KtFunc0<R> {
    instance: Instance,
    func: Supplier<R>,
}
impl<R> KtFunc0<R> {
    pub fn drop(self) {
        self.func.drop()
    }
}
impl<R> KtFunc0<R>
where
    R: TryFromInstanceTrait,
{
    pub fn invoke(&self) -> Result<R, J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let result = jvm.invoke(&self.get_instance()?, "invoke", InvocationArg::empty())?;
        R::try_from_instance(result)
    }
}

impl<R> KtFunc0<R>
where
    R: GetInstanceTrait,
{
    pub fn new<F>(closure: F) -> KtFunc0<R>
    where
        F: Fn() -> R + 'static,
    {
        let internal_fn = move || -> R { closure() };
        let func = Supplier::new(internal_fn);
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                "rt.lea.function.LumiaKtFunc0",
                &[InvocationArg::from(func.get_instance().unwrap())],
            )
            .unwrap();
        KtFunc0 { instance, func }
    }
}
