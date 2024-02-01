use crate::message::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MarketFaceTrait, MessageContentTrait,
    MessageHashCodeTrait, MessageTrait, SingleMessageTrait,
};
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::GetEnvTrait as _;
use mj_base::utils::instance_is_null;
use mj_macro::{FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, FromInstanceDerive)]
pub struct RockPaperScissors {
    instance: Instance,
}

impl RockPaperScissors {
    fn new(field: &str) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .static_class_field("net.mamoe.mirai.message.data.RockPaperScissors", field)
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
