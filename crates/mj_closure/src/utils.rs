use crate::RawPointer;
use j4rs::{Instance, InvocationArg, Jvm};

#[inline]
pub fn raw_pointer_to_instance<const CLASS_TYPE: &'static str>(
    call_from_java_raw_as_i8_16: RawPointer,
) -> Instance {
    let jvm = Jvm::attach_thread().unwrap();
    let call_from_java_raw_as_java_bytes =
        raw_pointer_to_bytes_instance(&jvm, call_from_java_raw_as_i8_16);
    jvm.create_instance(
        CLASS_TYPE,
        &[InvocationArg::from(call_from_java_raw_as_java_bytes)],
    )
    .unwrap()
}

#[inline]
pub fn raw_pointer_to_bytes_instance(jvm: &Jvm, array: RawPointer) -> Instance {
    let mut i8vector = Vec::new();
    for i in array {
        i8vector.push(InvocationArg::try_from(i).unwrap());
    }
    jvm.create_java_array("java.lang.Byte", &i8vector).unwrap()
}
