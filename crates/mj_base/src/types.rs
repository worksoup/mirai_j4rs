use j4rs::errors::J4RsError;
use j4rs::{Instance, Jvm};
use jbuchong::{AsInstanceTrait, GetInstanceTrait, TryFromInstanceTrait};

pub struct UnknownTypeValue(Instance);

impl GetInstanceTrait for UnknownTypeValue {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.clone_instance(&self.0)
    }
}
impl AsInstanceTrait for UnknownTypeValue {
    fn as_instance(&self) -> &Instance {
        &self.0
    }
}
impl TryFromInstanceTrait for UnknownTypeValue {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        Ok(Self(instance))
    }
}
