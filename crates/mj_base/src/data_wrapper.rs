use crate::env::{FromInstanceTrait, GetInstanceTrait};
use j4rs::{Instance, InvocationArg, Jvm};
use std::ops::Deref;

pub struct DataWrapper<T> {
    data: T,
}

impl<T> Deref for DataWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> From<T> for DataWrapper<T> {
    fn from(data: T) -> Self {
        Self { data }
    }
}

impl FromInstanceTrait for DataWrapper<String> {
    fn from_instance(instance: Instance) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust::<String>(instance).unwrap().into()
    }
}

impl GetInstanceTrait for DataWrapper<String> {
    fn get_instance(&self) -> Instance {
        Instance::try_from(InvocationArg::try_from(&self.data).unwrap()).unwrap()
    }
}

impl FromInstanceTrait for DataWrapper<Vec<i8>> {
    fn from_instance(instance: Instance) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust::<Vec<_>>(instance).unwrap().into()
    }
}

impl<P1, P2> DataWrapper<(P1, P2)>
where
    P1: FromInstanceTrait,
    P2: FromInstanceTrait,
{
    pub fn get_pair(self) -> (P1, P2) {
        self.data
    }
}

impl<P1, P2> FromInstanceTrait for DataWrapper<(P1, P2)>
where
    P1: FromInstanceTrait,
    P2: FromInstanceTrait,
{
    fn from_instance(instance: Instance) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.cast(&instance, "kotlin.Pair").unwrap();
        let val1 = jvm.invoke(&instance, "getFirst", &[]).unwrap();
        let val2 = jvm.invoke(&instance, "getSecond", &[]).unwrap();
        let val1 = P1::from_instance(val1);
        let val2 = P2::from_instance(val2);
        Self { data: (val1, val2) }
    }
}

impl DataWrapper<Instance> {
    pub fn get<E>(&self) -> E
    where
        E: FromInstanceTrait,
    {
        E::from_instance(
            Jvm::attach_thread()
                .unwrap()
                .clone_instance(&self.data)
                .unwrap(),
        )
    }
}

impl GetInstanceTrait for DataWrapper<Instance> {
    fn get_instance(&self) -> Instance {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.clone_instance(&self.data).unwrap()
    }
}

impl FromInstanceTrait for DataWrapper<Instance> {
    fn from_instance(instance: Instance) -> Self {
        Self { data: instance }
    }
}

impl FromInstanceTrait for DataWrapper<()> {
    fn from_instance(_instance: Instance) -> Self {
        Self { data: () }
    }
}

impl GetInstanceTrait for DataWrapper<()> {
    fn get_instance(&self) -> Instance {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.invoke_static("javax.lang.model.util.Types", "getNullType", &[])
            .unwrap()
    }
}

impl<T: FromInstanceTrait> FromInstanceTrait for DataWrapper<T> {
    fn from_instance(instance: Instance) -> Self {
        <T as FromInstanceTrait>::from_instance(instance).into()
    }
}

impl<T: GetInstanceTrait> GetInstanceTrait for DataWrapper<T> {
    fn get_instance(&self) -> Instance {
        <T as GetInstanceTrait>::get_instance(&self.data)
    }
}
