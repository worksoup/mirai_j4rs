use crate::env::FromInstance;
use crate::utils::other::enums::MiraiProtocol;
use j4rs::{Instance, InvocationArg, Jvm};
use std::collections::HashSet;
use std::hash::Hash;

#[inline]
pub fn get_bytes_md5_and_cast_to_i8_16(jvm: Jvm, instance: &Instance) -> [i8; 16] {
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
pub fn protocol_enum_r2j(
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

#[inline]
pub fn protocol_str2enum(protocol: String) -> MiraiProtocol {
    match protocol.as_str() {
        "ANDROID_PHONE" => MiraiProtocol::A,
        "IPAD" => MiraiProtocol::I,
        "MACOS" => MiraiProtocol::M,
        "ANDROID_PAD" => MiraiProtocol::P,
        "ANDROID_WATCH" => MiraiProtocol::W,
        _ => {
            eprintln!("协议枚举转换失败，默认转换结果为安卓协议。");
            MiraiProtocol::A
        }
    }
}

#[inline]
pub fn java_iter_to_rust_vec<T: FromInstance>(jvm: &Jvm, iter: Instance) -> Vec<T> {
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
pub fn java_iter_to_rust_hash_set<T: FromInstance + Hash + Eq>(
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

/// 请注意 close.
#[inline]
pub fn external_resource_from_file(jvm: &Jvm, path: &str) -> Instance {
    jvm.invoke_static(
        "net.mamoe.mirai.utils.ExternalResource",
        "create",
        &[InvocationArg::try_from(
            Jvm::attach_thread()
                .unwrap()
                .create_instance("java.io.File", &[InvocationArg::try_from(path).unwrap()])
                .unwrap(),
        )
            .unwrap()],
    )
        .unwrap()
}

#[inline]
pub fn external_resource_close(jvm: &Jvm, resource: Instance) {
    let _ = jvm.invoke(&resource, "close", &[]);
}

pub mod data_wrapper {
    use crate::env::{FromInstance, GetEnvTrait};
    use j4rs::{Instance, InvocationArg, Jvm};
    use std::ops::Deref;

    pub struct DataWrapper<T> {
        data: T,
    }

    impl<T> Deref for DataWrapper<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    impl<T> From<T> for DataWrapper<T> {
        fn from(data: T) -> Self {
            Self { data }
        }
    }

    impl FromInstance for DataWrapper<String> {
        fn from_instance(instance: Instance) -> Self {
            let jvm = Jvm::attach_thread().unwrap();
            jvm.to_rust::<String>(instance).unwrap().into()
        }
    }

    impl GetEnvTrait for DataWrapper<String> {
        fn get_instance(&self) -> Instance {
            Instance::try_from(InvocationArg::try_from(&self.data).unwrap()).unwrap()
        }
    }

    impl FromInstance for DataWrapper<Vec<i8>> {
        fn from_instance(instance: Instance) -> Self {
            let jvm = Jvm::attach_thread().unwrap();
            jvm.to_rust::<Vec<_>>(instance).unwrap().into()
        }
    }

    impl<P1, P2> DataWrapper<(P1, P2)>
        where
            P1: FromInstance,
            P2: FromInstance,
    {
        pub fn get_pair(self) -> (P1, P2) {
            self.data
        }
    }

    impl<P1, P2> FromInstance for DataWrapper<(P1, P2)>
        where
            P1: FromInstance,
            P2: FromInstance,
    {
        fn from_instance(instance: Instance) -> Self {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = jvm.cast(&instance, "kotlin.Pair").unwrap();
            let val1 = jvm.invoke(&instance, "getFirst", &[]).unwrap();
            let val2 = jvm.invoke(&instance, "getSecond", &[]).unwrap();
            let val1 = P1::from_instance(val1);
            let val2 = P2::from_instance(val2);
            Self { data: (val1, val2) }
        }
    }

    impl DataWrapper<Instance> {
        pub fn get<E>(&self) -> E
            where
                E: FromInstance,
        {
            E::from_instance(
                Jvm::attach_thread()
                    .unwrap()
                    .clone_instance(&self.data)
                    .unwrap(),
            )
        }
    }

    impl GetEnvTrait for DataWrapper<Instance> {
        fn get_instance(&self) -> Instance {
            let jvm = Jvm::attach_thread().unwrap();
            jvm.clone_instance(&self.data).unwrap()
        }
    }

    impl FromInstance for DataWrapper<Instance> {
        fn from_instance(instance: Instance) -> Self {
            Self { data: instance }
        }
    }

    impl FromInstance for DataWrapper<()> {
        fn from_instance(_instance: Instance) -> Self {
            Self { data: () }
        }
    }

    impl GetEnvTrait for DataWrapper<()> {
        fn get_instance(&self) -> Instance {
            let jvm = Jvm::attach_thread().unwrap();
            jvm.invoke_static("javax.lang.model.util.Types", "getNullType", &[])
                .unwrap()
        }
    }

    impl<QrCodeLoginListener: FromInstance> FromInstance for DataWrapper<QrCodeLoginListener> {
        fn from_instance(instance: Instance) -> Self {
            <QrCodeLoginListener as FromInstance>::from_instance(instance).into()
        }
    }

    impl<QrCodeLoginListener: GetEnvTrait> GetEnvTrait for DataWrapper<QrCodeLoginListener> {
        fn get_instance(&self) -> Instance {
            <QrCodeLoginListener as GetEnvTrait>::get_instance(self)
        }
    }
}
