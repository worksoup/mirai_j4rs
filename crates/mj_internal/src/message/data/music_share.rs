use j4rs::Instance;

use mj_helper_macro::mj_all;

use crate::message::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MessageContentTrait, MessageTrait,
    SingleMessageTrait,
};

#[mj_all("message.data.MusicShare")]
pub struct MusicShare {
    instance: Instance,
}

impl MessageTrait for MusicShare {}

impl SingleMessageTrait for MusicShare {}

impl MessageContentTrait for MusicShare {}

impl ConstrainSingleTrait for MusicShare {}

impl CodableMessageTrait for MusicShare {}
