use j4rs::{Instance, InvocationArg, Jvm};

use mj_base::env::{FromInstanceTrait, GetInstanceTrait};
use mj_macro::{mj_all, AsInstanceDerive, GetInstanceDerive};

use crate::{
    message::{
        data::single_message::SingleMessage,
        message_trait::{CodableMessageTrait, MessageChainTrait, MessageTrait},
    },
    utils::MiraiRsCollectionTrait,
};

#[mj_all("message.data.MessageChain")]
pub struct MessageChain {
    instance: Instance,
}

impl MessageChain {}

impl MessageTrait for MessageChain {}

impl CodableMessageTrait for MessageChain {}

impl MessageChainTrait for MessageChain {}

impl MiraiRsCollectionTrait for MessageChain {
    type Element = SingleMessage;

    fn get_size(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getSize", &[]).unwrap())
            .unwrap()
    }

    fn is_empty(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "isEmpty", &[]).unwrap())
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

impl IntoIterator for MessageChain {
    type Item = SingleMessage;
    type IntoIter = MessageChainIterator;

    fn into_iter(self) -> Self::IntoIter {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "iterator", &[]).unwrap();
        Self::IntoIter { instance }
    }
}

#[derive(AsInstanceDerive, GetInstanceDerive)]
pub struct MessageChainIterator {
    instance: Instance,
}

impl Iterator for MessageChainIterator {
    type Item = SingleMessage;

    fn next(&mut self) -> Option<Self::Item> {
        let jvm = Jvm::attach_thread().unwrap();
        let has_next = jvm
            .to_rust(jvm.invoke(&self.instance, "hasNext", &[]).unwrap())
            .unwrap();
        // 逻辑如下：
        // if hasNext
        //     return Some(next)
        // else return None
        if has_next {
            let next = jvm.invoke(&self.instance, "next", &[]).unwrap();
            let class_type: String = jvm
                .chain(&next)
                .unwrap()
                .invoke("getClass", &[])
                .unwrap()
                .invoke("toString", &[])
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
