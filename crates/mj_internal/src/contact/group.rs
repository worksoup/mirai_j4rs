use crate::message::data::Image;
use crate::{
    contact::{
        bot::Bot,
        contact_trait::{
            AnnouncementTrait, ContactOrBotTrait, ContactTrait, FileSupportedTrait,
            PublishAnnouncementSupportedTrait, SendMessageSupportedTrait,
        },
        file::ExternalResource,
        ContactList, NormalMember,
    },
    error::MiraiRsError,
    message::{
        data::{MessageChain, MessageSource},
        MessageHashCodeTrait,
    },
    utils::{
        other::enums::{AvatarSpec, GroupHonorType, MemberMedalType},
        JavaStream,
    },
};
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::{
    env::{FromInstance, GetInstanceTrait},
    utils::{instance_is_null, is_instance_of, java_iter_to_rust_hash_set, java_iter_to_rust_vec},
};
use mj_macro::{java_type, FromInstanceDerive, GetInstanceDerive};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

pub struct GroupSettings {
    instance: Instance,
}

impl GroupSettings {
    pub fn is_allow_member_invite(&self) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .chain(&self.instance)
            .unwrap()
            .invoke("isAllowMemberInvite", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    pub fn is_anonymous_chat_enabled(&self) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .chain(&self.instance)
            .unwrap()
            .invoke("isAnonymousChatEnabled", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    pub fn is_auto_approve_enabled(&self) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .chain(&self.instance)
            .unwrap()
            .invoke("isAutoApproveEnabled", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    pub fn is_mute_all(&self) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .chain(&self.instance)
            .unwrap()
            .invoke("isMuteAll", &[])
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

#[derive(GetInstanceDerive)]
pub struct Group {
    pub(crate) bot: Instance,
    pub(crate) instance: Instance,
    pub(crate) id: i64,
}

impl PublishAnnouncementSupportedTrait for Group {
    /// 在该群发布公告。需要管理员权限。发布公告后群内将会出现 "有新公告" 系统提示。
    ///
    /// 需要处理的错误有：[`MiraiRsErrorEnum::PermissionDenied`], [`MiraiRsErrorEnum::IllegalState`].
    fn publish_announcement(&self, content: &str) -> Result<OnlineAnnouncement, MiraiRsError> {
        let jvm = Jvm::attach_thread().unwrap();
        let group = self.get_instance();
        let group = InvocationArg::try_from(group).unwrap();
        let content = InvocationArg::try_from(content).unwrap();
        let online = jvm.invoke_static(
            "net.mamoe.mirai.contact.announcement.Announcement",
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
    ) -> Result<OnlineAnnouncement, MiraiRsError> {
        let jvm = Jvm::attach_thread().unwrap();
        let group = self.get_instance();
        let group = InvocationArg::try_from(group).unwrap();
        let content = InvocationArg::try_from(content).unwrap();
        let parameters = parameters.get_instance();
        let parameters = InvocationArg::try_from(parameters).unwrap();
        let online = jvm.invoke_static(
            "net.mamoe.mirai.contact.announcement.Announcement",
            "publishAnnouncement",
            &[group, content, parameters],
        )?;
        Ok(OnlineAnnouncement::from_instance(online))
    }
}

impl SendMessageSupportedTrait for Group {}

impl FileSupportedTrait for Group {}

pub struct GroupActive {
    instance: Instance,
}

/// 群公告的附加参数。
/// 字段均为公开，可以直接构造。
/// 同时可以通过 [`AnnouncementParameters::default`] 方法获取一个默认的实例。
/// 字段含义见结构体内注释。
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

impl AnnouncementParameters {
    pub fn default() -> Self {
        AnnouncementParameters {
            image: None,
            send_to_new_member: false,
            is_pinned: false,
            show_edit_card: false,
            show_popup: false,
            require_confirmation: false,
        }
    }
}

impl GetInstanceTrait for AnnouncementParameters {
    fn get_instance(&self) -> Instance {
        let jvm = Jvm::attach_thread().unwrap();
        let mut builder = jvm
            .create_instance(
                "net.mamoe.mirai.contact.announcement.AnnouncementParametersBuilder",
                &[],
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
        jvm.invoke(&builder, "build", &[]).unwrap()
    }
}

impl FromInstance for AnnouncementParameters {
    fn from_instance(instance: Instance) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let image = jvm.invoke(&instance, "image", &[]).unwrap();
        let image = if !instance_is_null(&image) {
            Some(AnnouncementImage { instance: image })
        } else {
            None
        };
        let is_pinned = jvm.invoke(&instance, "isPinned", &[]).unwrap();
        let is_pinned = jvm.to_rust(is_pinned).unwrap();
        let require_confirmation = jvm.invoke(&instance, "requireConfirmation", &[]).unwrap();
        let require_confirmation = jvm.to_rust(require_confirmation).unwrap();
        let send_to_new_member = jvm.invoke(&instance, "sendToNewMember", &[]).unwrap();
        let send_to_new_member = jvm.to_rust(send_to_new_member).unwrap();
        let show_edit_card = jvm.invoke(&instance, "showEditCard", &[]).unwrap();
        let show_edit_card = jvm.to_rust(show_edit_card).unwrap();
        let show_popup = jvm.invoke(&instance, "showPopup", &[]).unwrap();
        let show_popup = jvm.to_rust(show_popup).unwrap();
        AnnouncementParameters {
            image,
            send_to_new_member,
            is_pinned,
            show_edit_card,
            show_popup,
            require_confirmation,
        }
    }
}

#[derive(GetInstanceDerive)]
pub struct OfflineAnnouncement {
    instance: Instance,
}

impl FromInstance for OfflineAnnouncement {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }
}

impl OfflineAnnouncement {
    fn from_announcement(announcement: impl AnnouncementTrait) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let announcement = InvocationArg::try_from(announcement.get_instance()).unwrap();
        let offline_announcement = jvm
            .invoke_static(
                "net.mamoe.mirai.contact.announcement.OfflineAnnouncement",
                "from",
                &[announcement],
            )
            .unwrap();
        Self::from_instance(offline_announcement)
    }
}

impl From<Announcement> for OfflineAnnouncement {
    fn from(announcement: Announcement) -> Self {
        OfflineAnnouncement::from_announcement(announcement)
    }
}

impl From<OnlineAnnouncement> for OfflineAnnouncement {
    fn from(online_announcement: OnlineAnnouncement) -> Self {
        OfflineAnnouncement::from_announcement(online_announcement)
    }
}

impl AnnouncementTrait for OfflineAnnouncement {}

/// 在线公告，也就是已经发送的存在于服务器的公告。
///
/// 依靠 [`fid`][OnlineAnnouncement::get_fid] 唯一识别。可[删除][OnlineAnnouncement::delete]。
///
/// 另见 [`Announcement`] 与 [`AnnouncementTrait`]
#[derive(GetInstanceDerive)]
#[java_type("net.mamoe.mirai.contact.announcement.OnlineAnnouncement")]
pub struct OnlineAnnouncement {
    instance: Instance,
}

impl FromInstance for OnlineAnnouncement {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }
}

impl OnlineAnnouncement {
    /// 删除这个公告，也可以使用 [`Announcements::delete`].
    ///
    /// 成功返回 `true`, 公告已经被删除则返回 `false`.
    ///
    /// 需要处理的错误有：[`MiraiRsErrorEnum::PermissionDenied`], [`MiraiRsErrorEnum::IllegalState`].
    pub fn delete(&self) -> Result<bool, MiraiRsError> {
        let jvm = Jvm::attach_thread().unwrap();
        let r#bool = jvm.invoke(&self.instance, "delete", &[])?;
        Ok(jvm.to_rust(r#bool).unwrap())
    }
    /// 所有人都已阅读。如果 [`AnnouncementParameters`] 的 `require_confirmation` 为 `true` 则为所有人都已确认。
    pub fn get_all_confirmed(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let r#bool = jvm.invoke(&self.instance, "getAllConfirmed", &[]).unwrap();
        jvm.to_rust(r#bool).unwrap()
    }
    /// 已阅读成员的数量。如果 [`AnnouncementParameters`] 的 `require_confirmation` 为 `true` 则为已确认成员的数量。
    pub fn get_confirmed_members_count(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        let r#i32 = jvm
            .invoke(&self.instance, "getConfirmedMembersCount", &[])
            .unwrap();
        jvm.to_rust(r#i32).unwrap()
    }
    /// 唯一识别属性。
    pub fn get_fid(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let fid = jvm.invoke(&self.instance, "getFid", &[]).unwrap();
        jvm.to_rust(fid).unwrap()
    }
    /// 公告所属群。
    pub fn get_group(&self) -> Group {
        let jvm = Jvm::attach_thread().unwrap();
        let group = jvm.invoke(&self.instance, "getGroup", &[]).unwrap();
        Group::from_instance(group)
    }
    /// 公告所在群所属的 [`Bot`].
    ///
    /// 相当于 `self.get_group().get_bot()`.
    pub fn get_bot(&self) -> Bot {
        self.get_group().get_bot()
    }
    /// 公告发出时的时间戳。
    ///
    /// 另见 [std::time::UNIX_EPOCH].
    pub fn get_publication_time(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        let time = jvm
            .invoke(&self.instance, "getPublicationTime", &[])
            .unwrap();
        jvm.to_rust(time).unwrap()
    }
    /// [公告发送者][NormalMember]。该成员可能已经离开群，此时返回 `None`.
    pub fn get_sender(&self) -> Option<NormalMember> {
        let jvm = Jvm::attach_thread().unwrap();
        let sender = jvm.invoke(&self.instance, "getSender", &[]).unwrap();
        if !instance_is_null(&sender) {
            Some(NormalMember::from_instance(sender))
        } else {
            None
        }
    }
    /// [公告发送者][NormalMember] id.
    pub fn get_sender_id(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        let id = jvm.invoke(&self.instance, "getSenderId", &[]).unwrap();
        jvm.to_rust(id).unwrap()
    }
    /// 获取已确认或未确认（指定 `confirmed`）的群成员。
    ///
    /// 需要处理的错误有：[`MiraiRsErrorEnum::PermissionDenied`], [`MiraiRsErrorEnum::IllegalState`].
    pub fn members(&self, confirmed: bool) -> Result<Vec<NormalMember>, MiraiRsError> {
        let jvm = Jvm::attach_thread().unwrap();
        let confirmed = InvocationArg::try_from(confirmed)
            .unwrap()
            .into_primitive()
            .unwrap();
        let members = jvm.invoke(&self.instance, "members", &[confirmed])?;
        let iter = jvm.invoke(&members, "iterator", &[]).unwrap();
        Ok(java_iter_to_rust_vec(&jvm, iter))
    }
    /// 提醒未确认的群成员。
    ///
    /// 需要处理的错误有：[`MiraiRsErrorEnum::PermissionDenied`], [`MiraiRsErrorEnum::IllegalState`].
    pub fn remind(&self) -> Result<(), MiraiRsError> {
        let jvm = Jvm::attach_thread().unwrap();
        let _ = jvm.invoke(&self.instance, "remind", &[])?;
        Ok(())
    }
}

impl AnnouncementTrait for OnlineAnnouncement {}

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
pub enum Announcement {
    OnlineAnnouncement(OnlineAnnouncement),
    OfflineAnnouncement(OfflineAnnouncement),
}

impl GetInstanceTrait for Announcement {
    fn get_instance(&self) -> Instance {
        match self {
            Announcement::OnlineAnnouncement(a) => a.get_instance(),
            Announcement::OfflineAnnouncement(a) => a.get_instance(),
        }
    }
}

impl FromInstance for Announcement {
    fn from_instance(instance: Instance) -> Self {
        if is_instance_of(
            &instance,
            "net.mamoe.mirai.contact.announcement.OnlineAnnouncement",
        ) {
            Self::OnlineAnnouncement(OnlineAnnouncement::from_instance(instance))
        } else {
            Self::OfflineAnnouncement(OfflineAnnouncement::from_instance(instance))
        }
    }
}

impl AnnouncementTrait for Announcement {}

#[derive(GetInstanceDerive, FromInstanceDerive)]
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
        let instance = jvm.invoke_static("", "getHeight", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn get_width(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke_static("", "getWidth", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn get_id(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke_static("", "getId", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn get_url(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke_static("", "getUrl", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn to_string(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke_static("", "toString", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
}

impl MessageHashCodeTrait for AnnouncementImage {}

pub struct Announcements {
    instance: Instance,
}

impl Announcements {
    pub fn as_stream(&self) -> JavaStream<OnlineAnnouncement> {
        let jvm = Jvm::attach_thread().unwrap();
        JavaStream::from_instance(jvm.invoke(&self.instance, "asStream", &[]).unwrap())
    }
    pub fn delete(&self, fid: &str) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let fid = InvocationArg::try_from(fid).unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "delete", &[fid]).unwrap())
            .unwrap()
    }
    pub fn get(&self, fid: &str) -> OnlineAnnouncement {
        let jvm = Jvm::attach_thread().unwrap();
        let fid = InvocationArg::try_from(fid).unwrap();
        OnlineAnnouncement::from_instance(jvm.invoke(&self.instance, "get", &[fid]).unwrap())
    }
    pub fn members(&self) -> Vec<NormalMember> {
        let jvm = Jvm::attach_thread().unwrap();
        let list = jvm.invoke(&self.instance, "members", &[]).unwrap();
        let iter = jvm.invoke(&list, "iterator", &[]).unwrap();
        java_iter_to_rust_vec(&jvm, iter)
    }
    pub fn publish(&self, announcement: Announcement) -> OnlineAnnouncement {
        let jvm = Jvm::attach_thread().unwrap();
        let announcement = InvocationArg::try_from(announcement.get_instance()).unwrap();
        let announcement = jvm
            .invoke(&self.instance, "publish", &[announcement])
            .unwrap();
        OnlineAnnouncement::from_instance(announcement)
    }
    pub fn remind(&self, fid: &str) {
        let jvm = Jvm::attach_thread().unwrap();
        let fid = InvocationArg::try_from(fid).unwrap();
        let _ = jvm.invoke(&self.instance, "remind", &[fid]).unwrap();
    }
    pub fn upload_image(&self, resource: &ExternalResource) -> AnnouncementImage {
        let jvm = Jvm::attach_thread().unwrap();
        let image_instance = jvm
            .invoke(
                &self.instance,
                "uploadImage",
                &[
                    InvocationArg::try_from(jvm.clone_instance(&resource.get_instance()).unwrap())
                        .unwrap(),
                ],
            )
            .unwrap();
        AnnouncementImage::from_instance(image_instance)
    }
    pub fn upload_image_from_file(&self, path: &str) -> AnnouncementImage {
        let res = ExternalResource::create_from_file(path);
        let a = self.upload_image(&res);
        res.close();
        a
    }
}

#[derive(num_enum::FromPrimitive, num_enum::IntoPrimitive, Debug)]
#[repr(i32)]
pub enum MemberPermission {
    Member = 0,
    Administrator = 1,
    #[default]
    Owner = 2,
}

impl MemberPermission {
    fn internal_to_i32(a: &MemberPermission) -> i32 {
        unsafe { *(a as *const MemberPermission as *const i32) }
    }
}

impl PartialEq for MemberPermission {
    fn eq(&self, other: &Self) -> bool {
        let a = MemberPermission::internal_to_i32(self);
        let b = MemberPermission::internal_to_i32(other);
        a.eq(&b)
    }
}

impl PartialOrd for MemberPermission {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = MemberPermission::internal_to_i32(self);
        let b = MemberPermission::internal_to_i32(other);
        a.partial_cmp(&b)
    }
}

impl Eq for MemberPermission {}

impl Ord for MemberPermission {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct ActiveRankRecord {
    instance: Instance,
    member_name: Option<String>,
    member_id: Option<i64>,
    temperature: Option<i32>,
    score: Option<i32>,
}

impl ActiveRankRecord {
    pub fn new(
        member_name: String,
        member_id: i64,
        member: NormalMember,
        temperature: i32,
        score: i32,
    ) -> ActiveRankRecord {
        let instance = Jvm::attach_thread()
            .unwrap()
            .create_instance(
                "net.mamoe.mirai.contact.active.ActiveRankRecord",
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
        }
    }
    pub fn get_member(&self) -> NormalMember {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getMember", &[]).unwrap();
        // 笔记： rust 中此类代码的行为：完全限定的方法调用。
        // 同时指定了特型和类型。
        // 如果是 `FromInstance` 的话，应该是调用了默认的实现？
        <NormalMember as FromInstance>::from_instance(instance)
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
                        .invoke(&self.instance, "getMemberId", &[])
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
                        .invoke(&self.instance, "getMemberName", &[])
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
                        .invoke(&self.instance, "getScore", &[])
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
                        .invoke(&self.instance, "getTemperature", &[])
                        .unwrap(),
                )
                .unwrap()
        }
    }
}

pub struct MiraiMap<K, V> {
    pub(crate) instance: Instance,
    pub(crate) _t: Option<HashMap<K, V>>,
}

impl<K, V> MiraiMap<K, V> {
    //顺序复制为rust HashMap.
    pub fn to_hash_map_t(
        &self,
        cast: Box<
            dyn Fn(
                &Instance,                            //key
                &Instance,                            //value
                &Jvm,                                 //jvm
                &dyn Fn(&Instance, &str) -> Instance, //java中的类型转换。
            ) -> (K, V),
        >,
    ) -> HashMap<K, V>
    where
        K: Eq + PartialEq + std::hash::Hash,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let java_cast =
            |instance: &Instance, obj: &str| -> Instance { jvm.cast(&instance, obj).unwrap() };
        let mut map = HashMap::<K, V>::new();
        let entry_set = jvm.invoke(&self.instance, "entrySet", &[]).unwrap();
        let it = jvm.invoke(&entry_set, "iterator", &[]).unwrap();
        while jvm
            .chain(&it)
            .unwrap()
            .invoke("hasNext", &[])
            .unwrap()
            .to_rust()
            .unwrap()
        {
            let entry = jvm.invoke(&it, "next", &[]).unwrap();
            let entry = java_cast(&entry, "java.util.Map$Entry");
            let k = jvm.invoke(&entry, "getKey", &[]).unwrap();
            let v = jvm.invoke(&entry, "getValue", &[]).unwrap();

            let ins = cast(&k, &v, &jvm, &java_cast);

            map.insert(ins.0, ins.1);
        }
        map
    }
}

//特化版本。
impl MiraiMap<i32, String> {
    pub fn to_hash_map(&self) -> HashMap<i32, String> {
        self.to_hash_map_t(Box::new(
            |k: &Instance,
             v: &Instance,
             jvm: &Jvm,
             cast: &dyn Fn(&Instance, &str) -> Instance|
             -> (i32, String) {
                let k: i64 = jvm.to_rust(cast(&k, "java.lang.Integer")).unwrap();
                let k: i32 = (k & i32::MAX as i64) as i32;
                let v: String = jvm.to_rust(cast(&v, "java.lang.String")).unwrap();
                (k, v)
            },
        ))
    }
}

//特化版本。
impl MiraiMap<String, i32> {
    pub fn to_hash_map(&self) -> HashMap<String, i32> {
        self.to_hash_map_t(Box::new(
            |k: &Instance,
             v: &Instance,
             jvm: &Jvm,
             cast: &dyn Fn(&Instance, &str) -> Instance|
             -> (String, i32) {
                let k: String = jvm.to_rust(cast(&k, "java.lang.String")).unwrap();
                let v: i64 = jvm.to_rust(cast(&v, "java.lang.Integer")).unwrap();
                let v: i32 = (v & i32::MAX as i64) as i32;
                (k, v)
            },
        ))
    }
}

//特化版本。该版本不应当使用。
impl MiraiMap<String, String> {
    pub fn to_hash_map(&self) -> HashMap<String, String> {
        self.to_hash_map_t(Box::new(
            |k: &Instance,
             v: &Instance,
             jvm: &Jvm,
             cast: &dyn Fn(&Instance, &str) -> Instance|
             -> (String, String) {
                let k: String = jvm
                    .to_rust(jvm.invoke(&k, "toString", &[]).unwrap())
                    .unwrap();
                let v: String = jvm.to_rust(cast(&v, "java.lang.String")).unwrap();
                (k, v)
            },
        ))
    }
}

impl<K, V> GetInstanceTrait for MiraiMap<K, V> {
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}

pub struct MiraiList<T> {
    instance: Instance,
    vec: Option<Vec<T>>,
}

impl MiraiList<ActiveRankRecord> {
    ///这个函数记得改改。
    pub fn to_vector(&self) -> &Option<Vec<ActiveRankRecord>> {
        &self.vec
    }
    ///这个函数记得改改。
    pub fn refresh_vector(&mut self) {
        let jvm = Jvm::attach_thread().unwrap();
        let it = jvm.invoke(&self.instance, "listIterator", &[]).unwrap();
        let mut vec = Vec::<ActiveRankRecord>::new();
        while jvm
            .chain(&it)
            .unwrap()
            .invoke("hasNext", &[])
            .unwrap()
            .to_rust()
            .unwrap()
        {
            let v = jvm.invoke(&it, "next", &[]).unwrap();
            vec.push(ActiveRankRecord {
                instance: v,
                member_name: None,
                member_id: None,
                temperature: None,
                score: None,
            })
        }
        self.vec = Some(vec);
    }
}

impl<T> GetInstanceTrait for MiraiList<T> {
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}

pub struct ActiveChart {
    instance: Instance,
}

pub struct ActiveHonorInfo {
    instance: Instance,
}

/// 群荣耀历史数据
pub struct ActiveHonorList {
    instance: Instance,
}

pub struct MemberMedalInfo {
    instance: Instance,
}

impl MemberMedalInfo {
    pub fn get_color(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getColor", &[]).unwrap())
            .unwrap()
    }
    pub fn get_medals(&self) -> HashSet<MemberMedalType> {
        let jvm = Jvm::attach_thread().unwrap();
        let set = jvm.invoke(&self.instance, "getHonors", &[]).unwrap();
        let iter = jvm.invoke(&set, "iterator", &[]).unwrap();
        java_iter_to_rust_hash_set(&jvm, iter)
    }
    pub fn get_title(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getTitle", &[]).unwrap())
            .unwrap()
    }
    pub fn get_wearing(&self) -> MemberMedalType {
        let jvm = Jvm::attach_thread().unwrap();
        MemberMedalType::from_instance(jvm.invoke(&self.instance, "getWearing", &[]).unwrap())
    }
}

impl FromInstance for MemberMedalInfo {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }
}

pub struct MemberActive {
    instance: Instance,
}

impl FromInstance for MemberActive {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }
}

impl MemberActive {
    pub fn get_honors(&self) -> HashSet<GroupHonorType> {
        let jvm = Jvm::attach_thread().unwrap();
        let set = jvm.invoke(&self.instance, "getHonors", &[]).unwrap();
        let iter = jvm.invoke(&set, "iterator", &[]).unwrap();
        java_iter_to_rust_hash_set(&jvm, iter)
    }
    pub fn get_point(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getPoint", &[]).unwrap())
            .unwrap()
    }
    pub fn get_rank(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getRank", &[]).unwrap())
            .unwrap()
    }
    pub fn get_temperature(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getTemperature", &[]).unwrap())
            .unwrap()
    }
    pub fn query_medal(&self) -> MemberMedalInfo {
        let jvm = Jvm::attach_thread().unwrap();
        MemberMedalInfo {
            instance: jvm.invoke(&self.instance, "queryMedal", &[]).unwrap(),
        }
    }
}

impl GroupActive {
    pub fn get_rank_titles(&self) -> MiraiMap<i32, String> {
        MiraiMap {
            instance: Jvm::attach_thread()
                .unwrap()
                .invoke(&self.instance, "getRankTitles", &[])
                .unwrap(),
            _t: None,
        }
    }
    pub fn get_temperature_titles(&self) -> MiraiMap<i32, String> {
        MiraiMap {
            instance: Jvm::attach_thread()
                .unwrap()
                .invoke(&self.instance, "getTemperatureTitles", &[])
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
                    .invoke(&self.instance, "isHonorVisible", &[])
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
                    .invoke(&self.instance, "isTemperatureVisible", &[])
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
                    .invoke(&self.instance, "isTitleVisible", &[])
                    .unwrap(),
            )
            .unwrap()
    }
    pub fn query_active_rank(&self) -> MiraiList<ActiveRankRecord> {
        MiraiList {
            instance: Jvm::attach_thread()
                .unwrap()
                .invoke(&self.instance, "queryActiveRank", &[])
                .unwrap(),
            vec: None,
        }
    }
    pub fn query_chart(&self) -> ActiveChart {
        ActiveChart {
            instance: Jvm::attach_thread()
                .unwrap()
                .invoke(&self.instance, "queryChart", &[])
                .unwrap(),
        }
    }
    // need to test
    pub fn query_honor_history(&self) -> ActiveHonorList {
        let jvm = Jvm::attach_thread().unwrap();
        ActiveHonorList {
            instance: jvm
                .cast(
                    &jvm.invoke(&self.instance, "queryHonorHistory", &[])
                        .unwrap(),
                    "ActiveHonorList",
                )
                .unwrap(),
        }
    }
    pub fn refresh(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "refresh", &[])
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
                &[InvocationArg::try_from(mirai_map_instance)
                    .unwrap()
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
                &[InvocationArg::try_from(mirai_map_instance)
                    .unwrap()
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

impl FromInstance for Group {
    fn from_instance(instance: Instance) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let bot = jvm.invoke(&instance, "getBot", &[]).unwrap();
        let id = jvm
            .to_rust(jvm.invoke(&instance, "getId", &[]).unwrap())
            .unwrap();
        Group { bot, instance, id }
    }
}

impl ContactOrBotTrait for Group {
    fn get_id(&self) -> i64 {
        self.id
    }
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

impl ContactTrait for Group {}

impl Group {
    pub fn new(bot: &Bot, id: i64) -> Option<Group> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &bot.get_instance(),
                "getGroup",
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        if !instance_is_null(&instance) {
            Some(Group {
                bot: bot.get_instance(),
                instance,
                id,
            })
        } else {
            None
        }
    }
    pub fn contains_member(&self, member: &NormalMember) -> bool {
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
    pub fn get(self, id: i64) -> Option<NormalMember> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "get",
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        if !instance_is_null(&instance) {
            Some(NormalMember {
                bot: self.bot,
                instance,
                id,
            })
        } else {
            None
        }
    }
    pub fn get_active(&self) -> GroupActive {
        let active_instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getActive", &[])
            .unwrap();
        GroupActive {
            instance: active_instance,
        }
    }
    pub fn get_announcements(&self) -> Announcements {
        Announcements {
            instance: Jvm::attach_thread()
                .unwrap()
                .invoke(&self.instance, "getAnnouncements", &[])
                .unwrap(),
        }
    }
    pub fn get_bot_as_member(self) -> NormalMember {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getBotAsMember", &[]).unwrap();
        let id = Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.bot, "getId", &[])
                    .unwrap(),
            )
            .unwrap();
        NormalMember {
            bot: self.bot,
            instance,
            id,
        }
    }
    pub fn get_bot_mute_remaining(&self) -> i32 {
        Jvm::attach_thread()
            .unwrap()
            .chain(&self.instance)
            .unwrap()
            .invoke("getBotMuteRemaining", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    pub fn get_bot_permission(&self) -> MemberPermission {
        let jvm = Jvm::attach_thread().unwrap();
        let prem = jvm
            .invoke(&self.instance, "getMemberPermission", &[])
            .unwrap();
        let prem = jvm.invoke(&prem, "getLevel", &[]).unwrap();
        MemberPermission::from(jvm.to_rust::<i32>(prem).unwrap())
    }
    pub fn get_members(self) -> ContactList<NormalMember> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getMembers", &[])
            .unwrap();
        ContactList {
            instance,
            _unused: PhantomData::default(),
        }
    }
    pub fn get_name(&self) -> String {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "getName", &[])
                    .unwrap(),
            )
            .unwrap()
    }
    pub fn get_owner(self) -> NormalMember {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getSize", &[])
            .unwrap();
        let id = ContactOrBotTrait::get_id(&self);
        NormalMember {
            bot: self.bot,
            instance,
            id,
        }
    }
    pub fn get_settings(&self) -> GroupSettings {
        GroupSettings {
            instance: Jvm::attach_thread()
                .unwrap()
                .invoke(&self.instance, "getSettings", &[])
                .unwrap(),
        }
    }
    pub fn quit(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "quit", &[]).unwrap())
            .unwrap()
    }
    pub fn set_essence_message(&self, source: MessageSource) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(
                &self.instance,
                "setEssenceMessage",
                &[InvocationArg::try_from(source.get_instance()).unwrap()],
            )
            .unwrap(),
        )
        .unwrap()
    }
    // function name need to be changed.
    pub fn set_essence_message_s(group: Group, chain: MessageChain) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(
                &group.get_instance(),
                "setEssenceMessage",
                &[
                    InvocationArg::try_from(group.get_instance()).unwrap(),
                    InvocationArg::try_from(chain.get_instance()).unwrap(),
                ],
            )
            .unwrap(),
        )
        .unwrap()
    }
    pub fn set_name(&self, name: &str) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setName",
                &[InvocationArg::try_from(name).unwrap()],
            )
            .unwrap();
    }
    // TODO: 获取精华消息。
}
