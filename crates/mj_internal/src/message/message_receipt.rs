use j4rs::{Instance, InvocationArg, Jvm};

use jbuchong::{java, FromInstanceTrait};

use crate::utils::backend::BotBackend;
use crate::{
    contact::ContactTrait,
    error::MiraiRsError,
    message::{data::QuoteReply, message_trait::MessageTrait},
};
#[java("net.mamoe.mirai.message.MessageReceipt")]
pub struct MessageReceipt<'a, B: BotBackend, T>
where
    T: ContactTrait<B>,
{
    instance: Instance,
    target: &'a T,
    _backend: B,
}

impl<'a, B: BotBackend, T> MessageReceipt<'a, B, T>
where
    T: ContactTrait<B>,
{
    pub(crate) fn new(instance: Instance, target: &'a T) -> Self {
        MessageReceipt {
            instance,
            target,
            _backend: B::default(),
        }
    }
    pub fn get_target(&self) -> &T {
        self.target
    }
    pub fn get_source(&self) {
        todo!("message.data.OnlineMessageSource.Outgoing")
    }
    pub fn is_to_group(&self) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .chain(&self.instance)
            .unwrap()
            .invoke("isToGroup", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
    }
    pub fn quote(&self) -> QuoteReply<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "quote", InvocationArg::empty())
            .unwrap();
        QuoteReply::from_instance(instance)
    }
    pub fn quote_reply(&self, _message: impl MessageTrait<B>) -> QuoteReply<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "quote", InvocationArg::empty())
            .unwrap();
        QuoteReply::from_instance(instance)
    }
    pub fn quote_reply_string(&self, message: String) -> QuoteReply<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let message = InvocationArg::try_from(message).unwrap();
        let instance = jvm.invoke(&self.instance, "quote", &[message]).unwrap();
        QuoteReply::from_instance(instance)
    }
    pub fn recall(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "recall", InvocationArg::empty())
            .unwrap();
    }
    pub fn recall_in(&self, millis: i64) -> Result<(), MiraiRsError> {
        let jvm = Jvm::attach_thread().unwrap();
        let millis = InvocationArg::try_from(millis)
            .unwrap()
            .into_primitive()
            .unwrap();
        let _instance = jvm.invoke(&self.instance, "recallIn", &[millis]).unwrap();
        // TODO: 获取撤回结果并返回。
        Ok(())
    }
}
