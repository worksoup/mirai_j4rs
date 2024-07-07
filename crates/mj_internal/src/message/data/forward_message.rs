use j4rs::errors::J4RsError;
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::{FromInstanceTrait, GetClassTypeTrait, GetInstanceTrait, TryFromInstanceTrait};
use mj_helper_macro::mj_all;
use mj_macro::{java_type, AsInstanceDerive, GetInstanceDerive};

use crate::{
    contact::{ContactTrait, UserOrBotTrait},
    message::{
        data::message_chain::MessageChain,
        message_trait::{
            ConstrainSingleTrait, MessageContentTrait, MessageHashCodeTrait, MessageTrait,
            SingleMessageTrait,
        },
    },
};

#[derive(AsInstanceDerive, GetInstanceDerive)]
#[java_type("message.data.ForwardMessageBuilder")]
pub struct ForwardMessageBuilder {
    instance: Instance,
}

impl ForwardMessageBuilder {
    pub fn new(contact: &impl ContactTrait) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let contact = contact.get_instance();
        let contact = InvocationArg::try_from(contact).unwrap();
        let instance = jvm
            .create_instance(
                <Self as GetClassTypeTrait>::get_type_name().as_str(),
                &[contact],
            )
            .unwrap();
        Self { instance }
    }
    pub fn add(
        self,
        user_or_bot: &impl UserOrBotTrait,
        message: &impl MessageTrait,
        time: i32,
    ) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let user_or_bot = InvocationArg::try_from(user_or_bot.get_instance()).unwrap();
        let message = InvocationArg::try_from(message.get_instance()).unwrap();
        let time = InvocationArg::try_from(time)
            .unwrap()
            .into_primitive()
            .unwrap();
        let _ = jvm
            .invoke(&self.instance, "add", &[user_or_bot, message, time])
            .unwrap();
        self
    }
    /// add(sender_id, sender_name, message)
    pub fn add_(
        self,
        sender_id: i64,
        sender_name: &str,
        message: &impl MessageTrait,
        time: i32,
    ) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let builder = InvocationArg::try_from(self.get_instance()).unwrap();
        let sender_id = InvocationArg::try_from(sender_id)
            .unwrap()
            .into_primitive()
            .unwrap();
        let sender_name = InvocationArg::try_from(sender_name).unwrap();
        let message = InvocationArg::try_from(message.get_instance()).unwrap();
        let time = InvocationArg::try_from(time)
            .unwrap()
            .into_primitive()
            .unwrap();
        let _ = jvm
            .invoke_static(
                "rt.lea.LumiaUtils",
                "callAdd_",
                &[builder, sender_id, sender_name, message, time],
            )
            .unwrap();
        self
    }
    pub fn set_display_strategy(
        self,
        title: String,
        brief: String,
        source: String,
        preview: Vec<String>,
        summary: String,
    ) -> Self {
        todo!("set_display_strategy")
    }

    pub fn build(&self) -> ForwardMessage {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "build", InvocationArg::empty())
            .unwrap();
        ForwardMessage::from_instance(instance)
    }
}

// TODO: RawForwardMessage is necessary for set_display_strategy.
// TODO: to_forward_message for message and chain, etc.
#[mj_all("message.data.ForwardMessage")]
pub struct ForwardMessage {
    instance: Instance,
}

#[derive(AsInstanceDerive, GetInstanceDerive)]
pub struct ForwardMessageNode {
    instance: Instance,
}

impl ForwardMessageNode {
    pub fn get_sender_id(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getSenderId", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn get_time(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getTime", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn get_sender_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getSenderName", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn get_message_chain(&self) -> MessageChain {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getMessageChain", InvocationArg::empty())
            .unwrap();
        MessageChain::from_instance(instance)
    }

    pub fn to_string(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "toString", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
}

impl MessageHashCodeTrait for ForwardMessageNode {}

impl ForwardMessage {
    pub fn get_brief(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let brief = jvm
            .invoke(&self.instance, "getBrief", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(brief).unwrap()
    }
    pub fn get_node_vector(&self) -> Vec<ForwardMessageNode> {
        let jvm = Jvm::attach_thread().unwrap();
        let mut node_vector = Vec::new();
        let list = jvm
            .invoke(&self.instance, "getNodeList", InvocationArg::empty())
            .unwrap();
        while {
            let has_next = jvm
                .invoke(&list, "hasNext", InvocationArg::empty())
                .unwrap();
            jvm.to_rust(has_next).unwrap()
        } {
            let next = jvm.invoke(&list, "next", InvocationArg::empty()).unwrap();
            node_vector.push(ForwardMessageNode { instance: next })
        }
        node_vector
    }
    pub fn get_preview(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let preview = jvm
            .invoke(&self.instance, "getPreview", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(preview).unwrap()
    }
    pub fn equals() {
        todo!("低优先级")
    }
    pub fn get_source(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getSource", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn get_summary(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getSummary", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn get_title(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getTitle", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
}

impl MessageTrait for ForwardMessage {}

impl SingleMessageTrait for ForwardMessage {}

impl MessageContentTrait for ForwardMessage {}

impl ConstrainSingleTrait for ForwardMessage {}

impl MessageHashCodeTrait for ForwardMessage {}
