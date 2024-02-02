use crate::file::RemoteFiles;
use crate::{
    contact::group::{
        AnnouncementParameters, MemberActive, OfflineAnnouncement, OnlineAnnouncement,
    },
    contact::{
        group::{Group, MemberPermission},
        Friend,
    },
    error::MiraiRsError,
    message::{
        action::nudges::Nudge, data::image::Image, message_receipt::MessageReceipt,
        message_trait::MessageTrait,
    },
    utils::other::enums::AvatarSpec,
};
use j4rs::{InvocationArg, Jvm};
use mj_base::env::{FromInstance, GetInstanceTrait};

pub trait AssertMemberPermissionTrait: MemberTrait {
    fn is_owner(&self) -> bool;
    fn is_administrator(&self) -> bool;
    fn is_operator(&self) -> bool;
}

pub trait ContactOrBotTrait
where
    Self: Sized + GetInstanceTrait + FromInstance,
{
    fn get_bot(&self) -> crate::contact::bot::Bot {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&GetInstanceTrait::get_instance(self), "getBot", &[])
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
{
}

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
    fn upload_image_from_file(&self, path: &str) -> Image {
        let jvm = Jvm::attach_thread().unwrap();
        let resource = external_resource_from_file(&jvm, path);
        // 存疑：是否需要传入 Group(java) 本身？
        // 新：似乎不需要？
        // 新：前两条注释说的是什么来着？
        let image_instance = jvm
            .invoke(
                &self.get_instance(),
                "uploadImage",
                &[InvocationArg::try_from(jvm.clone_instance(&resource).unwrap()).unwrap()],
            )
            .unwrap();
        // Mirai 文档里说要 close.
        external_resource_close(&jvm, resource);
        Image::from_instance(image_instance)
    }
}

pub trait FileSupportedTrait
where
    Self: ContactTrait,
{
    fn get_files(&self) -> RemoteFiles {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .cast(
                &self.get_instance(),
                "net.mamoe.mirai.contact.FileSupported",
            )
            .unwrap();
        let instance = jvm.invoke(&instance, "getFiles", &[]).unwrap();
        RemoteFiles::from_instance(instance)
    }
}

pub trait AudioSupportedTrait
where
    Self: ContactTrait,
{
}

pub trait UserOrBotTrait
where
    Self: ContactOrBotTrait,
{
}

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
{
}

pub trait MemberTrait
where
    Self: UserTrait,
{
    fn get_group(&self) -> Group {
        let jvm = Jvm::attach_thread().unwrap();
        let group = jvm.invoke(&self.get_instance(), "getGroup", &[]).unwrap();
        Group::from_instance(group)
    }
    fn get_active(&self) -> MemberActive {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.get_instance(), "getActive", &[]).unwrap();
        MemberActive::from_instance(instance)
    }
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
pub trait AsFriend {
    fn as_friend(&self) -> Friend;
}

// TODO: 为 `Bot`, `NormalMember`, 实现。
pub trait AsStranger {
    fn as_stranger(&self) -> Friend;
}

pub trait AnnouncementTrait: GetInstanceTrait {
    /// 内容。
    fn get_content(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let content = jvm.invoke(&self.get_instance(), "getContent", &[]).unwrap();
        jvm.to_rust(content).unwrap()
    }
    /// 公告的附加属性。
    ///
    /// 参见 [`AnnouncementParameters`].
    fn get_parameters(&self) -> AnnouncementParameters {
        let jvm = Jvm::attach_thread().unwrap();
        let paras = jvm
            .invoke(&self.get_instance(), "getParameters", &[])
            .unwrap();
        AnnouncementParameters::from_instance(paras)
    }
    /// 创建 [`OfflineAnnouncement`]. 也可以使用 `self.into()` 或 [`OfflineAnnouncement::from`].
    fn to_offline(&self) -> OfflineAnnouncement {
        let jvm = Jvm::attach_thread().unwrap();
        let a = InvocationArg::try_from(self.get_instance()).unwrap();
        let offline = jvm
            .invoke_static(
                "net.mamoe.mirai.contact.announcement.AnnouncementKt",
                "toOffline",
                &[a],
            )
            .unwrap();
        OfflineAnnouncement::from_instance(offline)
    }
    /// 将该公告发布到群。需要管理员权限。发布公告后群内将会出现 "有新公告" 系统提示。
    ///
    /// 需要处理的错误有：[`MiraiRsErrorEnum::PermissionDenied`], [`MiraiRsErrorEnum::IllegalState`].
    fn publish_to(&self, group: Group) -> Result<OnlineAnnouncement, MiraiRsError> {
        let jvm = Jvm::attach_thread().unwrap();
        let group = group.get_instance();
        let group = InvocationArg::try_from(group).unwrap();
        let online = jvm.invoke(&self.get_instance(), "publishTo", &[group])?;
        Ok(OnlineAnnouncement::from_instance(online))
    }
}

pub trait PublishAnnouncementSupportedTrait {
    fn publish_announcement(&self, content: &str) -> Result<OnlineAnnouncement, MiraiRsError>;
    fn publish_announcement_with_parameters(
        &self,
        content: &str,
        parameters: AnnouncementParameters,
    ) -> Result<OnlineAnnouncement, MiraiRsError>;
}
