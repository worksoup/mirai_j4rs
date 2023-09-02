use std::path::PathBuf;

use super::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MarketFace, MessageChainTrait, MessageContentTrait,
    MessageTrait, RichMessageTrait, SingleMessageTrait,
};
use crate::{contact::contact_trait::ContactTrait, env::GetEnvTrait};
use j4rs::{Instance, InvocationArg, Jvm};
use num_enum::IntoPrimitive;

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
    pub fn get_source(&self) {
        todo!()
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
    pub fn quote(&self) {
        todo!()
    }
    pub fn quote_reply(&self) {
        todo!()
    } // 两个重载。
    pub fn _quote_reply() {
        todo!()
    } // 重载。
    pub fn recall(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "recall", &[])
            .unwrap();
    } //两个重载
    fn _recall() {
        todo!()
    } // 重载。
}
pub struct MessageChain {
    pub(crate) instance: Instance,
}
impl MessageChain {}
impl GetEnvTrait for MessageChain {
    fn get_instance(&self) -> j4rs::Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}
impl MessageTrait for MessageChain {}
impl CodableMessageTrait for MessageChain {}
impl MessageChainTrait for MessageChain {}

pub struct At {
    id: i64,
    instance: Instance,
}
impl GetEnvTrait for At {
    fn get_instance(&self) -> j4rs::Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
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
    /// # 说明
    /// java.lang.Long 里的实现：
    /// ``` java
    /// public static int hashCode(long value) {
    ///     return (int)(value ^ (value >>> 32));
    /// }
    /// ```
    pub fn hash_code(&self) -> i32 {
        (self.id ^ (self.id/*i64*/ >> 32)) as i32
    }
}

impl MessageTrait for At {
    fn to_content_text(&self) -> String {
        format!("@{}", self.id)
    }
}
impl CodableMessageTrait for At {
    fn to_mirai_code(&self) -> String {
        format!("[mirai:at:{}]", self.id)
    }
}
impl SingleMessageTrait for At {}
impl MessageContentTrait for At {}
pub struct AtAll {
    instance: Instance,
}
impl GetEnvTrait for AtAll {
    fn get_instance(&self) -> j4rs::Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}
impl AtAll {
    pub fn new() -> AtAll {
        let instance = Jvm::attach_thread()
            .unwrap()
            .static_class("net.mamoe.mirai.message.data.AtAll$INSTANCE")
            .unwrap();
        AtAll { instance }
    }
    pub fn hash_code(&self) -> i32 {
        todo!()
    }
}
impl MessageTrait for AtAll {
    fn to_content_text(&self) -> String {
        format!("@全体成员")
    }
    fn to_string(&self) -> String {
        format!("[mirai:at all]")
    }
}
impl CodableMessageTrait for AtAll {
    fn to_mirai_code(&self) -> String {
        self.to_string()
    }
}
impl SingleMessageTrait for AtAll {}
impl MessageContentTrait for AtAll {}
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
    pub fn hash_code(&self) -> i32 {
        todo!()
    }
    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}
impl GetEnvTrait for PlainText {
    fn get_instance(&self) -> j4rs::Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}
impl MessageTrait for PlainText {
    fn to_content_text(&self) -> String {
        self.get_content()
    }
    fn to_string(&self) -> String {
        self.get_content()
    }
}
impl CodableMessageTrait for PlainText {
    fn to_mirai_code(&self) -> String {
        self.get_content()
    }
}
impl SingleMessageTrait for PlainText {}
impl MessageContentTrait for PlainText {}

include!("./face.rs");
pub struct Face {
    name: String,
    id: i32,
    instance: Instance,
}
impl Face {
    pub fn hash_code(&self) -> i32 {
        todo!()
    }
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
pub trait SetFace<T> {
    fn set(&mut self, face: T);
}
impl SetFace<i32> for Face {
    fn set(&mut self, face: i32) {
        self.id = face;
        self.instance = Jvm::attach_thread()
            .unwrap()
            .create_instance("class_name", &[InvocationArg::try_from(self.id).unwrap()])
            .unwrap();
        self.name = Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "getName", &[])
                    .unwrap(),
            )
            .unwrap()
    }
}
impl SetFace<FaceEnum> for Face {
    fn set(&mut self, face: FaceEnum) {
        self.name = format!("[{:?}]", face);
        self.id = face.into();
        self.instance = Jvm::attach_thread()
            .unwrap()
            .create_instance("class_name", &[InvocationArg::try_from(self.id).unwrap()])
            .unwrap();
    }
}
impl GetEnvTrait for Face {
    fn get_instance(&self) -> j4rs::Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}
impl MessageTrait for Face {
    fn to_content_text(&self) -> String {
        self.name.clone()
    }
    fn to_string(&self) -> String {
        self.to_content_text()
    }
}
impl CodableMessageTrait for Face {
    fn to_mirai_code(&self) -> String {
        format!("[mirai:face:{}]", self.id)
    }
}
impl SingleMessageTrait for Face {}
impl MessageContentTrait for Face {}
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
            "PNG" => ImageType::PNG,
            "BMP" => ImageType::BMP,
            "JPG" => ImageType::JPG,
            "GIF" => ImageType::GIF,
            "APNG" => ImageType::APNG,
            "UNKNOW" => ImageType::UNKNOW,
            _ => ImageType::UNKNOW,
        })
    }
    pub fn get_format_name(&self) -> String {
        match self {
            ImageType::APNG => String::from("png"),
            ImageType::UNKNOW => String::from("gif"),
            _ => format!("{:?}", self).to_lowercase(),
        }
    }
}
pub struct Image {
    pub(crate) instance: Instance,
}
impl Image {
    pub fn from_file(path: PathBuf) -> Self {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke_static(
                "net.mamoe.mirai.utils.ExternalResource",
                "create",
                &[InvocationArg::try_from(
                    Jvm::attach_thread()
                        .unwrap()
                        .create_instance(
                            "java.io.File",
                            &[InvocationArg::try_from(path.to_str().unwrap()).unwrap()],
                        )
                        .unwrap(),
                )
                .unwrap()],
            )
            .unwrap();
        Image { instance }
    }
    pub fn hash_code(&self) -> i32 {
        todo!()
    }
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
    pub fn get_md5(&self) -> Vec<u8> {
        todo!()
    }
    pub fn from_id() {
        todo!()
    }
    pub fn get_size() -> (i32, i32) {
        todo!()
    }
    pub fn get_storage() -> i64 {
        todo!()
    }
    pub fn get_image_id_regex() {
        todo!()
    }
    pub fn get_image_type() -> ImageType {
        todo!()
    }
    pub fn is_emoji() -> bool {
        todo!()
    }
}
impl GetEnvTrait for Image {
    fn get_instance(&self) -> j4rs::Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}
impl MessageTrait for Image {}
impl CodableMessageTrait for Image {}
impl SingleMessageTrait for Image {}
impl MessageContentTrait for Image {}
pub struct UnsupportedMessage {}
impl GetEnvTrait for UnsupportedMessage {
    fn get_instance(&self) -> j4rs::Instance {
        todo!()
    }
}
impl MessageTrait for UnsupportedMessage {}
impl SingleMessageTrait for UnsupportedMessage {}
impl MessageContentTrait for UnsupportedMessage {}
pub struct FileMessage {
    instance: Instance,
}
impl GetEnvTrait for FileMessage {
    fn get_instance(&self) -> j4rs::Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}
impl MessageTrait for FileMessage {}
impl SingleMessageTrait for FileMessage {}
impl MessageContentTrait for FileMessage {}
impl ConstrainSingleTrait for FileMessage {}
impl CodableMessageTrait for FileMessage {}
pub struct MusicShare {
    instance: Instance,
}
impl GetEnvTrait for MusicShare {
    fn get_instance(&self) -> j4rs::Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}
impl MessageTrait for MusicShare {}
impl SingleMessageTrait for MusicShare {}
impl MessageContentTrait for MusicShare {}
impl ConstrainSingleTrait for MusicShare {}
impl CodableMessageTrait for MusicShare {}
pub struct LightApp {
    instance: Instance,
}
impl GetEnvTrait for LightApp {
    fn get_instance(&self) -> j4rs::Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}
impl MessageTrait for LightApp {}
impl SingleMessageTrait for LightApp {}
impl MessageContentTrait for LightApp {}
impl RichMessageTrait for LightApp {}
impl CodableMessageTrait for LightApp {}
pub struct ForwardMessage {
    instance: Instance,
}
impl GetEnvTrait for ForwardMessage {
    fn get_instance(&self) -> j4rs::Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}
impl MessageTrait for ForwardMessage {}
impl SingleMessageTrait for ForwardMessage {}
impl MessageContentTrait for ForwardMessage {}
impl ConstrainSingleTrait for ForwardMessage {}
pub struct Dice {
    instance: Instance,
}
impl GetEnvTrait for Dice {
    fn get_instance(&self) -> j4rs::Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}
impl MessageTrait for Dice {}
impl SingleMessageTrait for Dice {}
impl MessageContentTrait for Dice {}
impl ConstrainSingleTrait for Dice {}
impl CodableMessageTrait for Dice {}
impl MarketFace for Dice {}
// #[derive(GetEnvDerive)]
// pub struct  {
//     instance: Instance,
// }
// impl MessageTrait for  {}
// impl SingleMessageTrait for  {}
// impl MessageContentTrait for  {}
// impl ConstrainSingleTrait for  {}
// impl CodableMessageTrait for  {}
pub struct VipFace {
    instance: Instance,
}
impl GetEnvTrait for VipFace {
    fn get_instance(&self) -> j4rs::Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}
impl MessageTrait for VipFace {}
impl SingleMessageTrait for VipFace {}
impl MessageContentTrait for VipFace {}
impl ConstrainSingleTrait for VipFace {}
impl CodableMessageTrait for VipFace {}
pub struct PokeMessage {
    instance: Instance,
}
impl GetEnvTrait for PokeMessage {
    fn get_instance(&self) -> j4rs::Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}
impl MessageTrait for PokeMessage {}
impl SingleMessageTrait for PokeMessage {}
impl MessageContentTrait for PokeMessage {}
impl ConstrainSingleTrait for PokeMessage {}
impl CodableMessageTrait for PokeMessage {}

pub struct MessageSource {
    instance: Instance,
}
impl GetEnvTrait for MessageSource {
    fn get_instance(&self) -> j4rs::Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}
impl MessageTrait for MessageSource {}
impl SingleMessageTrait for MessageSource {}
impl ConstrainSingleTrait for MessageSource {}
