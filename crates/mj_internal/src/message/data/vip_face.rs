use crate::message::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MessageContentTrait, MessageTrait,
    SingleMessageTrait,
};
use j4rs::Instance;
use mj_macro::{FromInstanceDerive, GetInstanceDerive};

// TODO: 低优先级
#[derive(GetInstanceDerive, FromInstanceDerive)]
pub struct VipFace {
    instance: Instance,
}

impl MessageTrait for VipFace {}

impl SingleMessageTrait for VipFace {}

impl MessageContentTrait for VipFace {}

impl ConstrainSingleTrait for VipFace {}

impl CodableMessageTrait for VipFace {}
