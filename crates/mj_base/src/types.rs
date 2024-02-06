use j4rs::{Instance, Jvm};

use crate::env::{AsInstanceTrait, FromInstanceTrait, GetInstanceTrait};

pub struct UnknownTypeValue(Instance);

impl GetInstanceTrait for UnknownTypeValue {
    fn get_instance(&self) -> Instance {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.clone_instance(&self.0).unwrap()
    }
}
impl AsInstanceTrait for UnknownTypeValue {
    fn as_instance(&self) -> &Instance {
        &self.0
    }
}
impl FromInstanceTrait for UnknownTypeValue {
    fn from_instance(instance: Instance) -> Self {
        Self(instance)
    }
}
