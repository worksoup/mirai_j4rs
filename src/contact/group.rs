use crate::{
    contact::{
        bot::Bot,
        contact_trait::{ContactOrBotTrait, ContactTrait, FileSupportedTrait},
        ContactList, NormalMember,
    },
    env::{FromInstance, GetEnvTrait},
    message::{MessageChain, MessageSource},
    utils::{internal::instance_is_null, other::enums::AvatarSpec},
};
use contact_derive::GetInstanceDerive;
use j4rs::{Instance, InvocationArg, Jvm};
use std::{cmp::Ordering, collections::HashMap, marker::PhantomData};

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

impl FileSupportedTrait for Group {}

pub struct GroupActive {
    instance: Instance,
}

pub struct Announcements {
    instance: Instance,
}

impl Announcements {
    // TODO
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
    fn internal_clone_into_i32(a: &MemberPermission) -> i32 {
        match a {
            MemberPermission::Member => 0,
            MemberPermission::Administrator => 1,
            MemberPermission::Owner => 2,
        }
    }
}

impl PartialEq for MemberPermission {
    fn eq(&self, other: &Self) -> bool {
        let a = MemberPermission::internal_clone_into_i32(self);
        let b = MemberPermission::internal_clone_into_i32(other);
        a.eq(&b)
    }
}

impl PartialOrd for MemberPermission {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = MemberPermission::internal_clone_into_i32(self);
        let b = MemberPermission::internal_clone_into_i32(other);
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

impl<K, V> GetEnvTrait for MiraiMap<K, V> {
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
        let mut vec = Vec::<ActiveRankRecord>::new();
        let it = jvm.invoke(&self.instance, "listIterator", &[]).unwrap();
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

impl<T> GetEnvTrait for MiraiList<T> {
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

pub struct MemberMedalType {
    instance: Instance,
}

pub struct MemberActive {
    instance: Instance,
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
        if instance_is_null(&instance) {
            None
        } else {
            Some(Group {
                bot: bot.get_instance(),
                instance,
                id,
            })
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
        if instance_is_null(&instance) {
            None
        } else {
            Some(NormalMember {
                bot: self.bot,
                instance,
                id,
            })
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
