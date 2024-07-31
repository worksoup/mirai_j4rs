use j4rs::{InvocationArg, Jvm};

use crate::contact::Bot;
use crate::message::action::Nudge;
use crate::message::data::OfflineAudio;
use crate::utils::backend::BotBackend;
use crate::utils::data_wrapper::{DataWrapper, PrimitiveConvert};
use crate::{
    contact::{
        group::{
            AnnouncementParameters, Group, MemberActive, MemberPermission, OfflineAnnouncement,
            OnlineAnnouncement,
        },
        Friend,
    },
    error::MiraiRsError,
    message::{data::Image, MessageReceipt, MessageTrait},
    utils::{
        contact::file::{ExternalResource, RemoteFiles},
        other::enums::AvatarSpec,
    },
};
use jbuchong::{
    AsInstanceTrait, FromInstanceTrait, GetClassTypeTrait, GetInstanceTrait, TryFromInstanceTrait,
};
use mj_base::MIRAI_PREFIX;
use mj_helper_macro::{error_msg_suppressor, java_fn};

pub trait AssertMemberPermissionTrait<B: BotBackend>: MemberTrait<B> {
    fn is_owner(&self) -> bool;
    fn is_administrator(&self) -> bool;
    fn is_operator(&self) -> bool;
}

pub trait ContactOrBotTrait<B: BotBackend>
where
    Self: Sized + GetInstanceTrait + TryFromInstanceTrait + AsInstanceTrait,
{
    #[java_fn]
    fn get_bot(&self) -> Bot<B> {}
    #[java_fn]
    fn get_id(&self) -> i64 {}

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
        let size = j4rs::InvocationArg::from(
            Jvm::attach_thread()
                .unwrap()
                .field(
                    &Jvm::attach_thread()
                        .unwrap()
                        .static_class(<AvatarSpec as GetClassTypeTrait>::get_type_name())
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
        );
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.get_instance().unwrap(), "getAvatarUrl", &[size])
                    .unwrap(),
            )
            .unwrap()
    }
}

pub trait ContactTrait<B: BotBackend>
where
    Self: ContactOrBotTrait<B>,
{
}

pub trait SendMessageSupportedTrait<B: BotBackend>: ContactTrait<B> {
    fn send_message(&self, message: &impl MessageTrait<B>) -> MessageReceipt<B, Self> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.get_instance().unwrap(),
                "sendMessage",
                &[j4rs::InvocationArg::from(message.get_instance().unwrap())],
            )
            .unwrap();
        MessageReceipt::new(instance, self)
    }
    #[java_fn]
    fn send_string(&self, string: DataWrapper<&str>) -> MessageReceipt<B, Self> {
        let instance = error_msg_suppressor!("instance");
        MessageReceipt::new(instance, self)
    }
    #[java_fn]
    fn upload_image(&self, resource: &ExternalResource) -> Image<B> {}
    fn upload_image_from_file(&self, path: &str) -> Image<B> {
        // let jvm = Jvm::attach_thread().unwrap();
        let resource = ExternalResource::create_from_file(path);
        let image = self.upload_image(&resource);
        // Mirai 文档里说要 close.
        resource.close();
        image
    }
}

pub trait FileSupportedTrait<B: BotBackend>
where
    Self: ContactTrait<B>,
{
    fn get_files(&self) -> RemoteFiles<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .cast(
                &self.get_instance().unwrap(),
                (MIRAI_PREFIX.to_string() + "contact.FileSupported").as_str(),
            )
            .unwrap();
        let instance = jvm
            .invoke(&instance, "getFiles", InvocationArg::empty())
            .unwrap();
        RemoteFiles::from_instance(instance)
    }
}

pub trait AudioSupportedTrait<B: BotBackend>
where
    Self: ContactTrait<B>,
{
    #[java_fn]
    fn upload_audio(&self, resource: &ExternalResource) -> OfflineAudio<B> {}
}

pub trait UserOrBotTrait<B: BotBackend>
where
    Self: ContactOrBotTrait<B>,
{
}

pub trait NudgeSupportedTrait<B: BotBackend>: UserOrBotTrait<B> {
    #[java_fn]
    fn nudge(&self) -> Nudge<B, Self> {}
}

pub trait UserTrait<B: BotBackend>
where
    Self: UserOrBotTrait<B> + ContactTrait<B>,
{
}

pub trait MemberTrait<B: BotBackend>
where
    Self: UserTrait<B>,
{
    #[java_fn]
    fn get_group(&self) -> Group<B> {}
    #[java_fn]
    fn get_active(&self) -> MemberActive {}
    #[java_fn]
    fn get_name_card(&self) -> String {}
    #[java_fn]
    fn get_permission(&self) -> MemberPermission {
        let jvm = error_msg_suppressor!("jvm");
        let perm = jvm
            .invoke(
                &error_msg_suppressor!("instance"),
                "getLevel",
                InvocationArg::empty(),
            )
            .unwrap();
        let perm: i32 = jvm.to_rust(perm).unwrap();
        MemberPermission::from(perm)
    }
    #[java_fn]
    fn get_rank_title(&self) -> String {}
    #[java_fn]
    fn get_special_title(&self) -> String {}
    #[java_fn]
    fn get_temperature_title(&self) -> String {}
    // TODO: 会抛出错误。
    #[java_fn]
    fn mute(&self, duration_seconds: DataWrapper<i64, PrimitiveConvert>) {}
}

// TODO: 为 `Bot`, `Stranger`, `NormalMember`, 实现。
pub trait AsFriend<B: BotBackend>: UserOrBotTrait<B> {
    #[java_fn]
    fn as_friend(&self) -> Friend<B> {}
}

// TODO: 为 `Bot`, `NormalMember`, 实现。
pub trait AsStranger<B: BotBackend>: UserOrBotTrait<B> {
    #[java_fn]
    fn as_stranger(&self) -> Friend<B> {}
}

pub trait AnnouncementTrait<B: BotBackend>: GetInstanceTrait {
    /// 内容。
    #[java_fn]
    fn get_content(&self) -> String {}
    /// 公告的附加属性。
    ///
    /// 参见 [`AnnouncementParameters`].
    #[java_fn]
    fn get_parameters(&self) -> AnnouncementParameters {}
    /// 创建 [`OfflineAnnouncement`]. 也可以使用 `self.into()` 或 [`OfflineAnnouncement::from`].
    fn to_offline(&self) -> OfflineAnnouncement<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let a = InvocationArg::try_from(self.get_instance()).unwrap();
        let offline = jvm
            .invoke_static(
                (MIRAI_PREFIX.to_string() + "contact.announcement.AnnouncementKt").as_str(),
                "toOffline",
                &[a],
            )
            .unwrap();
        OfflineAnnouncement::from_instance(offline)
    }
    /// 将该公告发布到群。需要管理员权限。发布公告后群内将会出现 "有新公告" 系统提示。
    ///
    /// 需要处理的错误有：[`MiraiRsErrorEnum::PermissionDenied`], [`MiraiRsErrorEnum::IllegalState`].
    #[java_fn]
    fn publish_to(&self, group: Group<B>) -> Result<OnlineAnnouncement<B>, MiraiRsError> {
        let online = error_msg_suppressor!("instance");
        Ok(OnlineAnnouncement::from_instance(online))
    }
}

pub trait PublishAnnouncementSupportedTrait<B: BotBackend> {
    fn publish_announcement(&self, content: &str) -> Result<OnlineAnnouncement<B>, MiraiRsError>;
    fn publish_announcement_with_parameters(
        &self,
        content: &str,
        parameters: AnnouncementParameters,
    ) -> Result<OnlineAnnouncement<B>, MiraiRsError>;
}
