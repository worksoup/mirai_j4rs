use crate::{
    contact::{
        group::{Group, MemberPermission},
        Friend,
    },
    env::{FromInstance, GetEnvTrait},
    message::{message_trait::MessageTrait, Image, MessageReceipt},
    utils::other::enums::AvatarSpec,
};
use j4rs::{InvocationArg, Jvm};
use std::path::PathBuf;
use crate::action::nudges::Nudge;
use crate::contact::group::MemberActive;

pub trait AssertMemberPermissionTrait: MemberTrait {
    fn is_owner(&self) -> bool;
    fn is_administrator(&self) -> bool;
    fn is_operator(&self) -> bool;
}

pub trait ContactOrBotTrait
    where
        Self: Sized + GetEnvTrait + FromInstance,
{
    fn get_bot(&self) -> crate::contact::bot::Bot {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&GetEnvTrait::get_instance(self), "getBot", &[])
            .unwrap();
        crate::contact::bot::Bot::from_instance(instance)
    }

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
{}

pub trait SendMessageSupportedTrait: ContactTrait {
    fn send_message(&self, message: impl MessageTrait) -> MessageReceipt<Self> {
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

pub trait NudgeSupportedTrait: UserOrBotTrait {
    type NudgeType: Nudge;
    fn nudge(&self) -> Self::NudgeType {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.get_instance(), "nudge", &[]).unwrap();
        Self::NudgeType::from_instance(instance)
    }
}

pub trait UserTrait
    where
        Self: UserOrBotTrait + ContactTrait,
{}

pub trait MemberTrait
    where
        Self: UserTrait,
{
    fn get_group(&self) -> Group {
        let jvm = Jvm::attach_thread().unwrap();
        let group = jvm.invoke(&self.get_instance(), "getGroup", &[]).unwrap();
        Group::from_instance(group)
    }
    fn get_active(&self) -> MemberActive { todo!() }
    fn get_name_card(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let name_card = jvm
            .invoke(&self.get_instance(), "getNameCard", &[])
            .unwrap();
        jvm.to_rust(name_card).unwrap()
    }
    fn get_permission(&self) -> MemberPermission {
        let jvm = Jvm::attach_thread().unwrap();
        let perm = jvm
            .invoke(&self.get_instance(), "getPermission", &[])
            .unwrap();
        let perm = jvm.invoke(&perm, "getLevel", &[]).unwrap();
        let perm: i32 = jvm.to_rust(perm).unwrap();
        MemberPermission::from(perm)
    }
    fn get_rank_title(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.get_instance(), "getRankTitle", &[])
                .unwrap(),
        )
            .unwrap()
    }
    fn get_special_title(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.get_instance(), "getSpecialTitle", &[])
                .unwrap(),
        )
            .unwrap()
    }
    fn get_temperature_title(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.get_instance(), "getTemperatureTitle", &[])
                .unwrap(),
        )
            .unwrap()
    }
    // TODO: 会抛出错误。
    fn mute(&self, duration_seconds: i64) {
        let jvm = Jvm::attach_thread().unwrap();
        let seconds = InvocationArg::try_from(duration_seconds)
            .unwrap()
            .into_primitive()
            .unwrap();
        jvm.invoke(&self.get_instance(), "mute", &[seconds])
            .unwrap();
    }
}

// TODO: 为 `Bot`, `Stranger`, `NormalMember`, 实现。
// 为什么 Mirai 里实现得这么怪啊。
pub trait AsFriend {
    fn as_friend(&self) -> Friend;
}

// TODO: 为 `Bot`, `NormalMember`, 实现。
// 为什么 Mirai 里实现得这么怪啊。
pub trait AsStranger {
    fn as_stranger(&self) -> Friend;
}
