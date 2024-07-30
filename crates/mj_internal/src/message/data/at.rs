use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{java_all, AsInstanceTrait, GetClassTypeTrait, GetInstanceTrait};

use crate::utils::backend::BotBackend;
use crate::{
    contact::Group,
    message::message_trait::{
        CodableMessageTrait, MessageContentTrait, MessageHashCodeTrait, MessageTrait,
        SingleMessageTrait,
    },
};

fn get_target_(instance: &Instance) -> i64 {
    let jvm = Jvm::attach_thread().unwrap();
    jvm.to_rust(
        jvm.invoke(instance, "getTarget", InvocationArg::empty())
            .unwrap(),
    )
    .unwrap()
}
#[java_all("net.mamoe.mirai.message.data.At")]
pub struct At<B: BotBackend> {
    #[default(fn_name = get_target_)]
    id: i64,
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> At<B> {
    pub fn new(id: i64) -> Self {
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
        At {
            id,
            instance,
            _backend: B::default(),
        }
    }
    pub fn to_display_string(&self, group: &Group<B>) -> String {
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

impl<B: BotBackend> MessageTrait<B> for At<B> {
    fn to_content(&self) -> String {
        format!("@{}", self.id)
    }
}

impl<B: BotBackend> CodableMessageTrait<B> for At<B> {
    fn to_code(&self) -> String {
        format!("[mirai:at:{}]", self.id)
    }
}

impl<B: BotBackend> SingleMessageTrait<B> for At<B> {}

impl<B: BotBackend> MessageContentTrait<B> for At<B> {}

impl<B: BotBackend> MessageHashCodeTrait for At<B> {
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
