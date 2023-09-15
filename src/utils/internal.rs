use j4rs::{Instance, InvocationArg, Jvm};

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

pub(crate) fn i8_16_to_bytes_16<E>(jvm: &Jvm, array: [i8; 16]) -> Instance {
    let mut i8vector = Vec::new();
    for i in array {
        i8vector.push(InvocationArg::try_from(i).unwrap());
    }
    jvm.create_java_array("java.lang.Byte", &i8vector)
        .unwrap()
}

pub(crate) fn is_instance_of(instance: &Instance, class_name: &str) -> bool {
    let jvm = Jvm::attach_thread().unwrap();
    let instance = jvm.clone_instance(instance).unwrap();
    let instance = InvocationArg::try_from(instance).unwrap();
    let class_name = InvocationArg::try_from(class_name).unwrap();
    jvm.to_rust(
        jvm.invoke_static(
            "rt.lea.LumiaUtils",
            "isInstanceOf",
            &[instance, class_name],
        )
            .unwrap(),
    )
        .unwrap()
}