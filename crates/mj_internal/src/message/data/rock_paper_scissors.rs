use j4rs::{Instance, InvocationArg, Jvm};

use jbuchong::GetClassTypeTrait;
use jbuchong::{GetInstanceTrait as _, utils::instance_is_null};
use mj_helper_macro::mj_all;

use crate::message::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MarketFaceTrait, MessageContentTrait,
    MessageHashCodeTrait, MessageTrait, SingleMessageTrait,
};

#[mj_all("message.data.RockPaperScissors")]
pub struct RockPaperScissors {
    instance: Instance,
}

/// 魔法表情猜拳。
/// 新版客户端只能显示动画而不能显示结果。
impl RockPaperScissors {
    fn new(field: &str) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .static_class_field(<Self as GetClassTypeTrait>::get_type_name(), field)
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
                <Self as GetClassTypeTrait>::get_type_name(),
                "random",
                InvocationArg::empty(),
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
