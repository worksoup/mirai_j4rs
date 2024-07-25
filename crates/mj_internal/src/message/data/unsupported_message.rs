use j4rs::Instance;
use jbuchong::java_all;

use crate::message::message_trait::{MessageContentTrait, MessageTrait, SingleMessageTrait};

#[java_all]
pub struct UnsupportedMessage {
    instance: Instance,
}

impl MessageTrait for UnsupportedMessage {}

impl SingleMessageTrait for UnsupportedMessage {}

impl MessageContentTrait for UnsupportedMessage {}
