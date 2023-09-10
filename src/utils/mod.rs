use contact_derive::GetInstanceDerive;
use j4rs::{Instance, InvocationArg, Jvm};
use crate::env::GetEnvTrait;
use crate::message::message_trait::SingleMessageTrait;

pub trait MiraiRsCollectionTrait {
    type Element;
    fn get_size(&self) -> i32;
    fn is_empty(&self) -> bool;
    fn contains(&self, element: &Self::Element) -> bool;
    fn contains_all(&self, elements: Self) -> bool;
}

pub trait MiraiRsIterableTrait: Iterator {}


pub(crate) fn get_bytes_md5_and_cast_to_i8_16(jvm: Jvm, instance: &Instance) -> [i8; 16] {
    let bytes = jvm.invoke(&instance, "getMd5", &[]).unwrap();
    let instance = jvm
        .invoke_static(
            "org.apache.commons.lang3.ArrayUtils",
            "toObject",
            &[InvocationArg::try_from(bytes).unwrap()],
        )
        .unwrap();
    let instance = jvm
        .invoke_static(
            "java.util.Array",
            "stream",
            &[InvocationArg::try_from(instance).unwrap()],
        )
        .unwrap();
    jvm.chain(&instance)
        .unwrap()
        .invoke("toList", &[])
        .unwrap()
        .to_rust()
        .unwrap()
}