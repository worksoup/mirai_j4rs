use crate::message::{
    ConstrainSingleTrait, MessageMetaDataTrait, MessageTrait, SingleMessageTrait,
};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("message.data.MessageOrigin")]
pub struct MessageOrigin {
    instance: Instance,
}

impl MessageMetaDataTrait for MessageOrigin {}
impl ConstrainSingleTrait for MessageOrigin {}
impl SingleMessageTrait for MessageOrigin {}
impl MessageTrait for MessageOrigin {}
