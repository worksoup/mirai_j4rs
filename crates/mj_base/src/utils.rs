use std::{collections::HashSet, hash::Hash};

use j4rs::{Instance, InvocationArg, Jvm};

use crate::env::TryFromInstanceTrait;

pub fn primitive_byte_array_to_string(jvm: &Jvm, instance: Instance) -> Instance {
    // let instance = jvm.clone_instance(instance).unwrap();
    jvm.invoke_static(
        "rt.lea.LumiaUtils",
        "primitiveByteArrayToString",
        &[InvocationArg::try_from(instance).unwrap()],
    )
    .unwrap()
}

#[inline]
pub fn is_instance_of(instance: &Instance, class_name: &str) -> bool {
    let jvm = Jvm::attach_thread().unwrap();
    let instance = jvm.clone_instance(instance).unwrap();
    let instance = InvocationArg::try_from(instance).unwrap();
    let class_name = InvocationArg::try_from(class_name).unwrap();
    jvm.to_rust(
        jvm.invoke_static("rt.lea.LumiaUtils", "isInstanceOf", &[instance, class_name])
            .unwrap(),
    )
    .unwrap()
}

#[inline]
pub fn java_println(val: &Instance) {
    let jvm = Jvm::attach_thread().unwrap();
    let _ = jvm
        .invoke(
            &jvm.static_class_field("java.lang.System", "out").unwrap(),
            "println",
            &[InvocationArg::try_from(jvm.clone_instance(val).unwrap()).unwrap()],
        )
        .unwrap();
}

#[inline]
pub fn instance_is_null(instance: &Instance) -> bool {
    let jvm = Jvm::attach_thread().unwrap();
    jvm.to_rust(
        jvm.invoke_static(
            "java.util.Objects",
            "isNull",
            &[InvocationArg::try_from(jvm.clone_instance(instance).unwrap()).unwrap()],
        )
        .unwrap(),
    )
    .unwrap()
}

#[inline]
pub fn java_iter_to_rust_vec<T: TryFromInstanceTrait>(jvm: &Jvm, iter: Instance) -> Vec<T> {
    let mut res = Vec::new();
    while jvm
        .to_rust(
            jvm.invoke(&iter, "hasNext", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    {
        let next = jvm.invoke(&iter, "next", InvocationArg::empty()).unwrap();
        if let Ok(ele) = T::try_from_instance(next) {
            res.push(ele);
        }
    }
    res
}

#[inline]
pub fn java_iter_to_rust_hash_set<T: TryFromInstanceTrait + Hash + Eq>(
    jvm: &Jvm,
    iter: Instance,
) -> HashSet<T> {
    let mut res = HashSet::new();
    while jvm
        .to_rust(
            jvm.invoke(&iter, "hasNext", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    {
        let next = jvm.invoke(&iter, "next", InvocationArg::empty()).unwrap();
        if let Ok(ele) = T::try_from_instance(next) {
            res.insert(ele);
        }
    }
    res
}
