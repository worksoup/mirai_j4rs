use crate::message::data::dice::Dice;
use crate::message::data::face::Face;
use crate::message::data::file_message::FileMessage;
use crate::message::data::forward_message::ForwardMessage;
use crate::message::data::image::Image;
use crate::message::data::light_app::LightApp;
use crate::message::data::market_face::MarketFace;
use crate::message::data::music_share::MusicShare;
use crate::message::data::poke_message::PokeMessage;
use crate::message::data::rock_paper_scissors::RockPaperScissors;
use crate::message::data::super_face::SuperFace;
use crate::message::data::unsupported_message::UnsupportedMessage;
use crate::message::data::vip_face::VipFace;
use crate::{
    message::{
        data::{
            at::At, at_all::AtAll, message_source::MessageSource, plain_text::PlainText,
            quote_reply::QuoteReply, single_message::SingleMessage,
        },
        message_trait::{CodableMessageTrait, MessageChainTrait, MessageTrait},
    },
    utils::MiraiRsCollectionTrait,
};
use j4rs::{Instance, InvocationArg, Jvm};
use mjbase::{
    env::{FromInstance, GetEnvTrait},
    utils::is_instance_of,
};
use mjmacro::GetInstanceDerive;

#[derive(GetInstanceDerive)]
pub struct MessageChain {
    pub(crate) instance: Instance,
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

#[derive(GetInstanceDerive)]
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
            fn instance_to_single_message_enum(jvm: &Jvm, instance: Instance) -> SingleMessage {
                if is_instance_of(&instance, "net.mamoe.mirai.message.data.At") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.At")
                        .unwrap();
                    SingleMessage::At(At::from_instance(instance))
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.AtAll") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.AtAll")
                        .unwrap();
                    SingleMessage::AtAll(AtAll::from_instance(instance))
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.Face") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.Face")
                        .unwrap();
                    SingleMessage::Face(Face::from_instance(instance))
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.FileMessage") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.FileMessage")
                        .unwrap();
                    SingleMessage::FileMessage(FileMessage::from_instance(instance))
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.ForwardMessage") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.ForwardMessage")
                        .unwrap();
                    SingleMessage::ForwardMessage(ForwardMessage::from_instance(instance))
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.Image") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.Image")
                        .unwrap();
                    SingleMessage::Image(Image::from_instance(instance))
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.LightApp") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.LightApp")
                        .unwrap();
                    SingleMessage::LightApp(LightApp::from_instance(instance))
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.MessageSource") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.MessageSource")
                        .unwrap();
                    SingleMessage::MessageSource(MessageSource::from_instance(instance))
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.MarketFace") {
                    if is_instance_of(&instance, "net.mamoe.mirai.message.data.Dice") {
                        let instance = jvm
                            .cast(&instance, "net.mamoe.mirai.message.data.Dice")
                            .unwrap();
                        SingleMessage::Dice(Dice::from_instance(instance))
                    } else if is_instance_of(
                        &instance,
                        "net.mamoe.mirai.message.data.RockPaperScissors",
                    ) {
                        let instance = jvm
                            .cast(&instance, "net.mamoe.mirai.message.data.RockPaperScissors")
                            .unwrap();
                        SingleMessage::RockPaperScissors(RockPaperScissors::from_instance(instance))
                    } else {
                        let instance = jvm
                            .cast(&instance, "net.mamoe.mirai.message.data.MarketFace")
                            .unwrap();
                        SingleMessage::MarketFace(MarketFace::from_instance(instance))
                    }
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.MusicShare") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.MusicShare")
                        .unwrap();
                    SingleMessage::MusicShare(MusicShare::from_instance(instance))
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.PlainText") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.PlainText")
                        .unwrap();
                    SingleMessage::PlainText(PlainText::from_instance(instance))
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.PokeMessage") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.PokeMessage")
                        .unwrap();
                    SingleMessage::PokeMessage(PokeMessage::from_instance(instance))
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.QuoteReply") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.QuoteReply")
                        .unwrap();
                    SingleMessage::QuoteReply(QuoteReply::from_instance(instance))
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.SuperFace") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.SuperFace")
                        .unwrap();
                    SingleMessage::SuperFace(SuperFace::from_instance(instance))
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.VipFace") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.VipFace")
                        .unwrap();
                    SingleMessage::VipFace(VipFace::from_instance(instance))
                } else {
                    SingleMessage::UnsupportedMessage(UnsupportedMessage::from_instance(instance))
                }
            }
            Some(instance_to_single_message_enum(&jvm, next))
        } else {
            None
        }
    }
}
