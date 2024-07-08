use j4rs::Instance;

use mj_helper_macro::mj_all;

use crate::message::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MessageContentTrait, MessageTrait,
    SingleMessageTrait,
};

// TODO: 低优先级
#[mj_all("message.data.VipFace")]
pub struct VipFace {
    instance: Instance,
}

impl MessageTrait for VipFace {}

impl SingleMessageTrait for VipFace {}

impl MessageContentTrait for VipFace {}

impl ConstrainSingleTrait for VipFace {}

impl CodableMessageTrait for VipFace {}
