use contact_derive::GetInstanceDerive;
use std::hint::unreachable_unchecked;

use super::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MarketFaceTrait, MessageChainTrait,
    MessageContentTrait, MessageHashCodeTrait, MessageTrait, RichMessageTrait, SingleMessageTrait,
};
use crate::contact::bot::{Bot, Env};
use crate::contact::contact_trait::{FileSupportedTrait, UserOrBotTrait};
use crate::env::FromInstance;
use crate::file::AbsoluteFile;
use crate::message::message_trait::MessageMetaDataTrait;
use crate::message::ImageType::{APNG, BMP, GIF, JPG, PNG, UNKNOW};
use crate::utils::internal::is_instance_of;
use crate::utils::MiraiRsCollectionTrait;
use crate::{
    contact::{contact_trait::ContactTrait, group::Group},
    env::GetEnvTrait,
};
use j4rs::{Instance, InvocationArg, Jvm};

#[derive(GetInstanceDerive)]
pub struct QuoteReply {
    instance: Instance,
}

impl MessageMetaDataTrait for QuoteReply {}

impl SingleMessageTrait for QuoteReply {}

impl ConstrainSingleTrait for QuoteReply {}

impl MessageHashCodeTrait for QuoteReply {}

impl QuoteReply {
    pub fn get_source(&self) -> MessageSource {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getSource", &[]).unwrap();
        MessageSource { instance }
    }
}

impl From<MessageChain> for QuoteReply {
    fn from(source_message: MessageChain) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                "net.mamoe.mirai.message.data.QuoteReply",
                &[InvocationArg::try_from(source_message.get_instance()).unwrap()],
            )
            .unwrap();
        Self { instance }
    }
}

impl Clone for MessageSource {
    fn clone(&self) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                "net.mamoe.mirai.message.data.QuoteReply",
                &[InvocationArg::try_from(self.get_instance()).unwrap()],
            )
            .unwrap();
        Self { instance }
    }
}

impl MessageTrait for QuoteReply {}

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
        todo!("net.mamoe.mirai.message.data.OnlineMessageSource.Outgoing")
    }
    pub fn is_to_group(&self) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .chain(&self.instance)
            .unwrap()
            .invoke("isToGroup", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    pub fn quote(&self) -> QuoteReply {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "quote", &[]).unwrap();
        QuoteReply { instance }
    }
    pub fn quote_reply(&self, message: impl MessageTrait) -> () {
        // let jvm = Jvm::attach_thread().unwrap();
        // let instance = jvm
        //     .invoke(
        //         &self.instance,
        //         "quote",
        //         &[],
        //     )
        //     .unwrap();
        // Self { instance, target: &() }
        todo!("不太好办。")
    }
    // TODO: 两个重载。
    pub fn quote_reply_string(&self, message: String) -> () {
        todo!("不太好办。")
    }
    // 重载。
    pub fn recall(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "recall", &[])
            .unwrap();
    }
    pub fn recall_in(&self) {
        todo!("该函数是否应当实现？")
    }
}

// TODO: 需要知道 Java 或者 MessageChain 会不会返回除了以下消息之外的 SingleMessage
// TODO: 还有一些如 Audio 等消息没有实现，需要补上。
pub enum SingleMessage {
    At(At),
    AtAll(AtAll),
    Dice(Dice),
    Face(Face),
    FileMessage(FileMessage),
    ForwardMessage(ForwardMessage),
    Image(Image),
    LightApp(LightApp),
    MarketFace(MarketFace),
    MessageSource(MessageSource),
    MusicShare(MusicShare),
    PlainText(PlainText),
    PokeMessage(PokeMessage),
    QuoteReply(QuoteReply),
    RockPaperScissors(RockPaperScissors),
    UnsupportedMessage(UnsupportedMessage),
    VipFace(VipFace),
    // 以下这个应该不会被 MessageChain 返回吧？
}

impl GetEnvTrait for SingleMessage {
    fn get_instance(&self) -> Instance {
        match self {
            SingleMessage::At(a) => a.get_instance(),
            SingleMessage::AtAll(a) => a.get_instance(),
            SingleMessage::Dice(a) => a.get_instance(),
            SingleMessage::Face(a) => a.get_instance(),
            SingleMessage::FileMessage(a) => a.get_instance(),
            SingleMessage::ForwardMessage(a) => a.get_instance(),
            SingleMessage::Image(a) => a.get_instance(),
            SingleMessage::LightApp(a) => a.get_instance(),
            SingleMessage::MarketFace(a) => a.get_instance(),
            SingleMessage::MessageSource(a) => a.get_instance(),
            SingleMessage::MusicShare(a) => a.get_instance(),
            SingleMessage::PlainText(a) => a.get_instance(),
            SingleMessage::PokeMessage(a) => a.get_instance(),
            SingleMessage::QuoteReply(a) => a.get_instance(),
            SingleMessage::RockPaperScissors(a) => a.get_instance(),
            SingleMessage::UnsupportedMessage(a) => a.get_instance(),
            SingleMessage::VipFace(a) => a.get_instance(),
        }
    }
}

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
                    SingleMessage::At(At {
                        id: jvm
                            .to_rust(jvm.invoke(&instance, "getTarget", &[]).unwrap())
                            .unwrap(),
                        instance,
                    })
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.AtAll") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.AtAll")
                        .unwrap();
                    SingleMessage::AtAll(AtAll { instance })
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.Dice") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.Dice")
                        .unwrap();
                    SingleMessage::Dice(Dice { instance })
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.Face") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.Face")
                        .unwrap();
                    SingleMessage::Face(Face {
                        name: jvm
                            .to_rust(jvm.invoke(&instance, "getName", &[]).unwrap())
                            .unwrap(),
                        id: jvm
                            .to_rust(jvm.invoke(&instance, "getId", &[]).unwrap())
                            .unwrap(),
                        instance,
                    })
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.FileMessage") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.FileMessage")
                        .unwrap();
                    SingleMessage::FileMessage(FileMessage { instance })
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.ForwardMessage") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.ForwardMessage")
                        .unwrap();
                    SingleMessage::ForwardMessage(ForwardMessage { instance })
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.Image") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.Image")
                        .unwrap();
                    SingleMessage::Image(Image { instance })
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.LightApp") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.LightApp")
                        .unwrap();
                    SingleMessage::LightApp(LightApp { instance })
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.MessageSource") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.MessageSource")
                        .unwrap();
                    SingleMessage::MessageSource(MessageSource { instance })
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.MarketFace") {
                    if is_instance_of(&instance, "net.mamoe.mirai.message.data.RockPaperScissors") {
                        let instance = jvm
                            .cast(&instance, "net.mamoe.mirai.message.data.RockPaperScissors")
                            .unwrap();
                        SingleMessage::RockPaperScissors(RockPaperScissors { instance })
                    } else {
                        let instance = jvm
                            .cast(&instance, "net.mamoe.mirai.message.data.MarketFace")
                            .unwrap();
                        SingleMessage::MarketFace(MarketFace { instance })
                    }
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.MusicShare") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.MusicShare")
                        .unwrap();
                    SingleMessage::MusicShare(MusicShare { instance })
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.PlainText") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.PlainText")
                        .unwrap();
                    SingleMessage::PlainText(PlainText {
                        content: jvm
                            .to_rust(jvm.invoke(&instance, "getContent", &[]).unwrap())
                            .unwrap(),
                        instance,
                    })
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.PokeMessage") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.PokeMessage")
                        .unwrap();
                    SingleMessage::PokeMessage(PokeMessage::from_instance(instance))
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.QuoteReply") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.QuoteReply")
                        .unwrap();
                    SingleMessage::QuoteReply(QuoteReply { instance })
                } else if is_instance_of(&instance, "net.mamoe.mirai.message.data.VipFace") {
                    let instance = jvm
                        .cast(&instance, "net.mamoe.mirai.message.data.VipFace")
                        .unwrap();
                    SingleMessage::VipFace(VipFace { instance })
                } else {
                    SingleMessage::UnsupportedMessage(UnsupportedMessage { instance })
                }
            }
            Some(instance_to_single_message_enum(&jvm, next))
        } else {
            None
        }
    }
}

#[derive(GetInstanceDerive)]
pub struct At {
    id: i64,
    instance: Instance,
}

impl At {
    pub fn new(id: i64) -> At {
        let instance = Jvm::attach_thread()
            .unwrap()
            .create_instance(
                "net.mamoe.mirai.message.data.At",
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        At { id, instance }
    }
    pub fn to_display_string(&self, group: Group) -> String {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(
                        &self.get_instance(),
                        "getDisplay",
                        &[InvocationArg::try_from(group.get_instance()).unwrap()],
                    )
                    .unwrap(),
            )
            .unwrap()
    }
    pub fn get_target(&self) -> i64 {
        self.id
    }
}

impl MessageTrait for At {
    fn to_content(&self) -> String {
        format!("@{}", self.id)
    }
}

impl CodableMessageTrait for At {
    fn to_code(&self) -> String {
        format!("[mirai:at:{}]", self.id)
    }
}

impl SingleMessageTrait for At {}

impl MessageContentTrait for At {}

impl MessageHashCodeTrait for At {
    /// # 说明
    /// java.lang.Long 里的实现：
    /// ``` java
    /// public static int hashCode(long value) {
    ///     return (int)(value ^ (value >>> 32));
    /// }
    /// ```
    fn hash_code(&self) -> i32 {
        (self.id ^ (self.id/*i64*/ >> 32)) as i32
    }
}

#[derive(GetInstanceDerive)]
pub struct AtAll {
    instance: Instance,
}

impl AtAll {
    pub fn new() -> AtAll {
        let instance = Jvm::attach_thread()
            .unwrap()
            .static_class("net.mamoe.mirai.message.data.AtAll$INSTANCE")
            .unwrap();
        AtAll { instance }
    }
}

impl MessageTrait for AtAll {
    fn to_content(&self) -> String {
        format!("@全体成员")
    }
    fn to_string(&self) -> String {
        format!("[mirai:at all]")
    }
}

impl CodableMessageTrait for AtAll {
    fn to_code(&self) -> String {
        self.to_string()
    }
}

impl SingleMessageTrait for AtAll {}

impl MessageContentTrait for AtAll {}

impl MessageHashCodeTrait for AtAll {
    /// "@全体成员".hashCode()
    fn hash_code(&self) -> i32 {
        700264627
    }
}

#[derive(GetInstanceDerive)]
pub struct PlainText {
    content: String,
    instance: Instance,
}

impl From<&str> for PlainText {
    fn from(value: &str) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        PlainText {
            content: value.to_string(),
            instance: jvm
                .create_instance(
                    "net.mamoe.mirai.message.data.PlainText",
                    &[InvocationArg::try_from(value).unwrap()],
                )
                .unwrap(),
        }
    }
}

impl From<String> for PlainText {
    fn from(value: String) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                "net.mamoe.mirai.message.data.PlainText",
                &[InvocationArg::try_from(&value).unwrap()],
            )
            .unwrap();
        PlainText {
            content: value,
            instance,
        }
    }
}

impl PlainText {
    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}

impl MessageTrait for PlainText {
    fn to_content(&self) -> String {
        self.get_content()
    }
    fn to_string(&self) -> String {
        self.get_content()
    }
}

impl CodableMessageTrait for PlainText {
    fn to_code(&self) -> String {
        self.get_content()
    }
}

impl SingleMessageTrait for PlainText {}

impl MessageContentTrait for PlainText {}

impl MessageHashCodeTrait for PlainText {}

include!("face_res.rs");
#[derive(GetInstanceDerive)]
pub struct Face {
    name: String,
    id: i32,
    instance: Instance,
}

impl Face {
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl From<i32> for Face {
    fn from(id: i32) -> Self {
        let face = FaceEnum::from(id);
        Self::from(face)
    }
}

impl From<FaceEnum> for Face {
    fn from(face: FaceEnum) -> Self {
        let name = format!("[{:?}]", face);
        let id = face.into();
        let instance = Jvm::attach_thread()
            .unwrap()
            .create_instance(
                "net.mamoe.mirai.message.data.Face",
                &[InvocationArg::try_from(id).unwrap()],
            )
            .unwrap();
        Face { name, id, instance }
    }
}
// pub trait SetFace<T> {
//     fn set(&mut self, face: T);
// }
// impl SetFace<i32> for Face {
//     fn set(&mut self, face: i32) {
//         self.id = face;
//         self.instance = Jvm::attach_thread()
//             .unwrap()
//             .create_instance("net.mamoe.mirai.message.data.Face", &[InvocationArg::try_from(self.id).unwrap()])
//             .unwrap();
//         self.name = Jvm::attach_thread()
//             .unwrap()
//             .to_rust(
//                 Jvm::attach_thread()
//                     .unwrap()
//                     .invoke(&self.instance, "getName", &[])
//                     .unwrap(),
//             )
//             .unwrap()
//     }
// }
// impl SetFace<FaceEnum> for Face {
//     fn set(&mut self, face: FaceEnum) {
//         self.name = format!("[{:?}]", face);
//         self.id = face.into();
//         self.instance = Jvm::attach_thread()
//             .unwrap()
//             .create_instance("class_name", &[InvocationArg::try_from(self.id).unwrap()])
//             .unwrap();
//     }
// }

impl MessageTrait for Face {
    fn to_content(&self) -> String {
        self.name.clone()
    }
    fn to_string(&self) -> String {
        self.to_content()
    }
}

impl CodableMessageTrait for Face {
    fn to_code(&self) -> String {
        format!("[mirai:face:{}]", self.id)
    }
}

impl SingleMessageTrait for Face {}

impl MessageContentTrait for Face {}

impl MessageHashCodeTrait for Face {}

#[derive(Debug)]
pub enum ImageType {
    PNG,
    BMP,
    JPG,
    GIF,
    APNG,
    UNKNOW,
}

impl ImageType {
    pub fn r#match(format_name: String) -> Option<ImageType> {
        let binding = format_name.to_uppercase();
        let format_name = binding.as_str();
        Some(match format_name {
            "PNG" => PNG,
            "BMP" => BMP,
            "JPG" => JPG,
            "GIF" => GIF,
            "APNG" => APNG,
            "UNKNOW" => UNKNOW,
            _ => UNKNOW,
        })
    }
    pub fn get_format_name(&self) -> String {
        match self {
            APNG => String::from("png"),
            UNKNOW => String::from("gif"),
            _ => format!("{:?}", self).to_lowercase(),
        }
    }
}

#[derive(GetInstanceDerive)]
pub struct Image {
    pub(crate) instance: Instance,
}

impl Image {
    pub fn from_id(image_id: String) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static(
                "net.mamoe.mirai.message.data.Image",
                "fromId",
                &[InvocationArg::try_from(image_id).unwrap()],
            )
            .unwrap();
        Self { instance }
    }
    pub fn get_height(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.instance)
            .unwrap()
            .invoke("getHeight", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    // pub fn from_file(path: PathBuf) -> Self {
    //     let instance = Jvm::attach_thread()
    //         .unwrap()
    //         .invoke_static(
    //             "net.mamoe.mirai.utils.ExternalResource",
    //             "create",
    //             &[InvocationArg::try_from(
    //                 Jvm::attach_thread()
    //                     .unwrap()
    //                     .create_instance(
    //                         "java.io.File",
    //                         &[InvocationArg::try_from(path.to_str().unwrap()).unwrap()],
    //                     )
    //                     .unwrap(),
    //             )
    //             .unwrap()],
    //         )
    //         .unwrap();
    //     Image { instance }
    // }
    pub fn get_image_id(&self) -> String {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "getImageId", &[])
                    .unwrap(),
            )
            .unwrap()
    }
    pub fn get_image_id_regex() {
        todo!()
    }
    pub fn get_md5(&self) -> [i8; 16] {
        let jvm = Jvm::attach_thread().unwrap();
        crate::utils::internal::get_bytes_md5_and_cast_to_i8_16(jvm, &self.instance)
    }
    pub fn get_size(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.instance)
            .unwrap()
            .invoke("getSize", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    pub fn get_width(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.instance)
            .unwrap()
            .invoke("getWidth", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    // TODO: 吗的什么玩意儿。又是不知道哪来的。
    pub fn get_storage() -> i64 {
        todo!("低优先级。")
    }
    pub fn get_image_type(&self) -> ImageType {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getImageType", &[]).unwrap();
        let r#type = jvm.to_rust::<String>(instance).unwrap();
        match r#type.as_str() {
            "PNG" => PNG,
            "BMP" => BMP,
            "JPG" => JPG,
            "GIF" => GIF,
            "APNG" => APNG,
            _ => UNKNOW,
        }
    }
    pub fn is_emoji(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.instance)
            .unwrap()
            .invoke("isEmoji", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    pub fn is_uploaded(&self, bot: Bot, md5: [i8; 16], size: i64) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let bot = InvocationArg::try_from(bot.get_instance()).unwrap();
        let md5 = {
            let mut tmp = Vec::new();
            for item in md5 {
                tmp.push(
                    InvocationArg::try_from(item)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                );
            }
            tmp
        };
        let md5 = jvm.create_java_array("byte", &md5).unwrap();
        let md5 = InvocationArg::try_from(md5).unwrap();
        let size = InvocationArg::try_from(size)
            .unwrap()
            .into_primitive()
            .unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "isUpload", &[bot, md5, size])
                .unwrap(),
        )
            .unwrap()
    }
    /// TODO: 此函数为重载，还未实现。
    pub fn todo_is_uploaded() -> () {}
    pub fn query_url(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static(
                "net.mamoe.mirai.message.data.Image",
                "queryUrl",
                &[InvocationArg::try_from(self.get_instance()).unwrap()],
            )
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
}

impl MessageTrait for Image {}

impl CodableMessageTrait for Image {}

impl SingleMessageTrait for Image {}

impl MessageContentTrait for Image {}

impl MessageHashCodeTrait for Image {}

#[derive(GetInstanceDerive)]
pub struct UnsupportedMessage {
    instance: Instance,
}

impl MessageTrait for UnsupportedMessage {}

impl SingleMessageTrait for UnsupportedMessage {}

impl MessageContentTrait for UnsupportedMessage {}

#[derive(GetInstanceDerive)]
pub struct FileMessage {
    pub(crate) instance: Instance,
}

impl FileMessage {
    pub fn get_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getName", &[]).unwrap())
            .unwrap()
    }
    pub fn get_size(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getSize", &[]).unwrap())
            .unwrap()
    }
    pub fn get_file_id(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getId", &[]).unwrap())
            .unwrap()
    }
    pub fn get_internal_id(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getInternalId", &[]).unwrap())
            .unwrap()
    }
    pub fn new(file_id: String, internal_id: i32, name: String, size: i64) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let file_id = InvocationArg::try_from(&file_id).unwrap();
        let internal_id = InvocationArg::try_from(internal_id)
            .unwrap()
            .into_primitive()
            .unwrap();
        let name = InvocationArg::try_from(name).unwrap();
        let size = InvocationArg::try_from(size)
            .unwrap()
            .into_primitive()
            .unwrap();
        let instance = jvm
            .invoke_static(
                "net.mamoe.mirai.message.data.FileMessage",
                "create",
                &[file_id, internal_id, name, size],
            )
            .unwrap();
        FileMessage { instance }
    }
    pub fn to_absolute_file<FileSupported: FileSupportedTrait>(
        &self,
        contact: FileSupported,
    ) -> AbsoluteFile {
        let jvm = Jvm::attach_thread().unwrap();
        // let instance = InvocationArg::try_from(self.get_instance()).unwrap();
        let contact = InvocationArg::try_from(
            jvm.cast(
                &contact.get_instance(),
                "net.mamoe.mirai.contact.FileSupported",
            )
                .unwrap(),
        )
            .unwrap();
        let instance = jvm
            .invoke(&self.instance, "toAbsoluteFile", &[contact])
            .unwrap();
        // let instance = jvm
        //     .invoke_static(
        //         "rt.lea.LumiaUtils",
        //         "callToAbsoluteFile",
        //         &[instance, contact],
        //     )
        //     .unwrap();
        AbsoluteFile { instance }
    }
}

impl MessageTrait for FileMessage {}

impl SingleMessageTrait for FileMessage {}

impl MessageContentTrait for FileMessage {}

impl ConstrainSingleTrait for FileMessage {}

impl CodableMessageTrait for FileMessage {}

#[derive(GetInstanceDerive)]
pub struct MusicShare {
    instance: Instance,
}

impl MessageTrait for MusicShare {}

impl SingleMessageTrait for MusicShare {}

impl MessageContentTrait for MusicShare {}

impl ConstrainSingleTrait for MusicShare {}

impl CodableMessageTrait for MusicShare {}

#[derive(GetInstanceDerive)]
pub struct LightApp {
    instance: Instance,
}

impl MessageTrait for LightApp {}

impl SingleMessageTrait for LightApp {}

impl MessageContentTrait for LightApp {}

impl RichMessageTrait for LightApp {}

impl CodableMessageTrait for LightApp {}

#[derive(GetInstanceDerive)]
pub struct ForwardMessageBuilder {
    instance: Instance,
}

pub trait ForwaedMessageBuilderAddByUserAndMessageTrait: Sized {
    fn add(self, user_or_bot: impl UserOrBotTrait, message: impl MessageTrait, time: i32);
}

impl ForwaedMessageBuilderAddByUserAndMessageTrait for ForwardMessageBuilder {
    fn add(self, user_or_bot: impl UserOrBotTrait, message: impl MessageTrait, time: i32) {
        self.add__user_or_bot__message(user_or_bot, message, time);
    }
}

pub trait ForwaedMessageBuilderAddByIdNameAndMessageTrait: Sized {
    fn add(self, sender_id: i64, sender_name: &str, message: impl MessageTrait, time: i32);
}

impl ForwaedMessageBuilderAddByIdNameAndMessageTrait for ForwardMessageBuilder {
    fn add(self, sender_id: i64, sender_name: &str, message: impl MessageTrait, time: i32) {
        self.add__sender_id__sender_name__message(sender_id, sender_name, message, time);
    }
}

impl ForwardMessageBuilder {
    pub fn new(contact: impl ContactTrait) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let contact = contact.get_instance();
        let contact = InvocationArg::try_from(contact).unwrap();
        let instance = jvm
            .create_instance(
                "net.mamoe.mirai.message.data.ForwardMessageBuilder",
                &[contact],
            )
            .unwrap();
        Self { instance }
    }
    fn add__user_or_bot__message(self, user_or_bot: impl UserOrBotTrait, message: impl MessageTrait, time: i32) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let user_or_bot = InvocationArg::try_from(user_or_bot.get_instance()).unwrap();
        let message = InvocationArg::try_from(message.get_instance()).unwrap();
        let time = InvocationArg::try_from(time).unwrap().into_primitive().unwrap();
        let _ = jvm.invoke(&self.instance, "add", &[user_or_bot, message, time]).unwrap();
        self
    }
    fn add__sender_id__sender_name__message(self, sender_id: i64, sender_name: &str, message: impl MessageTrait, time: i32) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let sender_id = InvocationArg::try_from(sender_id).unwrap().into_primitive().unwrap();
        let sender_name = InvocationArg::try_from(sender_name).unwrap();
        let message = InvocationArg::try_from(message.get_instance()).unwrap();
        let time = InvocationArg::try_from(time).unwrap().into_primitive().unwrap();
        let _ = jvm.invoke(&self.instance, "add", &[sender_id, sender_name, message, time]).unwrap();
        self
    }
    pub fn set_display_strategy(self, title: String, brief: String, source: String, preview: Vec<String>, summary: String) -> Self { todo!() }
}

// TODO: RawForwardMessage is necessary for set_display_strategy.
// TODO: to_forward_message for message and chain, etc.
#[derive(GetInstanceDerive)]
pub struct ForwardMessage {
    instance: Instance,
}

#[derive(GetInstanceDerive)]
pub struct ForwardMessageNode {
    instance: Instance,
}

impl ForwardMessageNode {
    pub fn get_sender_id(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getSenderId", &[]).unwrap())
            .unwrap()
    }
    pub fn get_time(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getTime", &[]).unwrap())
            .unwrap()
    }
    pub fn get_sender_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getSenderName", &[]).unwrap())
            .unwrap()
    }
    pub fn get_message_chain(&self) -> MessageChain {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getMessageChain", &[]).unwrap();
        MessageChain { instance }
    }

    pub fn to_string(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "toString", &[]).unwrap())
            .unwrap()
    }
}

impl MessageHashCodeTrait for ForwardMessageNode {}

impl ForwardMessage {
    pub fn get_brief(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let brief = jvm.invoke(&self.instance, "getBrief", &[]).unwrap();
        jvm.to_rust(brief).unwrap()
    }
    pub fn get_node_vector(&self) -> Vec<ForwardMessageNode> {
        let jvm = Jvm::attach_thread().unwrap();
        let mut node_vector = Vec::new();
        let list = jvm.invoke(&self.instance, "getNodeList", &[]).unwrap();
        while {
            let has_next = jvm.invoke(&list, "hasNext", &[]).unwrap();
            jvm.to_rust(has_next).unwrap()
        } {
            let next = jvm.invoke(&list, "next", &[]).unwrap();
            node_vector.push(ForwardMessageNode { instance: next })
        }
        node_vector
    }
    pub fn get_preview(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let preview = jvm.invoke(&self.instance, "getPreview", &[]).unwrap();
        jvm.to_rust(preview).unwrap()
    }
    pub fn equals() {
        todo!()
    }
    pub fn get_source(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getSource", &[]).unwrap())
            .unwrap()
    }
    pub fn get_summary(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getSummary", &[]).unwrap())
            .unwrap()
    }
    pub fn get_title(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getTitle", &[]).unwrap())
            .unwrap()
    }
}

impl MessageTrait for ForwardMessage {}

impl SingleMessageTrait for ForwardMessage {}

impl MessageContentTrait for ForwardMessage {}

impl ConstrainSingleTrait for ForwardMessage {}

impl MessageHashCodeTrait for ForwardMessage {}

#[derive(GetInstanceDerive)]
pub struct Dice {
    instance: Instance,
}

impl Dice {
    /// 竟然可以直接指定值，太离谱了。。。
    /// 不知道新版 QQ 这个表情还能用不能。需要测试。
    /// TODO: 测试。
    pub fn new(mut value: u8) -> Self {
        if value > 6 {
            value = 1;
        }
        let value = value as i32;
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                "net.mamoe.mirai.message.data.Dice",
                &[InvocationArg::try_from(value)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        Self { instance }
    }
    pub fn equals() {
        todo!()
    }
    pub fn get_value(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.instance)
            .unwrap()
            .invoke("getValue", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    pub fn random() -> Self {
        use rand::prelude::*;
        let value = thread_rng().gen_range(1..=6);
        Self::new(value)
    }
}

impl MessageTrait for Dice {}

impl SingleMessageTrait for Dice {}

impl MessageContentTrait for Dice {}

impl ConstrainSingleTrait for Dice {}

impl CodableMessageTrait for Dice {}

impl MessageHashCodeTrait for Dice {}

impl MarketFaceTrait for Dice {}

#[derive(GetInstanceDerive)]
pub struct RockPaperScissors {
    instance: Instance,
}

impl RockPaperScissors {
    fn new(field: &str) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .static_class_field("net.mamoe.mirai.message.data.RockPaperScissors", field)
            .unwrap();
        Self { instance }
    }
    pub fn rock() -> Self {
        Self::new("ROCK")
    }
    pub fn scissors() -> Self {
        Self::new("SCISSORS")
    }
    pub fn paper() -> Self {
        Self::new("PAPER")
    }
    pub fn equals() {
        todo!()
    }
    pub fn eliminates(&self, other: RockPaperScissors) -> Option<bool> {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm
            .invoke(
                &self.instance,
                "eliminates",
                &[InvocationArg::try_from(other.get_instance()).unwrap()],
            )
            .unwrap();
        if Env::instance_is_null(&result) {
            None
        } else {
            Some(jvm.to_rust(result).unwrap())
        }
    }
    pub fn random() -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static(
                "net.mamoe.mirai.message.data.RockPaperScissors",
                "random",
                &[],
            )
            .unwrap();
        Self { instance }
    }
}

impl MessageTrait for RockPaperScissors {}

impl SingleMessageTrait for RockPaperScissors {}

impl MessageContentTrait for RockPaperScissors {}

impl ConstrainSingleTrait for RockPaperScissors {}

impl CodableMessageTrait for RockPaperScissors {}

impl MessageHashCodeTrait for RockPaperScissors {}

impl MarketFaceTrait for RockPaperScissors {}

// #[derive(GetEnvDerive)]
// pub struct  {
//     instance: Instance,
// }
// impl MessageTrait for  {}
// impl SingleMessageTrait for  {}
// impl MessageContentTrait for  {}
// impl ConstrainSingleTrait for  {}
// impl CodableMessageTrait for  {}

// TODO
#[derive(GetInstanceDerive)]
pub struct VipFace {
    instance: Instance,
}

impl MessageTrait for VipFace {}

impl SingleMessageTrait for VipFace {}

impl MessageContentTrait for VipFace {}

impl ConstrainSingleTrait for VipFace {}

impl CodableMessageTrait for VipFace {}

pub enum PokeMessage {
    戳一戳,
    比心,
    点赞,
    心碎,
    六六六,
    放大招,
    宝贝球,
    玫瑰花,
    召唤术,
    让你皮,
    结印,
    手雷,
    勾引,
    抓一下,
    碎屏,
    敲门,
}

impl FromInstance for PokeMessage {
    fn from_instance(instance: Instance) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let t: (i32, i32) = (
            jvm.to_rust(jvm.invoke(&instance, "getPokeType", &[]).unwrap())
                .unwrap(),
            jvm.to_rust(jvm.invoke(&instance, "getId", &[]).unwrap())
                .unwrap(),
        );
        match t {
            (a, -1) => match a {
                1 => PokeMessage::戳一戳,
                2 => PokeMessage::比心,
                3 => PokeMessage::点赞,
                4 => PokeMessage::心碎,
                5 => PokeMessage::六六六,
                6 => PokeMessage::放大招,
                _ => unsafe { unreachable_unchecked() },
            },
            (126, b) => match b {
                2011 => PokeMessage::宝贝球,
                2009 => PokeMessage::让你皮,
                2007 => PokeMessage::玫瑰花,
                2006 => PokeMessage::召唤术,
                2005 => PokeMessage::结印,
                2004 => PokeMessage::手雷,
                2003 => PokeMessage::勾引,
                2002 => PokeMessage::碎屏,
                2001 => PokeMessage::抓一下,
                2000 => PokeMessage::敲门,
                _ => unsafe { unreachable_unchecked() },
            },
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl PokeMessage {
    fn name__poke_type__id(&self) -> (&str, i32, i32) {
        // match self {
        //     PokeMessage::戳一戳 => ("戳一戳", 1, -1),
        //     PokeMessage::比心 => ("比心", 2, -1),
        //     PokeMessage::点赞 => ("点赞", 3, -1),
        //     PokeMessage::心碎 => ("心碎", 4, -1),
        //     PokeMessage::六六六 => ("666", 5, -1),
        //     PokeMessage::放大招 => ("放大招", 6, -1),
        //     PokeMessage::宝贝球 => ("宝贝球", 126, 2011),
        //     PokeMessage::让你皮 => ("让你皮", 126, 2009),
        //     PokeMessage::玫瑰花 => ("玫瑰花", 126, 2007),
        //     PokeMessage::召唤术 => ("召唤术", 126, 2006),
        //     PokeMessage::结印 => ("结印", 126, 2005),
        //     PokeMessage::手雷 => ("手雷", 126, 2004),
        //     PokeMessage::勾引 => ("勾引", 126, 2003),
        //     PokeMessage::碎屏 => ("碎屏", 126, 2002),
        //     PokeMessage::抓一下 => ("抓一下", 126, 2001),
        //     PokeMessage::敲门 => ("敲门", 126, 2000),
        // }
        (self.get_name(), self.get_poke_type(), self.get_id())
    }
    pub fn get_name(&self) -> &str {
        match self {
            PokeMessage::戳一戳 => "戳一戳",
            PokeMessage::比心 => "比心",
            PokeMessage::点赞 => "点赞",
            PokeMessage::心碎 => "心碎",
            PokeMessage::六六六 => "666",
            PokeMessage::放大招 => "放大招",
            PokeMessage::宝贝球 => "宝贝球",
            PokeMessage::让你皮 => "让你皮",
            PokeMessage::玫瑰花 => "玫瑰花",
            PokeMessage::召唤术 => "召唤术",
            PokeMessage::结印 => "结印",
            PokeMessage::手雷 => "手雷",
            PokeMessage::勾引 => "勾引",
            PokeMessage::碎屏 => "碎屏",
            PokeMessage::抓一下 => "抓一下",
            PokeMessage::敲门 => "敲门",
        }
    }
    pub fn get_poke_type(&self) -> i32 {
        match self {
            PokeMessage::戳一戳 => 1,
            PokeMessage::比心 => 2,
            PokeMessage::点赞 => 3,
            PokeMessage::心碎 => 4,
            PokeMessage::六六六 => 5,
            PokeMessage::放大招 => 6,
            PokeMessage::宝贝球
            | PokeMessage::让你皮
            | PokeMessage::玫瑰花
            | PokeMessage::召唤术
            | PokeMessage::结印
            | PokeMessage::手雷
            | PokeMessage::勾引
            | PokeMessage::碎屏
            | PokeMessage::抓一下
            | PokeMessage::敲门 => 126,
        }
    }
    pub fn get_id(&self) -> i32 {
        match self {
            PokeMessage::戳一戳
            | PokeMessage::比心
            | PokeMessage::点赞
            | PokeMessage::心碎
            | PokeMessage::六六六
            | PokeMessage::放大招 => -1,
            PokeMessage::宝贝球 => 2011,
            PokeMessage::让你皮 => 2009,
            PokeMessage::玫瑰花 => 2007,
            PokeMessage::召唤术 => 2006,
            PokeMessage::结印 => 2005,
            PokeMessage::手雷 => 2004,
            PokeMessage::勾引 => 2003,
            PokeMessage::碎屏 => 2002,
            PokeMessage::抓一下 => 2001,
            PokeMessage::敲门 => 2000,
        }
    }
}

impl GetEnvTrait for PokeMessage {
    fn get_instance(&self) -> Instance {
        let jvm = Jvm::attach_thread().unwrap();
        let (name, poke_type, id) = self.name__poke_type__id();
        let (name, poke_type, id) = (
            InvocationArg::try_from(name).unwrap(),
            InvocationArg::try_from(poke_type).unwrap(),
            InvocationArg::try_from(id).unwrap(),
        );
        jvm.create_instance(
            "net.mamoe.mirai.message.data.PokeMessage",
            &[name, poke_type, id],
        )
            .unwrap()
    }
}

impl MessageTrait for PokeMessage {
    fn to_content(&self) -> String {
        String::from("[戳一戳]")
    }

    fn to_string(&self) -> String {
        let mut str = String::from("[mirai:poke");
        str.push_str(self.get_name());
        str.push(',');
        str.push_str(self.get_poke_type().to_string().as_str());
        str.push(',');
        str.push_str(self.get_id().to_string().as_str());
        str.push(']');
        str
    }
}

impl SingleMessageTrait for PokeMessage {}

impl MessageContentTrait for PokeMessage {}

impl ConstrainSingleTrait for PokeMessage {}

impl CodableMessageTrait for PokeMessage {}

impl MessageHashCodeTrait for PokeMessage {}

// TODO
#[derive(GetInstanceDerive)]
pub struct MessageSource {
    instance: Instance,
}

impl MessageTrait for MessageSource {}

impl SingleMessageTrait for MessageSource {}

impl ConstrainSingleTrait for MessageSource {}

#[derive(GetInstanceDerive)]
pub struct MarketFace {
    pub(crate) instance: Instance,
}

impl MessageTrait for MarketFace {}

impl SingleMessageTrait for MarketFace {}

impl ConstrainSingleTrait for MarketFace {}

impl MessageContentTrait for MarketFace {}

impl MarketFaceTrait for MarketFace {}
