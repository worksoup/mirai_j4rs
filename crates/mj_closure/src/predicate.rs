
use crate::function::Function;
use j4rs::errors::J4RsError;
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::TryFromInstanceTrait;
use mj_base::env::GetInstanceTrait;
use mj_macro::GetInstanceDerive;

#[derive(GetInstanceDerive)]
pub struct Predicate<T> {
    instance: Instance,
    func: Function<T, bool>,
}
impl<T> Predicate<T> {
    pub fn drop(self) {
        self.func.drop()
    }
    pub fn test(&self, v: InvocationArg) -> Result<bool, J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let result = jvm.invoke(&self.get_instance()?, "test", &[v])?;
        bool::try_from_instance(result)
    }
}

impl<T> Predicate<T>
where
    T: TryFromInstanceTrait,
{
    pub fn new<F>(closure: F) -> Predicate<T>
    where
        F: Fn(T) -> bool + 'static,
    {
        let internal_fn = move |v1: T| -> bool { closure(v1) };
        let func = Function::new(internal_fn);
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                "rt.lea.function.LumiaPredicate",
                &[InvocationArg::from(func.get_instance().unwrap())],
            )
            .unwrap();
        Predicate { instance, func }
    }
}
