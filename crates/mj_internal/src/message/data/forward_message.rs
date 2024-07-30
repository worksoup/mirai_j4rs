use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{java_all, FromInstanceTrait, GetClassTypeTrait, GetInstanceTrait};

use mj_helper_macro::mj_all;

use crate::utils::backend::BotBackend;
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

#[mj_all("message.data.ForwardMessageBuilder")]
pub struct ForwardMessageBuilder<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> ForwardMessageBuilder<B> {
    pub fn new(contact: &impl ContactTrait<B>) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let contact = contact.get_instance();
        let contact = InvocationArg::try_from(contact).unwrap();
        let instance = jvm
            .create_instance(<Self as GetClassTypeTrait>::get_type_name(), &[contact])
            .unwrap();
        Self::from_instance(instance)
    }
    pub fn add(
        self,
        user_or_bot: &impl UserOrBotTrait<B>,
        message: &impl MessageTrait<B>,
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
        message: &impl MessageTrait<B>,
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
        _title: String,
        _brief: String,
        _source: String,
        _preview: Vec<String>,
        _summary: String,
    ) -> Self {
        todo!("set_display_strategy")
    }

    pub fn build(&self) -> ForwardMessage<B> {
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
pub struct ForwardMessage<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

#[java_all]
pub struct ForwardMessageNode<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> ForwardMessageNode<B> {
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
    pub fn get_message_chain(&self) -> MessageChain<B> {
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

impl<B: BotBackend> MessageHashCodeTrait for ForwardMessageNode<B> {}

impl<B: BotBackend> ForwardMessage<B> {
    pub fn get_brief(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let brief = jvm
            .invoke(&self.instance, "getBrief", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(brief).unwrap()
    }
    pub fn get_node_vector(&self) -> Vec<ForwardMessageNode<B>> {
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
            node_vector.push(ForwardMessageNode::from_instance(next))
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

impl<B: BotBackend> MessageTrait<B> for ForwardMessage<B> {}

impl<B: BotBackend> SingleMessageTrait<B> for ForwardMessage<B> {}

impl<B: BotBackend> MessageContentTrait<B> for ForwardMessage<B> {}

impl<B: BotBackend> ConstrainSingleTrait<B> for ForwardMessage<B> {}

impl<B: BotBackend> MessageHashCodeTrait for ForwardMessage<B> {}
