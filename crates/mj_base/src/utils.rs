use crate::env::FromInstanceTrait;
use j4rs::{Instance, InvocationArg, Jvm};
use std::collections::HashSet;
use std::hash::Hash;

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
pub fn i8_16_to_bytes_16(jvm: &Jvm, array: [i8; 16]) -> Instance {
    let mut i8vector = Vec::new();
    for i in array {
        i8vector.push(InvocationArg::try_from(i).unwrap());
    }
    jvm.create_java_array("java.lang.Byte", &i8vector).unwrap()
}

#[inline]
pub fn instance_from_i8_16<const CLASS_TYPE: &'static str>(
    call_from_java_raw_as_i8_16: [i8; 16],
) -> Instance {
    let jvm = Jvm::attach_thread().unwrap();
    let call_from_java_raw_as_java_bytes = i8_16_to_bytes_16(&jvm, call_from_java_raw_as_i8_16);
    jvm.create_instance(
        CLASS_TYPE,
        &[InvocationArg::try_from(call_from_java_raw_as_java_bytes).unwrap()],
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
pub fn java_iter_to_rust_vec<T: FromInstanceTrait>(jvm: &Jvm, iter: Instance) -> Vec<T> {
    let mut res = Vec::new();
    while jvm
        .to_rust(jvm.invoke(&iter, "hasNext", &[]).unwrap())
        .unwrap()
    {
        let next = jvm.invoke(&iter, "next", &[]).unwrap();
        res.push(T::from_instance(next));
    }
    res
}

#[inline]
pub fn java_iter_to_rust_hash_set<T: FromInstanceTrait + Hash + Eq>(
    jvm: &Jvm,
    iter: Instance,
) -> HashSet<T> {
    let mut res = HashSet::new();
    while jvm
        .to_rust(jvm.invoke(&iter, "hasNext", &[]).unwrap())
        .unwrap()
    {
        let next = jvm.invoke(&iter, "next", &[]).unwrap();
        res.insert(T::from_instance(next));
    }
    res
}
