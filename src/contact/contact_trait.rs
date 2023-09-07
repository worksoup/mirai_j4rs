use std::path::PathBuf;

use j4rs::{InvocationArg, Jvm};

use crate::{
    env::GetBotTrait,
    env::GetEnvTrait,
    message::{message_trait::MessageTrait, Image, MessageReceipt},
    other::enums::AvatarSpec,
};

pub trait ContactOrBotTrait
    where
        Self: Sized + GetEnvTrait + GetBotTrait,
{
    fn get_id(&self) -> i64 {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.get_instance(), "getId", &[])
                    .unwrap(),
            )
            .unwrap()
    }
    //
    //应为：
    // ```rust
    // fn get_avatar_url(&self, size: Option<AvatarSpec>) -> String {
    //     let size = j4rs::InvocationArg::try_from(
    //     Jvm::attach_thread()
    //         .unwrap()
    //         .field(
    //              &Jvm::attach_thread()
    //                   .unwrap()
    //                   .static_class("net.mamoe.mirai.contact.AvatarSpec")
    //                   .unwrap(),
    //              match size.unwrap() {
    //                    AvatarSpec::XS => "SMALLEST",
    //                    AvatarSpec::S => "SMALL",
    //                    AvatarSpec::M => "MEDIUM",
    //                    AvatarSpec::L => "LARGE",
    //                    AvatarSpec::XL => "LARGEST",
    //                    AvatarSpec::ORIGINAL => "ORIGINAL",
    //              },
    //         )
    //         .unwrap(),
    //     )
    //     .unwrap();
    //     Jvm::attach_thread()
    //         .unwrap()
    //         .to_rust(
    //             Jvm::attach_thread()
    //             .unwrap()
    //             .invoke(&self.get_instance(), "getAvatarUrl", &[size])
    //             .unwrap(),
    //      )
    //      .unwrap()
    // }
    // ```
    //
    // 根据mirai源码，各个类型实现该trait时实际如此：
    // ```rust
    // fn get_avatar_url(&self, size: Option<AvatarSpec>) -> String {
    //     let size: i32 = if let Some(size) = size {
    //         size.into()
    //     } else {
    //         AvatarSpec::XL.into()
    //     };
    //     return format!(r"http://q.qlogo.cn/g?b=qq&nk={}&s={}", self.get_id(), size,);
    // }
    // ```
    fn get_avatar_url(&self, size: Option<AvatarSpec>) -> String {
        let size = j4rs::InvocationArg::try_from(
            Jvm::attach_thread()
                .unwrap()
                .field(
                    &Jvm::attach_thread()
                        .unwrap()
                        .static_class("net.mamoe.mirai.contact.AvatarSpec")
                        .unwrap(),
                    match size.unwrap() {
                        AvatarSpec::XS => "SMALLEST",
                        AvatarSpec::S => "SMALL",
                        AvatarSpec::M => "MEDIUM",
                        AvatarSpec::L => "LARGE",
                        AvatarSpec::XL => "LARGEST",
                        AvatarSpec::ORIGINAL => "ORIGINAL",
                    },
                )
                .unwrap(),
        )
            .unwrap();
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.get_instance(), "getAvatarUrl", &[size])
                    .unwrap(),
            )
            .unwrap()
    }
}

pub trait ContactTrait
    where
        Self: ContactOrBotTrait,
{
    fn send_message<'a>(&self, message: impl MessageTrait) -> MessageReceipt<Self> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.get_instance(),
                "sendMessage",
                &[j4rs::InvocationArg::try_from(message.get_instance()).unwrap()],
            )
            .unwrap();
        MessageReceipt::new(instance, &self)
    }
    // fn send_message_suspend<'a>(
    //     &'a self,
    //     _message: impl MessageTrait,
    // ) -> MessageReceipt<'a, Self> {
    //     // let instance = self
    //     //     .get_jvm()
    //     //     .invoke(
    //     //         &self.get_instance(),
    //     //         "sendMessage",
    //     //         &[j4rs::InvocationArg::try_from(message.get_instance()).unwrap()],
    //     //     )
    //     //     .unwrap();
    //     // MessageReceipt::new( Jvm::attach_thread().unwrap(), instance, &self)
    //
    // }

    ///
    /// 对应一个sendMessage的重载
    fn send_string(&self, string: &str) -> MessageReceipt<'_, Self> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.get_instance(),
                "sendMessage",
                &[j4rs::InvocationArg::try_from(string).unwrap()],
            )
            .unwrap();
        MessageReceipt::new(instance, &self)
    }
    // 感觉没什么用
    // fn send_image(&self, img: Image) -> MessageReceipt<Self> {
    //     let _ = Jvm::attach_thread()
    //         .unwrap()
    //         .invoke(
    //             &self.get_instance(),
    //             "uploadImage",
    //             &[InvocationArg::try_from(img.get_instance()).unwrap()],
    //         )
    //         .unwrap();
    //     let instance = Jvm::attach_thread()
    //         .unwrap()
    //         .invoke(
    //             &self.get_instance(),
    //             "sendMessage",
    //             &[InvocationArg::try_from(img.get_instance()).unwrap()],
    //         )
    //         .unwrap();
    //     MessageReceipt::new(instance, &self)
    // }
    fn upload_image_from_file(&self, path: &PathBuf) -> Image {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
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
        // 存疑：是否需要传入 Group(java) 本身？
        // 新：似乎不需要？
        // 新：前两条注释说的是什么来着？
        let image_instance = jvm
            .invoke(
                &self.get_instance(),
                "uploadImage",
                &[InvocationArg::try_from(jvm.clone_instance(&instance).unwrap()).unwrap()],
            )
            .unwrap();
        // Mirai 文档里说要 close.
        let _ = jvm.invoke(&instance, "close", &[]);
        Image {
            instance: image_instance,
        }
    }
}

pub trait FileSupportedTrait
    where
        Self: ContactTrait,
{}

pub trait AudioSupportedTrait
    where
        Self: ContactTrait,
{}

pub trait UserOrBotTrait
    where
        Self: ContactOrBotTrait,
{}

pub trait UserTrait
    where
        Self: UserOrBotTrait + ContactTrait,
{}

pub trait MemberTrait
    where
        Self: UserTrait,
{}

pub trait StrangerTrait
    where
        Self: UserTrait,
{}
