use crate::contact::ContactTrait;
use crate::message::MessageHashCodeTrait;
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::{FromInstance, GetInstanceTrait};
use mj_base::utils::instance_is_null;
use std::marker::PhantomData;

pub struct ContactList<T>
where
    T: ContactTrait + FromInstance,
{
    pub(crate) instance: Instance,
    pub(crate) _unused: PhantomData<T>,
}

impl<T: ContactTrait + FromInstance> FromInstance for ContactList<T> {
    fn from_instance(instance: Instance) -> Self {
        ContactList {
            instance,
            _unused: PhantomData::default(),
        }
    }
}

impl<T: ContactTrait + FromInstance> GetInstanceTrait for ContactList<T> {
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}

impl<T> ContactList<T>
where
    T: ContactTrait + FromInstance,
{
    pub fn contains(&self, contact: T) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(
                        &self.instance,
                        "contains",
                        &[InvocationArg::try_from(contact.get_instance()).unwrap()],
                    )
                    .unwrap(),
            )
            .unwrap()
    }
    pub fn contains_id(&self, id: i64) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(
                        &self.instance,
                        "contains",
                        &[InvocationArg::try_from(id)
                            .unwrap()
                            .into_primitive()
                            .unwrap()],
                    )
                    .unwrap(),
            )
            .unwrap()
    }
    pub fn get(self, id: i64) -> Option<T> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "get",
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        if !instance_is_null(&instance) {
            Some(T::from_instance(instance))
        } else {
            None
        }
    }
    pub fn get_size(&self) -> usize {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "getSize", &[])
                    .unwrap(),
            )
            .unwrap()
    }
    pub fn is_empty(&self) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "isEmpty", &[])
                    .unwrap(),
            )
            .unwrap()
    }
    pub fn to_string(&self) -> String {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "toString", &[])
                    .unwrap(),
            )
            .unwrap()
    }
}

impl<T: ContactTrait + FromInstance> MessageHashCodeTrait for ContactList<T> {}
// TODO: impl MiraiRsCollectionTrait fot ContactList<_>{}
