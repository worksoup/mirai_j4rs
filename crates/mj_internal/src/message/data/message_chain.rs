use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{java_all, FromInstanceTrait, GetInstanceTrait};
use mj_helper_macro::mj_all;

use crate::utils::backend::BotBackend;
use crate::{
    message::{
        data::single_message::SingleMessage,
        message_trait::{CodableMessageTrait, MessageChainTrait, MessageTrait},
    },
    utils::MiraiRsCollectionTrait,
};

#[mj_all("message.data.MessageChain")]
pub struct MessageChain<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> MessageChain<B> {}

impl<B: BotBackend> MessageTrait<B> for MessageChain<B> {}

impl<B: BotBackend> CodableMessageTrait<B> for MessageChain<B> {}

impl<B: BotBackend> MessageChainTrait<B> for MessageChain<B> {}

impl<B: BotBackend> MiraiRsCollectionTrait for MessageChain<B> {
    type Element = SingleMessage<B>;

    fn get_size(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getSize", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }

    fn is_empty(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "isEmpty", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }

    fn contains(&self, element: &Self::Element) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let element = InvocationArg::try_from(element.get_instance()).unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "contains", &[element]).unwrap())
            .unwrap()
    }

    fn contains_all(&self, elements: Self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let elements = InvocationArg::try_from(elements.get_instance()).unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "contains", &[elements]).unwrap())
            .unwrap()
    }
}
impl<B: BotBackend> MessageChain<B> {
    pub fn iter(&self) -> MessageChainIter<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "iterator", InvocationArg::empty())
            .unwrap();
        MessageChainIter::from_instance(instance)
    }
}
impl<B: BotBackend> IntoIterator for MessageChain<B> {
    type Item = SingleMessage<B>;
    type IntoIter = MessageChainIter<B>;

    fn into_iter(self) -> Self::IntoIter {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "iterator", InvocationArg::empty())
            .unwrap();
        Self::IntoIter::from_instance(instance)
    }
}
#[java_all]
pub struct MessageChainIter<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> Iterator for MessageChainIter<B> {
    type Item = SingleMessage<B>;

    fn next(&mut self) -> Option<Self::Item> {
        let jvm = Jvm::attach_thread().unwrap();
        let has_next = jvm
            .to_rust(
                jvm.invoke(&self.instance, "hasNext", InvocationArg::empty())
                    .unwrap(),
            )
            .unwrap();
        // 逻辑如下：
        // if hasNext
        //     return Some(next)
        // else return None
        if has_next {
            let next = jvm
                .invoke(&self.instance, "next", InvocationArg::empty())
                .unwrap();
            let class_type: String = jvm
                .chain(&next)
                .unwrap()
                .invoke("getClass", InvocationArg::empty())
                .unwrap()
                .invoke("toString", InvocationArg::empty())
                .unwrap()
                .to_rust()
                .unwrap();
            println!("消息类型：{class_type}");
            Some(SingleMessage::from_instance(next))
        } else {
            None
        }
    }
}
