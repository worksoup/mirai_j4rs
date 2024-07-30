use j4rs::{Instance, InvocationArg, Jvm};

use jbuchong::GetClassTypeTrait;
use jbuchong::{utils::instance_is_null, GetInstanceTrait as _};
use mj_helper_macro::mj_all;

use crate::message::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MarketFaceTrait, MessageContentTrait,
    MessageHashCodeTrait, MessageTrait, SingleMessageTrait,
};
use crate::utils::backend::BotBackend;

#[mj_all("message.data.RockPaperScissors")]
pub struct RockPaperScissors<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

/// 魔法表情猜拳。
/// 新版客户端只能显示动画而不能显示结果。
impl<B: BotBackend> RockPaperScissors<B> {
    fn new(field: &str) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .static_class_field(<Self as GetClassTypeTrait>::get_type_name(), field)
            .unwrap();
        Self {
            instance,
            _backend: B::default(),
        }
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
    pub fn eliminates(&self, other: RockPaperScissors<B>) -> Option<bool> {
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
        Self {
            instance,
            _backend: B::default(),
        }
    }
}

impl<B: BotBackend> MessageTrait<B> for RockPaperScissors<B> {}

impl<B: BotBackend> SingleMessageTrait<B> for RockPaperScissors<B> {}

impl<B: BotBackend> MessageContentTrait<B> for RockPaperScissors<B> {}

impl<B: BotBackend> ConstrainSingleTrait<B> for RockPaperScissors<B> {}

impl<B: BotBackend> CodableMessageTrait<B> for RockPaperScissors<B> {}

impl<B: BotBackend> MessageHashCodeTrait for RockPaperScissors<B> {}

impl<B: BotBackend> MarketFaceTrait<B> for RockPaperScissors<B> {}
