use std::marker::PhantomData;

use j4rs::errors::J4RsError;
use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{TryFromInstanceTrait, GetInstanceTrait};
use jbuchong::utils::instance_is_null;

use crate::contact::ContactTrait;
use crate::message::MessageHashCodeTrait;
use crate::utils::MiraiRsCollectionTrait;

pub struct ContactList<T>
where
    T: ContactTrait + TryFromInstanceTrait,
{
    pub(crate) instance: Instance,
    pub(crate) _unused: PhantomData<T>,
}

impl<T: ContactTrait + TryFromInstanceTrait> TryFromInstanceTrait for ContactList<T> {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        Ok(ContactList {
            instance,
            _unused: PhantomData::default(),
        })
    }
}

impl<T: ContactTrait + TryFromInstanceTrait> GetInstanceTrait for ContactList<T> {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        Ok(Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap())
    }
}

impl<T> ContactList<T>
where
    T: ContactTrait + TryFromInstanceTrait,
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
            T::try_from_instance(instance).ok()
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
                    .invoke(&self.instance, "getSize", InvocationArg::empty())
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
                    .invoke(&self.instance, "isEmpty", InvocationArg::empty())
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
                    .invoke(&self.instance, "toString", InvocationArg::empty())
                    .unwrap(),
            )
            .unwrap()
    }
}

impl<T: ContactTrait + TryFromInstanceTrait> MessageHashCodeTrait for ContactList<T> {}
impl<T: ContactTrait + TryFromInstanceTrait> MiraiRsCollectionTrait for ContactList<T> {
    type Element = T;

    fn get_size(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getSize", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }

    fn is_empty(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "isEmpty", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }

    fn contains(&self, element: &Self::Element) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let element = InvocationArg::try_from(element.get_instance()).unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "contains", &[element]).unwrap())
            .unwrap()
    }

    fn contains_all(&self, elements: Self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let elements = InvocationArg::try_from(elements.get_instance()).unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "contains", &[elements]).unwrap())
            .unwrap()
    }
}
