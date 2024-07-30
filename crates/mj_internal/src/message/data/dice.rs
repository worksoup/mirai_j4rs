use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{FromInstanceTrait, GetClassTypeTrait};
use mj_helper_macro::mj_all;

use crate::message::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MarketFaceTrait, MessageContentTrait,
    MessageHashCodeTrait, MessageTrait, SingleMessageTrait,
};
use crate::utils::backend::BotBackend;

#[mj_all("message.data.Dice")]
pub struct Dice<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

/// 魔法表情骰子。
/// 新版客户端只能显示动画而不能显示点数。
impl<B: BotBackend> Dice<B> {
    pub fn new(mut value: u8) -> Self {
        if value > 6 {
            value = 1;
        }
        let value = value as i32;
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                <Self as GetClassTypeTrait>::get_type_name(),
                &[InvocationArg::try_from(value)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        Self::from_instance(instance)
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

impl<B: BotBackend> MessageTrait<B> for Dice<B> {}

impl<B: BotBackend> SingleMessageTrait<B> for Dice<B> {}

impl<B: BotBackend> MessageContentTrait<B> for Dice<B> {}

impl<B: BotBackend> ConstrainSingleTrait<B> for Dice<B> {}

impl<B: BotBackend> CodableMessageTrait<B> for Dice<B> {}

impl<B: BotBackend> MessageHashCodeTrait for Dice<B> {}

impl<B: BotBackend> MarketFaceTrait<B> for Dice<B> {}
