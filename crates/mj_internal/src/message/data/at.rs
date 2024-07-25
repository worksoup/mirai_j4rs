use j4rs::{errors::J4RsError, Instance, InvocationArg, Jvm};
use jbuchong::{java, AsInstanceTrait, GetClassTypeTrait, GetInstanceTrait, TryFromInstanceTrait};

use crate::{
    contact::Group,
    message::message_trait::{
        CodableMessageTrait, MessageContentTrait, MessageHashCodeTrait, MessageTrait,
        SingleMessageTrait,
    },
};

#[java("net.mamoe.mirai.message.data.At")]
pub struct At {
    id: i64,
    instance: Instance,
}

impl At {
    pub fn new(id: i64) -> At {
        let instance = Jvm::attach_thread()
            .unwrap()
            .create_instance(
                <Self as GetClassTypeTrait>::get_type_name(),
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        At { id, instance }
    }
    pub fn to_display_string(&self, group: &Group) -> String {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(
                        self.as_instance(),
                        "getDisplay",
                        &[InvocationArg::try_from(group.get_instance()).unwrap()],
                    )
                    .unwrap(),
            )
            .unwrap()
    }
    pub fn get_target(&self) -> i64 {
        self.id
    }
}

impl MessageTrait for At {
    fn to_content(&self) -> String {
        format!("@{}", self.id)
    }
}

impl CodableMessageTrait for At {
    fn to_code(&self) -> String {
        format!("[mirai:at:{}]", self.id)
    }
}

impl SingleMessageTrait for At {}

impl MessageContentTrait for At {}

impl MessageHashCodeTrait for At {
    /// # 说明
    /// java.lang.Long 里的实现：
    /// ``` java
    /// public static int hashCode(long value) {
    ///     return (int)(value ^ (value >>> 32));
    /// }
    /// ```
    fn hash_code(&self) -> i32 {
        (self.id ^ (self.id/*i64*/ >> 32)) as i32
    }
}

impl TryFromInstanceTrait for At {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        let jvm = Jvm::attach_thread().unwrap();
        Ok(Self {
            id: jvm
                .to_rust(
                    jvm.invoke(&instance, "getTarget", InvocationArg::empty())
                        .unwrap(),
                )
                .unwrap(),
            instance,
        })
    }
}
