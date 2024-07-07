use j4rs::{Instance, InvocationArg, Jvm};

use mj_base::env::{FromInstanceTrait, TryFromInstanceTrait as _};

use crate::{
    contact::ContactTrait,
    error::MiraiRsError,
    message::{data::QuoteReply, message_trait::MessageTrait},
};

pub struct MessageReceipt<'a, T>
where
    T: ContactTrait,
{
    instance: Instance,
    target: &'a T,
}

impl<'a, T> MessageReceipt<'a, T>
where
    T: ContactTrait,
{
    pub(crate) fn new(instance: Instance, target: &'a T) -> Self {
        MessageReceipt { instance, target }
    }
    pub fn get_target(&self) -> &T {
        self.target
    }
    pub fn get_source(&self) -> () {
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
    pub fn quote(&self) -> QuoteReply {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "quote", InvocationArg::empty())
            .unwrap();
        QuoteReply::from_instance(instance)
    }
    pub fn quote_reply(&self, message: impl MessageTrait) -> QuoteReply {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "quote", InvocationArg::empty())
            .unwrap();
        QuoteReply::from_instance(instance)
    }
    pub fn quote_reply_string(&self, message: String) -> QuoteReply {
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
        let instance = jvm.invoke(&self.instance, "recallIn", &[millis]).unwrap();
        // TODO: 获取撤回结果并返回。
        Ok(())
    }
}
