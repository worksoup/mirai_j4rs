use crate::message::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MessageContentTrait, MessageTrait,
    SingleMessageTrait,
};
use j4rs::Instance;
use mj_macro::{java_type, FromInstanceDerive, GetInstanceDerive};

// TODO: 低优先级
#[derive(GetInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.message.data.VipFace")]
pub struct VipFace {
    instance: Instance,
}

impl MessageTrait for VipFace {}

impl SingleMessageTrait for VipFace {}

impl MessageContentTrait for VipFace {}

impl ConstrainSingleTrait for VipFace {}

impl CodableMessageTrait for VipFace {}
