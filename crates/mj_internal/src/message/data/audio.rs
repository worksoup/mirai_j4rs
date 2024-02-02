use crate::message::message_trait::{
    AudioTrait, CodableMessageTrait, ConstrainSingleTrait, MessageContentTrait, MessageTrait,
    SingleMessageTrait,
};
use j4rs::Instance;
use mj_macro::{java_type, FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.message.data.Audio")]
pub struct Audio {
    instance: Instance,
}
impl AudioTrait for Audio {}
impl SingleMessageTrait for Audio {}
impl MessageContentTrait for Audio {}
impl ConstrainSingleTrait for Audio {}
impl CodableMessageTrait for Audio {}
impl MessageTrait for Audio {}
