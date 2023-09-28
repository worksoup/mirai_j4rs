use crate::utils::other::enums::MiraiProtocol;
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

pub(super) fn i8_16_to_bytes_16<E>(jvm: &Jvm, array: [i8; 16]) -> Instance {
    let mut i8vector = Vec::new();
    for i in array {
        i8vector.push(InvocationArg::try_from(i).unwrap());
    }
    jvm.create_java_array("java.lang.Byte", &i8vector).unwrap()
}

pub(crate) fn is_instance_of(instance: &Instance, class_name: &str) -> bool {
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

pub(crate) fn java_println(val: &Instance) {
    let jvm = Jvm::attach_thread().unwrap();
    let _ = jvm
        .invoke(
            &jvm.static_class_field("java.lang.System", "out").unwrap(),
            "println",
            &[InvocationArg::try_from(jvm.clone_instance(val).unwrap()).unwrap()],
        )
        .unwrap();
}

pub(crate) fn instance_is_null(instance: &Instance) -> bool {
    Jvm::attach_thread()
        .unwrap()
        .to_rust(
            Jvm::attach_thread()
                .unwrap()
                .invoke_static(
                    "java.util.Objects",
                    "isNull",
                    &[InvocationArg::try_from(
                        Jvm::attach_thread()
                            .unwrap()
                            .clone_instance(instance)
                            .unwrap(),
                    )
                        .unwrap()],
                )
                .unwrap(),
        )
        .unwrap()
}

pub(crate) fn protocol_enum_r2j(
    protocol: MiraiProtocol,
) -> Result<InvocationArg, std::convert::Infallible> {
    InvocationArg::try_from(
        Jvm::attach_thread()
            .unwrap()
            .field(
                &Jvm::attach_thread()
                    .unwrap()
                    .static_class("net.mamoe.mirai.utils.BotConfiguration$MiraiProtocol")
                    .unwrap(),
                match protocol {
                    MiraiProtocol::A => "ANDROID_PHONE",
                    MiraiProtocol::I => "IPAD",
                    MiraiProtocol::M => "MACOS",
                    MiraiProtocol::P => "ANDROID_PAD",
                    MiraiProtocol::W => "ANDROID_WATCH",
                },
            )
            .unwrap(),
    )
}

pub(crate) fn protocol_str2enum(protocol: String) -> MiraiProtocol {
    match protocol.as_str() {
        "ANDROID_PHONE" => MiraiProtocol::A,
        "IPAD" => MiraiProtocol::I,
        "MACOS" => MiraiProtocol::M,
        "ANDROID_PAD" => MiraiProtocol::P,
        "ANDROID_WATCH" => MiraiProtocol::W,
        _ => {
            println!("&self.instance is None!");
            MiraiProtocol::A
        }
    }
}
