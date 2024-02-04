use crate::message::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MessageContentTrait, MessageTrait,
    SingleMessageTrait,
};
use j4rs::Instance;
use mj_macro::{java_type, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

#[derive(AsInstanceDerive, GetInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.message.data.MusicShare")]
pub struct MusicShare {
    instance: Instance,
}

impl MessageTrait for MusicShare {}

impl SingleMessageTrait for MusicShare {}

impl MessageContentTrait for MusicShare {}

impl ConstrainSingleTrait for MusicShare {}

impl CodableMessageTrait for MusicShare {}
