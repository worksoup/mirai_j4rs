use j4rs::{errors::J4RsError, Instance, InvocationArg, Jvm};
use jbuchong::{
    java_all, utils::java_iter_to_rust_hash_set, FromInstanceTrait, GetClassTypeTrait,
    GetInstanceTrait, TryFromInstanceTrait,
};
use mj_helper_macro::{java_fn, mj_all};
use std::collections::HashSet;

use crate::{
    contact::{
        AudioSupportedTrait, Bot, ContactOrBotTrait, ContactTrait, FileSupportedTrait,
        NormalMember, PublishAnnouncementSupportedTrait, SendMessageSupportedTrait,
    },
    error::MiraiRsError,
    message::data::{MessageChain, MessageSource},
    utils::{
        backend::BotBackend,
        contact::ContactList,
        data_wrapper::DataWrapper,
        other::enums::{AvatarSpec, GroupHonorType, MemberMedalType},
    },
};
pub use group_settings::*;

pub use announcements::*;
pub use group_active::*;

mod group_settings {
    use j4rs::{Instance, InvocationArg, Jvm};
    use jbuchong::java_all;

    #[java_all]
    pub struct GroupSettings {
        instance: Instance,
    }

    impl GroupSettings {
        pub fn is_allow_member_invite(&self) -> bool {
            Jvm::attach_thread()
                .unwrap()
                .chain(&self.instance)
                .unwrap()
                .invoke("isAllowMemberInvite", InvocationArg::empty())
                .unwrap()
                .to_rust()
                .unwrap()
        }
        pub fn is_anonymous_chat_enabled(&self) -> bool {
            Jvm::attach_thread()
                .unwrap()
                .chain(&self.instance)
                .unwrap()
                .invoke("isAnonymousChatEnabled", InvocationArg::empty())
                .unwrap()
                .to_rust()
                .unwrap()
        }
        pub fn is_auto_approve_enabled(&self) -> bool {
            Jvm::attach_thread()
                .unwrap()
                .chain(&self.instance)
                .unwrap()
                .invoke("isAutoApproveEnabled", InvocationArg::empty())
                .unwrap()
                .to_rust()
                .unwrap()
        }
        pub fn is_mute_all(&self) -> bool {
            Jvm::attach_thread()
                .unwrap()
                .chain(&self.instance)
                .unwrap()
                .invoke("isMuteAll", InvocationArg::empty())
                .unwrap()
                .to_rust()
                .unwrap()
        }
        pub fn set_allow_member_invite(&self, yes: bool) {
            Jvm::attach_thread()
                .unwrap()
                .chain(&self.instance)
                .unwrap()
                .invoke(
                    "setAllowMemberInvite",
                    &[InvocationArg::try_from(yes)
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap();
        }
        pub fn set_anonymous_chat_enabled(&self, yes: bool) {
            Jvm::attach_thread()
                .unwrap()
                .chain(&self.instance)
                .unwrap()
                .invoke(
                    "setAnonymousChatEnabled",
                    &[InvocationArg::try_from(yes)
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap();
        }
        pub fn set_mute_all(&self, yes: bool) {
            Jvm::attach_thread()
                .unwrap()
                .chain(&self.instance)
                .unwrap()
                .invoke(
                    "setMuteAll",
                    &[InvocationArg::try_from(yes)
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap();
        }
    }
}
mod group_active {
    use crate::contact::{ActiveChart, ActiveHonorList, ActiveRankRecord};
    use crate::utils::backend::BotBackend;
    use crate::utils::{MiraiList, MiraiMap};
    use j4rs::{Instance, InvocationArg, Jvm};
    use jbuchong::TryFromInstanceTrait;
    use mj_helper_macro::mj_all;
    use std::collections::HashMap;

    #[mj_all("contact.active.GroupActive")]
    pub struct GroupActive<B: BotBackend> {
        instance: Instance,
        _backend: B,
    }

    impl<B: BotBackend> GroupActive<B> {
        pub fn get_rank_titles(&self) -> MiraiMap<i32, String> {
            MiraiMap {
                instance: Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "getRankTitles", InvocationArg::empty())
                    .unwrap(),
                _t: None,
            }
        }
        pub fn get_temperature_titles(&self) -> MiraiMap<i32, String> {
            MiraiMap {
                instance: Jvm::attach_thread()
                    .unwrap()
                    .invoke(
                        &self.instance,
                        "getTemperatureTitles",
                        InvocationArg::empty(),
                    )
                    .unwrap(),
                _t: None,
            }
        }
        pub fn is_honor_visible(&self) -> bool {
            Jvm::attach_thread()
                .unwrap()
                .to_rust(
                    Jvm::attach_thread()
                        .unwrap()
                        .invoke(&self.instance, "isHonorVisible", InvocationArg::empty())
                        .unwrap(),
                )
                .unwrap()
        }
        pub fn is_temperature_visible(&self) -> bool {
            Jvm::attach_thread()
                .unwrap()
                .to_rust(
                    Jvm::attach_thread()
                        .unwrap()
                        .invoke(
                            &self.instance,
                            "isTemperatureVisible",
                            InvocationArg::empty(),
                        )
                        .unwrap(),
                )
                .unwrap()
        }
        pub fn is_title_visible(&self) -> bool {
            Jvm::attach_thread()
                .unwrap()
                .to_rust(
                    Jvm::attach_thread()
                        .unwrap()
                        .invoke(&self.instance, "isTitleVisible", InvocationArg::empty())
                        .unwrap(),
                )
                .unwrap()
        }
        pub fn query_active_rank(&self) -> MiraiList<B, ActiveRankRecord<B>> {
            MiraiList::try_from_instance(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "queryActiveRank", InvocationArg::empty())
                    .unwrap(),
            )
            .unwrap()
        }
        pub fn query_chart(&self) -> ActiveChart {
            ActiveChart {
                instance: Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "queryChart", InvocationArg::empty())
                    .unwrap(),
            }
        }
        // need to test
        pub fn query_honor_history(&self) -> ActiveHonorList {
            let jvm = Jvm::attach_thread().unwrap();
            ActiveHonorList {
                instance: jvm
                    .cast(
                        &jvm.invoke(&self.instance, "queryHonorHistory", InvocationArg::empty())
                            .unwrap(),
                        "ActiveHonorList",
                    )
                    .unwrap(),
            }
        }
        pub fn refresh(&self) {
            Jvm::attach_thread()
                .unwrap()
                .invoke(&self.instance, "refresh", InvocationArg::empty())
                .unwrap();
        }
        pub fn set_honor_visible(&self, visible: bool) {
            Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self.instance,
                    "setHonorVisible",
                    &[InvocationArg::try_from(visible)
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap();
        }
        pub fn set_rank_titles(&self, map: HashMap<i32, String>) {
            let mirai_map_instance = Jvm::attach_thread()
                .unwrap()
                .java_map(j4rs::JavaClass::Integer, j4rs::JavaClass::String, map)
                .unwrap();
            Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self.instance,
                    "setRankTitles",
                    &[InvocationArg::from(mirai_map_instance)
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap();
        }
        pub fn set_temperature_titles(&self, map: HashMap<i32, String>) {
            let mirai_map_instance = Jvm::attach_thread()
                .unwrap()
                .java_map(j4rs::JavaClass::Integer, j4rs::JavaClass::String, map)
                .unwrap();
            Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self.instance,
                    "setTemperatureTitles",
                    &[InvocationArg::from(mirai_map_instance)
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap();
        }
        pub fn set_temperature_visible(&self, visible: bool) {
            Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self.instance,
                    "setTemperatureVisible",
                    &[InvocationArg::try_from(visible)
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap();
        }
        pub fn set_title_visible(&self, visible: bool) {
            Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self.instance,
                    "setTitleVisible",
                    &[InvocationArg::try_from(visible)
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap();
        }
    }
}
mod announcements {
    use crate::contact::{AnnouncementTrait, Bot, ContactOrBotTrait, Group, NormalMember};
    use crate::error::MiraiRsError;
    use crate::message::MessageHashCodeTrait;
    use crate::utils::backend::BotBackend;
    use crate::utils::contact::file::ExternalResource;
    use crate::utils::data_wrapper::{DataWrapper, DefaultConvert};
    use crate::utils::JavaStream;
    use j4rs::errors::J4RsError;
    use j4rs::{Instance, InvocationArg, Jvm};
    use jbuchong::utils::{instance_is_null, java_iter_to_rust_vec};
    use jbuchong::{java_all, GetInstanceTrait, TryFromInstanceTrait};
    use mj_base::MIRAI_PREFIX;
    use mj_helper_macro::{error_msg_suppressor, java_fn, mj_all};
    use std::fmt::{Display, Formatter};

    /// 群公告的附加参数。
    /// 字段均为公开，可以直接构造。
    /// 同时可以通过 [`AnnouncementParameters::default`] 方法获取一个默认的实例。
    /// 字段含义见结构体内注释。
    #[derive(Default)]
    pub struct AnnouncementParameters {
        /// 群公告的图片，目前仅支持发送图片，不支持获得图片。可通过 [`Announcements::upload_image_from_file`] 上传图片。
        ///
        /// 另见 [`AnnouncementImage`]
        pub image: Option<AnnouncementImage>,
        /// 发送给新成员。
        pub send_to_new_member: bool,
        /// 置顶. 可以有多个置顶公告。
        pub is_pinned: bool,
        /// 显示能够引导群成员修改昵称的窗口。
        pub show_edit_card: bool,
        /// 使用弹窗。
        pub show_popup: bool,
        /// 需要群成员确认。
        pub require_confirmation: bool,
    }

    impl GetInstanceTrait for AnnouncementParameters {
        fn get_instance(&self) -> Result<Instance, J4RsError> {
            let jvm = Jvm::attach_thread().unwrap();
            let mut builder = jvm
                .create_instance(
                    (MIRAI_PREFIX.to_string()
                        + "contact.announcement.AnnouncementParametersBuilder")
                        .as_str(),
                    InvocationArg::empty(),
                )
                .unwrap();
            if let Some(image) = &self.image {
                let image = image.get_instance();
                let image = InvocationArg::try_from(image).unwrap();
                builder = jvm.invoke(&builder, "image", &[image]).unwrap();
            }
            let is_pinned = InvocationArg::try_from(self.is_pinned)
                .unwrap()
                .into_primitive()
                .unwrap();
            let require_confirmation = InvocationArg::try_from(self.require_confirmation)
                .unwrap()
                .into_primitive()
                .unwrap();
            let send_to_new_member = InvocationArg::try_from(self.send_to_new_member)
                .unwrap()
                .into_primitive()
                .unwrap();
            let show_edit_card = InvocationArg::try_from(self.show_edit_card)
                .unwrap()
                .into_primitive()
                .unwrap();
            let show_popup = InvocationArg::try_from(self.show_popup)
                .unwrap()
                .into_primitive()
                .unwrap();
            builder = jvm.invoke(&builder, "isPinned", &[is_pinned]).unwrap();
            builder = jvm
                .invoke(&builder, "requireConfirmation", &[require_confirmation])
                .unwrap();
            builder = jvm
                .invoke(&builder, "sendToNewMember", &[send_to_new_member])
                .unwrap();
            builder = jvm
                .invoke(&builder, "showEditCard", &[show_edit_card])
                .unwrap();
            builder = jvm.invoke(&builder, "showPopup", &[show_popup]).unwrap();
            Ok(jvm
                .invoke(&builder, "build", InvocationArg::empty())
                .unwrap())
        }
    }

    impl TryFromInstanceTrait for AnnouncementParameters {
        fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
            let jvm = Jvm::attach_thread().unwrap();
            let image = jvm
                .invoke(&instance, "image", InvocationArg::empty())
                .unwrap();
            let image = if !instance_is_null(&image) {
                Some(AnnouncementImage { instance: image })
            } else {
                None
            };
            let is_pinned = jvm
                .invoke(&instance, "isPinned", InvocationArg::empty())
                .unwrap();
            let is_pinned = jvm.to_rust(is_pinned).unwrap();
            let require_confirmation = jvm
                .invoke(&instance, "requireConfirmation", InvocationArg::empty())
                .unwrap();
            let require_confirmation = jvm.to_rust(require_confirmation).unwrap();
            let send_to_new_member = jvm
                .invoke(&instance, "sendToNewMember", InvocationArg::empty())
                .unwrap();
            let send_to_new_member = jvm.to_rust(send_to_new_member).unwrap();
            let show_edit_card = jvm
                .invoke(&instance, "showEditCard", InvocationArg::empty())
                .unwrap();
            let show_edit_card = jvm.to_rust(show_edit_card).unwrap();
            let show_popup = jvm
                .invoke(&instance, "showPopup", InvocationArg::empty())
                .unwrap();
            let show_popup = jvm.to_rust(show_popup).unwrap();
            Ok(AnnouncementParameters {
                image,
                send_to_new_member,
                is_pinned,
                show_edit_card,
                show_popup,
                require_confirmation,
            })
        }
    }

    #[mj_all("contact.announcement.OfflineAnnouncement")]
    pub struct OfflineAnnouncement<B: BotBackend> {
        instance: Instance,
        _backend: B,
    }
    impl<B: BotBackend> OfflineAnnouncement<B> {
        #[java_fn("from")]
        fn from_announcement_internal(announcement: Instance) -> Self {}
        pub fn from_announcement(announcement: impl AnnouncementTrait<B>) -> Self {
            Self::from_announcement_internal(announcement.get_instance().unwrap())
        }
    }

    impl<B: BotBackend> From<Announcement<B>> for OfflineAnnouncement<B> {
        fn from(announcement: Announcement<B>) -> Self {
            OfflineAnnouncement::from_announcement(announcement)
        }
    }

    impl<B: BotBackend> From<OnlineAnnouncement<B>> for OfflineAnnouncement<B> {
        fn from(online_announcement: OnlineAnnouncement<B>) -> Self {
            OfflineAnnouncement::from_announcement(online_announcement)
        }
    }

    impl<B: BotBackend> AnnouncementTrait<B> for OfflineAnnouncement<B> {}

    /// 在线公告，也就是已经发送的存在于服务器的公告。
    ///
    /// 依靠 [`fid`][OnlineAnnouncement::get_fid] 唯一识别。可[删除][OnlineAnnouncement::delete]。
    ///
    /// 另见 [`Announcement`] 与 [`AnnouncementTrait`]
    #[mj_all("contact.announcement.OnlineAnnouncement")]
    pub struct OnlineAnnouncement<B: BotBackend> {
        instance: Instance,
        _backend: B,
    }

    impl<B: BotBackend> OnlineAnnouncement<B> {
        /// 删除这个公告，也可以使用 [`Announcements::delete`].
        ///
        /// 成功返回 `true`, 公告已经被删除则返回 `false`.
        ///
        /// 需要处理的错误有：[`MiraiRsErrorEnum::PermissionDenied`], [`MiraiRsErrorEnum::IllegalState`].
        pub fn delete(&self) -> Result<bool, MiraiRsError> {
            let jvm = Jvm::attach_thread().unwrap();
            let r#bool = jvm.invoke(&self.instance, "delete", InvocationArg::empty())?;
            Ok(jvm.to_rust(r#bool).unwrap())
        }
        /// 所有人都已阅读。如果 [`AnnouncementParameters`] 的 `require_confirmation` 为 `true` 则为所有人都已确认。
        pub fn get_all_confirmed(&self) -> bool {
            let jvm = Jvm::attach_thread().unwrap();
            let r#bool = jvm
                .invoke(&self.instance, "getAllConfirmed", InvocationArg::empty())
                .unwrap();
            jvm.to_rust(r#bool).unwrap()
        }
        /// 已阅读成员的数量。如果 [`AnnouncementParameters`] 的 `require_confirmation` 为 `true` 则为已确认成员的数量。
        pub fn get_confirmed_members_count(&self) -> i32 {
            let jvm = Jvm::attach_thread().unwrap();
            let r#i32 = jvm
                .invoke(
                    &self.instance,
                    "getConfirmedMembersCount",
                    InvocationArg::empty(),
                )
                .unwrap();
            jvm.to_rust(r#i32).unwrap()
        }
        /// 唯一识别属性。
        pub fn get_fid(&self) -> String {
            let jvm = Jvm::attach_thread().unwrap();
            let fid = jvm
                .invoke(&self.instance, "getFid", InvocationArg::empty())
                .unwrap();
            jvm.to_rust(fid).unwrap()
        }
        /// 公告所属群。
        pub fn get_group(&self) -> Group<B> {
            let jvm = Jvm::attach_thread().unwrap();
            let group = jvm
                .invoke(&self.instance, "getGroup", InvocationArg::empty())
                .unwrap();
            Group::try_from_instance(group).unwrap()
        }
        /// 公告所在群所属的 [`Bot`].
        ///
        /// 相当于 `self.get_group().get_bot()`.
        pub fn get_bot(&self) -> Bot<B> {
            self.get_group().get_bot()
        }
        /// 公告发出时的时间戳。
        ///
        /// 另见 [std::time::UNIX_EPOCH].
        pub fn get_publication_time(&self) -> i64 {
            let jvm = Jvm::attach_thread().unwrap();
            let time = jvm
                .invoke(&self.instance, "getPublicationTime", InvocationArg::empty())
                .unwrap();
            jvm.to_rust(time).unwrap()
        }
        /// [公告发送者][NormalMember]。该成员可能已经离开群，此时返回 `None`.
        pub fn get_sender(&self) -> Option<NormalMember<B>> {
            let jvm = Jvm::attach_thread().unwrap();
            let sender = jvm
                .invoke(&self.instance, "getSender", InvocationArg::empty())
                .unwrap();
            if !instance_is_null(&sender) {
                NormalMember::try_from_instance(sender).ok()
            } else {
                None
            }
        }
        /// [公告发送者][NormalMember] id.
        pub fn get_sender_id(&self) -> i64 {
            let jvm = Jvm::attach_thread().unwrap();
            let id = jvm
                .invoke(&self.instance, "getSenderId", InvocationArg::empty())
                .unwrap();
            jvm.to_rust(id).unwrap()
        }
        /// 获取已确认或未确认（指定 `confirmed`）的群成员。
        ///
        /// 需要处理的错误有：[`MiraiRsErrorEnum::PermissionDenied`], [`MiraiRsErrorEnum::IllegalState`].
        pub fn members(&self, confirmed: bool) -> Result<Vec<NormalMember<B>>, MiraiRsError> {
            let jvm = Jvm::attach_thread().unwrap();
            let confirmed = InvocationArg::try_from(confirmed)
                .unwrap()
                .into_primitive()
                .unwrap();
            let members = jvm.invoke(&self.instance, "members", &[confirmed])?;
            let iter = jvm
                .invoke(&members, "iterator", InvocationArg::empty())
                .unwrap();
            Ok(java_iter_to_rust_vec(&jvm, iter))
        }
        /// 提醒未确认的群成员。
        ///
        /// 需要处理的错误有：[`MiraiRsErrorEnum::PermissionDenied`], [`MiraiRsErrorEnum::IllegalState`].
        pub fn remind(&self) -> Result<(), MiraiRsError> {
            let jvm = Jvm::attach_thread().unwrap();
            let _ = jvm.invoke(&self.instance, "remind", InvocationArg::empty())?;
            Ok(())
        }
    }

    impl<B: BotBackend> AnnouncementTrait<B> for OnlineAnnouncement<B> {}

    /// 群公告。可以是 [`OnlineAnnouncement`] 或 [`OfflineAnnouncement`].
    ///
    /// ## 发布公告
    ///
    /// ### 构造一条新公告并发布
    ///
    /// 构造 [`OfflineAnnouncement`] 然后调用 [`OfflineAnnouncement::publish_to`] 或 [`Announcements::publish`]
    ///
    /// 构造时的 [`AnnouncementParameters`] 可以设置一些附加属性。
    ///
    /// 也可以使用 [`Group::publish_announcement`] 和 [`Group::publish_announcement_with_parameters`] 创建并发布公告。
    ///
    /// ### 转发获取的公告到其他群
    ///
    /// 通过一个群的 [`Announcements`] 获取到 [`OnlineAnnouncement`], 然后调用 [`OnlineAnnouncement::publish_to`] 即可。
    /// 由于 `Mirai` 目前不支持获取公告图片，所以转发的公告也不会带有原公告的图片。
    #[mj_all("contact.announcement.Announcement")]
    pub enum Announcement<B: BotBackend> {
        OnlineAnnouncement(OnlineAnnouncement<B>),
        OfflineAnnouncement(OfflineAnnouncement<B>),
    }

    impl<B: BotBackend> AnnouncementTrait<B> for Announcement<B> {}

    #[java_all]
    pub struct AnnouncementImage {
        instance: Instance,
    }

    impl AnnouncementImage {
        pub fn new(id: &str, h: i32, w: i32) -> Self {
            let jvm = Jvm::attach_thread().unwrap();
            let id = InvocationArg::try_from(id).unwrap();
            let h = InvocationArg::try_from(h)
                .unwrap()
                .into_primitive()
                .unwrap();
            let w = InvocationArg::try_from(w)
                .unwrap()
                .into_primitive()
                .unwrap();
            let instance = jvm.invoke_static("", "create", &[id, h, w]).unwrap();
            Self { instance }
        }
        pub fn get_height(&self) -> i32 {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = jvm
                .invoke_static("", "getHeight", InvocationArg::empty())
                .unwrap();
            jvm.to_rust(instance).unwrap()
        }
        pub fn get_width(&self) -> i32 {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = jvm
                .invoke_static("", "getWidth", InvocationArg::empty())
                .unwrap();
            jvm.to_rust(instance).unwrap()
        }
        pub fn get_id(&self) -> String {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = jvm
                .invoke_static("", "getId", InvocationArg::empty())
                .unwrap();
            jvm.to_rust(instance).unwrap()
        }
        pub fn get_url(&self) -> String {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = jvm
                .invoke_static("", "getUrl", InvocationArg::empty())
                .unwrap();
            jvm.to_rust(instance).unwrap()
        }
    }
    impl Display for AnnouncementImage {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(
                {
                    let jvm = Jvm::attach_thread().unwrap();
                    let instance = jvm
                        .invoke_static("", "toString", InvocationArg::empty())
                        .unwrap();
                    jvm.to_rust::<String>(instance).unwrap()
                }
                .as_str(),
            )
        }
    }

    impl MessageHashCodeTrait for AnnouncementImage {}

    #[mj_all("contact.announcement.Announcements")]
    pub struct Announcements<B: BotBackend> {
        instance: Instance,
        _backend: B,
    }

    impl<B: BotBackend> Announcements<B> {
        #[java_fn]
        pub fn as_stream(&self) -> JavaStream<OnlineAnnouncement<B>> {}
        #[java_fn]
        pub fn delete(&self, fid: DataWrapper<&str, DefaultConvert>) -> bool {}
        #[java_fn]
        pub fn get(&self, fid: DataWrapper<&str, DefaultConvert>) -> OnlineAnnouncement<B> {}
        #[java_fn]
        pub fn members(&self) -> Vec<NormalMember<B>> {
            let jvm: Jvm = error_msg_suppressor!("jvm");
            let list: Instance = error_msg_suppressor!("instance");
            let iter = jvm
                .invoke(&list, "iterator", InvocationArg::empty())
                .unwrap();
            java_iter_to_rust_vec(&jvm, iter)
        }
        #[java_fn]
        pub fn publish(&self, announcement: Announcement<B>) -> OnlineAnnouncement<B> {}
        #[java_fn]
        pub fn remind(&self, fid: DataWrapper<&str, DefaultConvert>) {}
        #[java_fn]
        pub fn upload_image(&self, resource: &ExternalResource) -> AnnouncementImage {}
        pub fn upload_image_from_file(&self, path: &str) -> AnnouncementImage {
            let res = ExternalResource::create_from_file(path);
            let a = self.upload_image(&res);
            res.close();
            a
        }
    }
}
#[derive(
    num_enum::FromPrimitive,
    num_enum::IntoPrimitive,
    Debug,
    PartialOrd,
    Ord,
    Eq,
    PartialEq,
    Copy,
    Clone,
)]
#[repr(i32)]
pub enum MemberPermission {
    Member = 0,
    Administrator = 1,
    #[default]
    Owner = 2,
}

// impl MemberPermission {
//     fn internal_to_i32(a: &MemberPermission) -> i32 {
//         unsafe { *(a as *const MemberPermission as *const i32) }
//     }
// }

#[mj_all("contact.active.ActiveRankRecord")]
pub struct ActiveRankRecord<B: BotBackend> {
    instance: Instance,
    member_name: Option<String>,
    member_id: Option<i64>,
    temperature: Option<i32>,
    score: Option<i32>,
    _backend: B,
}

impl<B: BotBackend> ActiveRankRecord<B> {
    pub fn new(
        member_name: String,
        member_id: i64,
        member: NormalMember<B>,
        temperature: i32,
        score: i32,
    ) -> ActiveRankRecord<B> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .create_instance(
                <Self as GetClassTypeTrait>::get_type_name(),
                &[
                    InvocationArg::try_from(member_name.clone()).unwrap(),
                    InvocationArg::try_from(member_id)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(member.get_instance()).unwrap(),
                    InvocationArg::try_from(temperature)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(score)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                ],
            )
            .unwrap();
        ActiveRankRecord {
            instance,
            member_name: Some(member_name),
            member_id: Some(member_id),
            temperature: Some(temperature),
            score: Some(score),
            _backend: B::default(),
        }
    }
    pub fn get_member(&self) -> NormalMember<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getMember", InvocationArg::empty())
            .unwrap();
        // 笔记： rust 中此类代码的行为：完全限定的方法调用。
        // 同时指定了特型和类型。
        // 如果是 `FromInstance` 的话，应该是调用了默认的实现？
        <NormalMember<B> as TryFromInstanceTrait>::try_from_instance(instance).unwrap()
    }
    pub fn get_member_id(&self) -> i64 {
        if let Some(id) = self.member_id {
            id
        } else {
            Jvm::attach_thread()
                .unwrap()
                .to_rust(
                    Jvm::attach_thread()
                        .unwrap()
                        .invoke(&self.instance, "getMemberId", InvocationArg::empty())
                        .unwrap(),
                )
                .unwrap()
        }
    }
    pub fn get_member_name(&self) -> String {
        if let Some(name) = &self.member_name {
            name.clone()
        } else {
            Jvm::attach_thread()
                .unwrap()
                .to_rust(
                    Jvm::attach_thread()
                        .unwrap()
                        .invoke(&self.instance, "getMemberName", InvocationArg::empty())
                        .unwrap(),
                )
                .unwrap()
        }
    }
    pub fn get_score(&self) -> i32 {
        if let Some(score) = self.score {
            score
        } else {
            Jvm::attach_thread()
                .unwrap()
                .to_rust(
                    Jvm::attach_thread()
                        .unwrap()
                        .invoke(&self.instance, "getScore", InvocationArg::empty())
                        .unwrap(),
                )
                .unwrap()
        }
    }
    pub fn get_temperature(&self) -> i32 {
        if let Some(temperature) = self.temperature {
            temperature
        } else {
            Jvm::attach_thread()
                .unwrap()
                .to_rust(
                    Jvm::attach_thread()
                        .unwrap()
                        .invoke(&self.instance, "getTemperature", InvocationArg::empty())
                        .unwrap(),
                )
                .unwrap()
        }
    }
}
#[java_all]
pub struct ActiveChart {
    instance: Instance,
}

#[java_all]
pub struct ActiveHonorInfo {
    instance: Instance,
}

/// 群荣耀历史数据
#[java_all]
pub struct ActiveHonorList {
    instance: Instance,
}

pub struct MemberMedalInfo {
    instance: Instance,
}

impl MemberMedalInfo {
    pub fn get_color(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getColor", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn get_medals(&self) -> HashSet<MemberMedalType> {
        let jvm = Jvm::attach_thread().unwrap();
        let set = jvm
            .invoke(&self.instance, "getHonors", InvocationArg::empty())
            .unwrap();
        let iter = jvm
            .invoke(&set, "iterator", InvocationArg::empty())
            .unwrap();
        java_iter_to_rust_hash_set(&jvm, iter)
    }
    pub fn get_title(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getTitle", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn get_wearing(&self) -> MemberMedalType {
        let jvm = Jvm::attach_thread().unwrap();
        MemberMedalType::try_from_instance(
            jvm.invoke(&self.instance, "getWearing", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
}

impl TryFromInstanceTrait for MemberMedalInfo {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        Ok(Self { instance })
    }
}

pub struct MemberActive {
    instance: Instance,
}

impl TryFromInstanceTrait for MemberActive {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        Ok(Self { instance })
    }
}

impl MemberActive {
    pub fn get_honors(&self) -> HashSet<GroupHonorType> {
        let jvm = Jvm::attach_thread().unwrap();
        let set = jvm
            .invoke(&self.instance, "getHonors", InvocationArg::empty())
            .unwrap();
        let iter = jvm
            .invoke(&set, "iterator", InvocationArg::empty())
            .unwrap();
        java_iter_to_rust_hash_set(&jvm, iter)
    }
    pub fn get_point(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getPoint", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn get_rank(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getRank", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn get_temperature(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getTemperature", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn query_medal(&self) -> MemberMedalInfo {
        let jvm = Jvm::attach_thread().unwrap();
        MemberMedalInfo {
            instance: jvm
                .invoke(&self.instance, "queryMedal", InvocationArg::empty())
                .unwrap(),
        }
    }
}

#[mj_all("contact.Group")]
pub struct Group<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> PublishAnnouncementSupportedTrait<B> for Group<B> {
    /// 在该群发布公告。需要管理员权限。发布公告后群内将会出现 "有新公告" 系统提示。
    ///
    /// 需要处理的错误有：[`MiraiRsErrorEnum::PermissionDenied`], [`MiraiRsErrorEnum::IllegalState`].
    fn publish_announcement(&self, content: &str) -> Result<OnlineAnnouncement<B>, MiraiRsError> {
        let jvm = Jvm::attach_thread().unwrap();
        let group = self.get_instance();
        let group = InvocationArg::try_from(group).unwrap();
        let content = InvocationArg::try_from(content).unwrap();
        let online = jvm.invoke_static(
            <Announcement<B> as GetClassTypeTrait>::get_type_name(),
            "publishAnnouncement",
            &[group, content],
        )?;
        Ok(OnlineAnnouncement::from_instance(online))
    }
    /// 在该群发布公告。需要管理员权限。发布公告后群内将会出现 "有新公告" 系统提示。
    ///
    /// 需要处理的错误有：[`MiraiRsErrorEnum::PermissionDenied`], [`MiraiRsErrorEnum::IllegalState`].
    ///
    /// 另见 [`AnnouncementParameters`].
    fn publish_announcement_with_parameters(
        &self,
        content: &str,
        parameters: AnnouncementParameters,
    ) -> Result<OnlineAnnouncement<B>, MiraiRsError> {
        let jvm = Jvm::attach_thread().unwrap();
        let group = self.get_instance();
        let group = InvocationArg::try_from(group).unwrap();
        let content = InvocationArg::try_from(content).unwrap();
        let parameters = parameters.get_instance();
        let parameters = InvocationArg::try_from(parameters).unwrap();
        let online = jvm.invoke_static(
            <Announcement<B> as GetClassTypeTrait>::get_type_name(),
            "publishAnnouncement",
            &[group, content, parameters],
        )?;
        Ok(OnlineAnnouncement::from_instance(online))
    }
}

impl<B: BotBackend> SendMessageSupportedTrait<B> for Group<B> {}

impl<B: BotBackend> FileSupportedTrait<B> for Group<B> {}

impl<B: BotBackend> ContactOrBotTrait<B> for Group<B> {
    fn get_avatar_url(&self, size: Option<AvatarSpec>) -> String {
        let size: i32 = if let Some(size) = size {
            size.into()
        } else {
            AvatarSpec::XL.into()
        };
        let id = self.get_id().to_string();
        // 这里 Mirai 源码中应该是 http 而不是 https.
        return "https://p.qlogo.cn/gh/".to_string()
            + id.as_str()
            + "/"
            + id.as_str()
            + "/"
            + size.to_string().as_str();
    }
}

impl<B: BotBackend> ContactTrait<B> for Group<B> {}

impl<B: BotBackend> Group<B> {
    pub fn new(bot: &Bot<B>, id: i64) -> Option<Group<B>> {
        bot.get_group(id)
    }
    pub fn contains_member(&self, member: &NormalMember<B>) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .chain(&self.instance)
            .unwrap()
            .invoke(
                "contains",
                &[InvocationArg::try_from(member.get_instance()).unwrap()],
            )
            .unwrap()
            .to_rust()
            .unwrap()
    }
    pub fn contains_id(&self, id: i64) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .chain(&self.instance)
            .unwrap()
            .invoke(
                "contains",
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap()
            .to_rust()
            .unwrap()
    }
    pub fn get(&self, id: i64) -> Option<NormalMember<B>> {
        NormalMember::in_group(self, id)
    }
    pub fn get_active(&self) -> GroupActive<B> {
        let active_instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getActive", InvocationArg::empty())
            .unwrap();
        GroupActive::from_instance(active_instance)
    }
    #[java_fn]
    pub fn get_announcements(&self) -> Announcements<B> {}
    pub fn get_bot_as_member(&self) -> NormalMember<B> {
        NormalMember::bot_in(self)
    }
    pub fn get_bot_mute_remaining(&self) -> i32 {
        Jvm::attach_thread()
            .unwrap()
            .chain(&self.instance)
            .unwrap()
            .invoke("getBotMuteRemaining", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
    }
    pub fn get_bot_permission(&self) -> MemberPermission {
        let jvm = Jvm::attach_thread().unwrap();
        let prem = jvm
            .invoke(
                &self.instance,
                "getMemberPermission",
                InvocationArg::empty(),
            )
            .unwrap();
        let prem = jvm
            .invoke(&prem, "getLevel", InvocationArg::empty())
            .unwrap();
        MemberPermission::from(jvm.to_rust::<i32>(prem).unwrap())
    }
    #[java_fn]
    pub fn get_members(&self) -> ContactList<B, NormalMember<B>> {}
    pub fn get_name(&self) -> String {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "getName", InvocationArg::empty())
                    .unwrap(),
            )
            .unwrap()
    }
    pub fn get_owner(&self) -> NormalMember<B> {
        NormalMember::owner_of(self)
    }
    #[java_fn]
    pub fn get_settings(&self) -> GroupSettings {}
    pub fn quit(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "quit", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    #[java_fn]
    pub fn set_essence_message(&self, source: MessageSource<B>) -> bool {}
    // function name need to be changed.
    #[java_fn("setEssenceMessage")]
    pub fn set_essence_message_s(&self, chain: MessageChain<B>) -> bool {}
    #[java_fn]
    pub fn set_name(&self, name: DataWrapper<&str>) {}
    // TODO: 获取精华消息。
}

impl<B: BotBackend> AudioSupportedTrait<B> for Group<B> {}
