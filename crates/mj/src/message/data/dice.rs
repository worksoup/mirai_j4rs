use crate::message::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MarketFaceTrait, MessageContentTrait,
    MessageHashCodeTrait, MessageTrait, SingleMessageTrait,
};
use j4rs::{Instance, InvocationArg, Jvm};
use mjmacro::{FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, FromInstanceDerive)]
pub struct Dice {
    instance: Instance,
}

impl Dice {
    /// 竟然可以直接指定值，太离谱了。。。
    /// 不知道新版 QQ 这个表情还能用不能。需要测试。
    /// TODO: 测试。
    pub fn new(mut value: u8) -> Self {
        if value > 6 {
            value = 1;
        }
        let value = value as i32;
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                "net.mamoe.mirai.message.data.Dice",
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
