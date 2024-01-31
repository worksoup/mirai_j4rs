use crate::contact::bot::Bot;
use crate::message::message_trait::{
    CodableMessageTrait, MessageContentTrait, MessageHashCodeTrait, MessageTrait,
    SingleMessageTrait,
};
use crate::message::IMAGE_ID_REGEX;
use j4rs::{Instance, InvocationArg, Jvm};
use mjbase::env::GetEnvTrait;
use mjmacro::{FromInstanceDerive, GetInstanceDerive};
use regex::Regex;

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
            ImageType::PNG => format!("{:?}", self).to_lowercase(),
            ImageType::BMP => format!("{:?}", self).to_lowercase(),
            ImageType::JPG => format!("{:?}", self).to_lowercase(),
            ImageType::GIF => String::from("gif"),
            ImageType::APNG => String::from("png"),
            ImageType::UNKNOW => format!("{:?}", self).to_lowercase(),
        }
    }
}

#[derive(GetInstanceDerive, FromInstanceDerive)]
pub struct Image {
    instance: Instance,
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
    /// 模板：`\{[0-9a-fA-F]{8}-([0-9a-fA-F]{4}-){3}[0-9a-fA-F]{12}\}\..{3,5}`
    /// 示例：`{01E9451B-70ED-EAE3-B37C-101F1EEBF5B5}.ext`
    /// 可以直接使用 [IMAGE_ID_REGEX] 静态对象。
    pub fn get_image_id_regex() -> Regex {
        return IMAGE_ID_REGEX.clone();
    }
    pub fn get_md5(&self) -> [i8; 16] {
        let jvm = Jvm::attach_thread().unwrap();
        mjbase::utils::get_bytes_md5_and_cast_to_i8_16(jvm, &self.instance)
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
    pub fn get_image_type(&self) -> ImageType {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getImageType", &[]).unwrap();
        let r#type = jvm.to_rust::<String>(instance).unwrap();
        match r#type.as_str() {
            "PNG" => ImageType::PNG,
            "BMP" => ImageType::BMP,
            "JPG" => ImageType::JPG,
            "GIF" => ImageType::GIF,
            "APNG" => ImageType::APNG,
            _ => ImageType::UNKNOW,
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
