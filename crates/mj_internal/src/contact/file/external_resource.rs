use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::GetClassTypeTrait;
use mj_macro::{java_type, FromInstanceDerive, GetInstanceDerive};

/// 请注意 close.
#[inline]
pub(crate) fn external_resource_from_file(jvm: &Jvm, path: &str) -> Instance {
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
pub(crate) fn external_resource_close(jvm: &Jvm, resource: Instance) {
    let _ = jvm.invoke(&resource, "close", &[]);
}

#[inline]
fn i8_array_to_java_byte_array(vec: &[i8]) -> Instance {
    let jvm = Jvm::attach_thread().unwrap();
    let args: Vec<_> = vec
        .iter()
        .map(|elem| {
            InvocationArg::try_from(elem)
                .unwrap()
                .into_primitive()
                .unwrap()
        })
        .collect();
    jvm.create_java_array("byte", &args).unwrap()
}
#[derive(GetInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.utils.ExternalResource")]
pub struct ExternalResource {
    instance: Instance,
}
impl ExternalResource {
    pub fn create_from_file(path: &str) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = external_resource_from_file(&jvm, path);
        Self { instance }
    }
    pub fn create_from_bytes(bytes: &[i8]) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let bytes = i8_array_to_java_byte_array(bytes);
        let instance = jvm
            .invoke_static(
                Self::get_type_name(),
                "create",
                &[InvocationArg::try_from(bytes).unwrap()],
            )
            .unwrap();
        Self { instance }
    }

    pub fn close(self) {
        let jvm = Jvm::attach_thread().unwrap();
        external_resource_close(&jvm, self.instance)
    }
}
