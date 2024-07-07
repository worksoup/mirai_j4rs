use std::cmp::Ordering;

use crate::bi_function::BiFunction;
use crate::function::Function;
use j4rs::errors::J4RsError;
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::TryFromInstanceTrait;
use mj_base::env::GetInstanceTrait;
use mj_macro::GetInstanceDerive;

#[derive(GetInstanceDerive)]
pub struct Comparator<T> {
    instance: Instance,
    func: BiFunction<T, T, Ordering>,
}
impl<T> Comparator<T> {
    pub fn drop(self) {
        self.func.drop()
    }
    pub fn compare(&self, v1: InvocationArg, v2: InvocationArg) -> Result<Ordering, J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let result = jvm.invoke(&self.get_instance()?, "compare", &[v1, v2])?;
        Ordering::try_from_instance(result)
    }
}

impl<T> Comparator<T>
where
    T: TryFromInstanceTrait,
{
    pub fn new<F>(closure: F) -> Comparator<T>
    where
        F: Fn(T, T) -> Ordering + 'static,
    {
        let internal_fn = move |v1: T, v2: T| -> Ordering { closure(v1, v2) };
        let func = BiFunction::new(internal_fn);
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                "rt.lea.function.LumiaComparator",
                &[InvocationArg::from(func.get_instance().unwrap())],
            )
            .unwrap();
        Comparator { instance, func }
    }
}
