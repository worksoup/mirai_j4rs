use std::ops::Deref;

use crate::env::{TryFromInstanceTrait, GetInstanceTrait};
use j4rs::errors::J4RsError;
use j4rs::{Instance, InvocationArg, Jvm};

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

impl TryFromInstanceTrait for DataWrapper<String> {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust::<String>(instance).map(|r| r.into())
    }
}

impl GetInstanceTrait for DataWrapper<String> {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        Instance::try_from(InvocationArg::try_from(&self.data).unwrap())
    }
}

impl TryFromInstanceTrait for DataWrapper<Vec<i8>> {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust::<Vec<_>>(instance).map(|r| r.into())
    }
}

impl<P1, P2> DataWrapper<(P1, P2)>
where
    P1: TryFromInstanceTrait,
    P2: TryFromInstanceTrait,
{
    pub fn get_pair(self) -> (P1, P2) {
        self.data
    }
}

impl DataWrapper<Instance> {
    pub fn get<T>(&self) -> Result<T, J4RsError>
    where
        T: TryFromInstanceTrait,
    {
        T::try_from_instance(Jvm::attach_thread()?.clone_instance(&self.data)?)
    }
}

impl GetInstanceTrait for DataWrapper<Instance> {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.clone_instance(&self.data)
    }
}

impl TryFromInstanceTrait for DataWrapper<Instance> {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        Ok(Self { data: instance })
    }
}

impl TryFromInstanceTrait for DataWrapper<()> {
    fn try_from_instance(_: Instance) -> Result<Self, J4RsError> {
        Ok(Self { data: () })
    }
}

impl GetInstanceTrait for DataWrapper<()> {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.invoke_static(
            "javax.lang.model.util.Types",
            "getNullType",
            InvocationArg::empty(),
        )
    }
}

impl<T: TryFromInstanceTrait> TryFromInstanceTrait for DataWrapper<T> {
    default fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        <T as TryFromInstanceTrait>::try_from_instance(instance).map(|r| r.into())
    }
}
impl<T: GetInstanceTrait> GetInstanceTrait for DataWrapper<T> {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        <T as GetInstanceTrait>::get_instance(&self.data)
    }
}
