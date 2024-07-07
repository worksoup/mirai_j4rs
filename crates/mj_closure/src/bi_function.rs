use crate::function::Function;
use j4rs::errors::J4RsError;
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::TryFromInstanceTrait;
use mj_base::env::GetInstanceTrait;
use mj_macro::GetInstanceDerive;

#[derive(GetInstanceDerive)]
pub struct BiFunction<T1, T2, R> {
    instance: Instance,
    func: Function<(T1, T2), R>,
}
impl<T1, T2, R> BiFunction<T1, T2, R> {
    pub fn drop(self) {
        self.func.drop()
    }
}
impl<T1, T2, R> BiFunction<T1, T2, R>
where
    R: TryFromInstanceTrait,
{
    pub fn apply(&self, v1: InvocationArg, v2: InvocationArg) -> Result<R, J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let result = jvm.invoke(&self.get_instance()?, "apply", &[v1, v2])?;
        R::try_from_instance(result)
    }
}

impl<T1, T2, R> BiFunction<T1, T2, R>
where
    T1: TryFromInstanceTrait,
    T2: TryFromInstanceTrait,
    R: GetInstanceTrait,
{
    pub fn new<F>(closure: F) -> BiFunction<T1, T2, R>
    where
        F: Fn(T1, T2) -> R + 'static,
    {
        let internal_fn = move |(v1, v2): (T1, T2)| -> R { closure(v1, v2) };
        let func = Function::new(internal_fn);
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                "rt.lea.function.LumiaBiFunction",
                &[InvocationArg::from(func.get_instance().unwrap())],
            )
            .unwrap();
        BiFunction { instance, func }
    }
}
