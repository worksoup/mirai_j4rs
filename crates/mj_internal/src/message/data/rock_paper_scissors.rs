use crate::message::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MarketFaceTrait, MessageContentTrait,
    MessageHashCodeTrait, MessageTrait, SingleMessageTrait,
};
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::{
    env::{GetClassTypeTrait as _, GetInstanceTrait as _},
    utils::instance_is_null,
};
use mj_macro::{java_type, FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.message.data.RockPaperScissors")]
pub struct RockPaperScissors {
    instance: Instance,
}

/// 魔法表情猜拳。
/// 新版客户端只能显示动画而不能显示结果。
impl RockPaperScissors {
    fn new(field: &str) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .static_class_field(Self::get_type_name(), field)
            .unwrap();
        Self { instance }
    }
    pub fn rock() -> Self {
        Self::new("ROCK")
    }
    pub fn scissors() -> Self {
        Self::new("SCISSORS")
    }
    pub fn paper() -> Self {
        Self::new("PAPER")
    }
    pub fn equals() {
        todo!("低优先级")
    }
    pub fn eliminates(&self, other: RockPaperScissors) -> Option<bool> {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm
            .invoke(
                &self.instance,
                "eliminates",
                &[InvocationArg::try_from(other.get_instance()).unwrap()],
            )
            .unwrap();
        if !instance_is_null(&result) {
            Some(jvm.to_rust(result).unwrap())
        } else {
            None
        }
    }
    pub fn random() -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static(
                "net.mamoe.mirai.message.data.RockPaperScissors",
                "random",
                &[],
            )
            .unwrap();
        Self { instance }
    }
}

impl MessageTrait for RockPaperScissors {}

impl SingleMessageTrait for RockPaperScissors {}

impl MessageContentTrait for RockPaperScissors {}

impl ConstrainSingleTrait for RockPaperScissors {}

impl CodableMessageTrait for RockPaperScissors {}

impl MessageHashCodeTrait for RockPaperScissors {}

impl MarketFaceTrait for RockPaperScissors {}