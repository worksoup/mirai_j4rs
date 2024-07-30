use std::{
    fmt::{Display, Formatter},
    marker::PhantomData,
};

use crate::{
    contact::ContactTrait,
    message::MessageHashCodeTrait,
    utils::{backend::BotBackend, MiraiRsCollectionTrait},
};
use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{utils::instance_is_null, GetInstanceTrait, TryFromInstanceTrait};
use mj_helper_macro::mj_all;

#[mj_all("contact.ContactList")]
pub struct ContactList<B: BotBackend, T>
where
    T: ContactTrait<B> + TryFromInstanceTrait,
{
    instance: Instance,
    _backend: B,
    _unused: PhantomData<T>,
}

impl<B: BotBackend, T> ContactList<B, T>
where
    T: ContactTrait<B> + TryFromInstanceTrait,
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
}
impl<B: BotBackend, T> Display for ContactList<B, T>
where
    T: ContactTrait<B> + TryFromInstanceTrait,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: String = Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "toString", InvocationArg::empty())
                    .unwrap(),
            )
            .unwrap();
        f.write_str(s.as_str())
    }
}

impl<B: BotBackend, T: ContactTrait<B> + TryFromInstanceTrait> MessageHashCodeTrait
    for ContactList<B, T>
{
}
impl<B: BotBackend, T: ContactTrait<B> + TryFromInstanceTrait> MiraiRsCollectionTrait
    for ContactList<B, T>
{
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
