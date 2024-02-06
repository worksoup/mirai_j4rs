use j4rs::{Instance, InvocationArg, Jvm};

use mj_base::env::GetClassTypeTrait;
use mj_macro::mj_all;

use crate::message::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MarketFaceTrait, MessageContentTrait,
    MessageHashCodeTrait, MessageTrait, SingleMessageTrait,
};

#[mj_all("message.data.Dice")]
pub struct Dice {
    instance: Instance,
}

/// 魔法表情骰子。
/// 新版客户端只能显示动画而不能显示点数。
impl Dice {
    pub fn new(mut value: u8) -> Self {
        if value > 6 {
            value = 1;
        }
        let value = value as i32;
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                Self::get_type_name(),
                &[InvocationArg::try_from(value)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        Self { instance }
    }
    pub fn equals() {
        todo!("低优先级")
    }
    pub fn get_value(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.instance)
            .unwrap()
            .invoke("getValue", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    pub fn random() -> Self {
        use rand::prelude::*;
        let value = thread_rng().gen_range(1..=6);
        Self::new(value)
    }
}

impl MessageTrait for Dice {}

impl SingleMessageTrait for Dice {}

impl MessageContentTrait for Dice {}

impl ConstrainSingleTrait for Dice {}

impl CodableMessageTrait for Dice {}

impl MessageHashCodeTrait for Dice {}

impl MarketFaceTrait for Dice {}
