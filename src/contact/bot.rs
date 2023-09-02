use super::{
    contact_trait::{ContactOrBotTrait, UserOrBotTrait},
    group::Group,
    ContactList, Friend, OtherClient, Stranger,
};
use crate::{
    contact::{bot, group::MiraiMap},
    env::{GetBotTrait, GetEnvTrait},
    event::{EventChannel, MiraiEventTrait},
    other::{
        enums::{self, AvatarSpec, HeartbeatStrategy, MiraiProtocol},
        tools,
    },
};
use const_unit_poc::values::{h, ms};
use core::str;
use j4rs::{ClasspathEntry, Instance, InvocationArg, JavaOpt, Jvm, JvmBuilder};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub struct Bot {
    pub(crate) bot: Instance,
    pub(crate) id: i64,
}
impl GetEnvTrait for Bot {
    fn get_instance(&self) -> j4rs::Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.bot)
            .unwrap()
    }
}
impl GetBotTrait for Bot {
    fn get_bot(&self) -> bot::Bot {
        Bot {
            bot: Jvm::attach_thread()
                .unwrap()
                .clone_instance(&self.bot)
                .unwrap(),
            id: self.id,
        }
    }
}
impl<'a> Bot {
    pub fn close(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.bot, "close", &[])
            .unwrap();
    }
    pub fn close_and_join(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.bot, "closeAndJoin", &[])
            .unwrap();
        todo!()
    }
    pub fn get_as_friend(&self) -> Friend {
        let id = self.id;
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.bot, "getAsFriend", &[])
            .unwrap();
        Friend {
            bot: self.get_instance(),
            instance,
            id,
        }
    }
    pub fn get_as_stranger(&self) -> Stranger {
        let id = self.id;
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.bot, "getAsStranger", &[])
            .unwrap();
        Stranger {
            bot: self.get_instance(),
            instance,
            id,
        }
    }
    pub fn get_configuration(&self) -> BotConfiguration {
        let bot_configuration = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.bot, "getConfiguration", &[])
            .unwrap();
        BotConfiguration {
            instance: bot_configuration,
        }
    }
    pub fn get_event_channel<E>(&self) -> EventChannel<E>
    where
        E: MiraiEventTrait,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.bot, "getEventChannel", &[])
            .unwrap();
        let _unused = None;
        EventChannel {
            jvm,
            instance,
            _unused,
        }
    }
    pub fn get_friend(&self, id: i64) -> Option<Friend> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.bot,
                "getFriend",
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        if Env::instance_is_null(&instance) {
            None
        } else {
            Some(Friend {
                bot: self.get_instance(),
                instance,
                id,
            })
        }
    }
    pub fn get_friend_groups(&self) {
        todo!()
    }
    pub fn get_friends(&self) -> ContactList<Friend> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.bot, "getFriends", &[])
            .unwrap();
        ContactList {
            bot: self.get_instance(),
            instance,
            _unused: None,
        }
    }
    pub fn get_group(&self, id: i64) -> Option<Group> {
        Group::new(self, id)
    }
    pub fn get_groups(&self) -> ContactList<Group> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.bot, "getGroups", &[])
            .unwrap();
        ContactList {
            bot: self.get_instance(),
            instance,
            _unused: None,
        }
    }
    pub fn get_logger() -> MiraiLogger {
        todo!()
    }
    pub fn get_other_clients(&self) -> ContactList<OtherClient> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.bot, "getOtherClients", &[])
            .unwrap();
        ContactList {
            bot: self.get_instance(),
            instance,
            _unused: None,
        }
    }
    pub fn get_stranger(&self, id: i64) -> Option<Stranger> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.bot,
                "getStranger",
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        if Env::instance_is_null(&instance) {
            None
        } else {
            Some(Stranger {
                bot: self.get_instance(),
                instance,
                id,
            })
        }
    }
    pub fn get_strangers(&self) -> ContactList<Stranger> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.bot, "getStrangers", &[])
            .unwrap();
        ContactList {
            bot: self.get_instance(),
            instance,
            _unused: None,
        }
    }
    pub fn is_online(&self) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .chain(&self.bot)
            .unwrap()
            .invoke("isOnline", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    pub fn join(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.bot, "join", &[])
            .unwrap();
    }
    pub fn login(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.bot, "login", &Vec::new())
            .unwrap();
    }
    pub fn nudge() {
        todo!()
    }
}
impl ContactOrBotTrait for Bot {
    fn get_id(&self) -> i64 {
        self.id
    }
    fn get_avatar_url(&self, size: Option<crate::other::enums::AvatarSpec>) -> String {
        let size: i32 = if let Some(size) = size {
            size.into()
        } else {
            AvatarSpec::XL.into()
        };
        return format!(r"http://q.qlogo.cn/g?b=qq&nk={}&s={}", self.get_id(), size,);
    }
}
impl UserOrBotTrait for Bot {}
pub struct MiraiLogger(Instance);
pub struct DeviceInfo(Instance);
pub struct LoginSolver(Instance);
pub struct ContactListCache {
    instance: Instance,
}
impl ContactListCache {
    pub fn get_save_interval_millis(&self) -> u64 {
        return Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "getSaveIntervalMillis", &[])
                    .unwrap(),
            )
            .unwrap();
    }
    pub fn set_save_interval_millis(&self, millis: u64) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setSaveIntervalMillis",
                &[InvocationArg::try_from(millis as i64)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
    }
    pub fn get_friend_list_cache_enabled(&self) -> bool {
        return Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "getFriendListCacheEnabled", &[])
                    .unwrap(),
            )
            .unwrap();
    }
    pub fn set_friend_list_cache_enabled(&self, bool: bool) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setFriendListCacheEnabled",
                &[InvocationArg::try_from(bool)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
    }
    pub fn get_group_member_list_cache_enabled(&self) -> bool {
        return Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "getGroupMemberListCacheEnabled", &[])
                    .unwrap(),
            )
            .unwrap();
    }
    pub fn set_group_member_list_cache_enabled(&self, bool: bool) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setGroupMemberListCacheEnabled",
                &[InvocationArg::try_from(bool)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
    }
}
pub struct Env {
    jvm: Jvm,
    instance: Instance,
}
impl Env {
    pub fn new(jar_paths: &Vec<String>, java_opts: &Vec<String>) -> Self {
        let entries = {
            let mut entries = Vec::new();
            for path in jar_paths {
                let entry = ClasspathEntry::new(path);
                entries.push(entry);
            }
            entries
        };
        let opts = {
            let mut opts = Vec::new();
            for opt in java_opts {
                let opt = JavaOpt::new(opt.as_str());
                opts.push(opt);
            }
            opts
        };
        let jvm = JvmBuilder::new()
            .classpath_entries(entries)
            .java_opts(opts)
            .build()
            .unwrap();
        let instance = jvm
            .field(
                &jvm.static_class("net.mamoe.mirai.BotFactory").unwrap(),
                "INSTANCE",
            )
            .unwrap();
        Self { jvm, instance }
    }
    pub fn fix_protocol_version_fetch(
        &self,
        protocol: enums::MiraiProtocol,
        version: String,
    ) -> () {
        println!("fix protocol version - tmp - fetch");
        let _ = self.jvm.invoke_static(
            "xyz.cssxsh.mirai.tool.FixProtocolVersion",
            "fetch",
            &[
                tools::protocol_enum_r2j(protocol).unwrap(),
                InvocationArg::try_from(version).unwrap(),
            ],
        );
    }
    pub fn fix_protocol_version_load(&self, protocol: enums::MiraiProtocol) -> () {
        println!("fix protocol version - tmp - load");
        let _ = self.jvm.invoke_static(
            "xyz.cssxsh.mirai.tool.FixProtocolVersion",
            "load",
            &[tools::protocol_enum_r2j(protocol).unwrap()],
        );
    }
    pub fn fix_protocol_version_info(&self) -> HashMap<String, String> {
        println!("fix protocol version - tmp - info");
        let map: MiraiMap<String, String> = MiraiMap {
            instance: Jvm::attach_thread()
                .unwrap()
                .invoke_static("xyz.cssxsh.mirai.tool.FixProtocolVersion", "info", &[])
                .unwrap(),
            _t: None,
        };
        map.to_hash_map()
    }
    pub fn fix_protocol_version_update(&self) -> () {
        println!("fix protocol version - tmp - update");
        let _ = self
            .jvm
            .invoke_static("xyz.cssxsh.mirai.tool.FixProtocolVersion", "update", &[]);
    }
    pub fn new_bot_configuration(&self) -> BotConfiguration {
        let bot_configuration = Jvm::attach_thread()
            .unwrap()
            .invoke_static("net.mamoe.mirai.utils.BotConfiguration", "getDefault", &[])
            .unwrap();
        BotConfiguration {
            instance: bot_configuration,
        }
    }
    pub(crate) fn instance_is_null(instance: &Instance) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke_static(
                        "java.util.Objects",
                        "isNull",
                        &[InvocationArg::try_from(
                            Jvm::attach_thread()
                                .unwrap()
                                .clone_instance(instance)
                                .unwrap(),
                        )
                        .unwrap()],
                    )
                    .unwrap(),
            )
            .unwrap()
    }
    pub fn find_bot(&self, id: i64) -> Option<Bot> {
        let bot = self
            .jvm
            .invoke_static(
                "net.mamoe.mirai.Bot$Companion",
                "findInstance",
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        if Env::instance_is_null(&bot) {
            None
        } else {
            Some(Bot { bot, id })
        }
    }
    pub fn get_bots(&self) -> ContactList<Friend> /*should be 'MiraiList<Bot>' which has not been implemented yet*/
    {
        todo!()
    }
    //默认是global的。
    pub fn event_channel<E>(&self) -> EventChannel<E>
    where
        E: MiraiEventTrait,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = Jvm::attach_thread()
        .unwrap()
        .static_class("net.mamoe.mirai.event.GlobalEventChannel$INSTANCE")
        .unwrap();
        let _unused = None;
        EventChannel {
            jvm,
            instance,
            _unused,
        }
    }
}
pub trait Certificate<T> {
    fn new_bot(&self, id: i64, password: T, bot_configuration: Option<BotConfiguration>) -> Bot;
}
impl Certificate<String> for Env {
    fn new_bot(
        &self,
        id: i64,
        password: String,
        bot_configuration: Option<BotConfiguration>,
    ) -> Bot {
        let bot = if let Some(bot_config) = bot_configuration {
            Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self.instance,
                    "newBot",
                    &[
                        InvocationArg::try_from(id)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                        InvocationArg::try_from(password).unwrap(),
                        InvocationArg::try_from(bot_config.get_instance()).unwrap(),
                    ],
                )
                .unwrap()
        } else {
            Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self.instance,
                    "newBot",
                    &vec![
                        InvocationArg::try_from(id)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                        InvocationArg::try_from(password).unwrap(),
                    ],
                )
                .unwrap()
        };
        Bot { bot, id }
    }
}
impl Certificate<[u8; 16]> for Env {
    fn new_bot(
        &self,
        id: i64,
        password: [u8; 16],
        bot_configuration: Option<BotConfiguration>,
    ) -> Bot {
        let mut password_md5 = Vec::new();
        for i in password {
            password_md5.push(
                InvocationArg::try_from(i.clone() as i8)
                    .unwrap()
                    .into_primitive()
                    .unwrap(),
            );
        }
        let bot = if let Some(bot_configuration) = bot_configuration {
            Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self
                        .jvm
                        .invoke(
                            &self
                                .jvm
                                .invoke_static("net.mamoe.mirai.Mirai", "getInstance", &[])
                                .unwrap(),
                            "getBotFactory",
                            &[],
                        )
                        .unwrap(),
                    "newBot",
                    &[
                        InvocationArg::try_from(id)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                        InvocationArg::try_from(
                            Jvm::attach_thread()
                                .unwrap()
                                .create_java_array("byte", &password_md5)
                                .unwrap(),
                        )
                        .unwrap(),
                        InvocationArg::try_from(bot_configuration.get_instance()).unwrap(),
                    ],
                )
                .unwrap()
        } else {
            Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self.instance,
                    "newBot",
                    &vec![
                        InvocationArg::try_from(id)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                        InvocationArg::try_from(
                            Jvm::attach_thread()
                                .unwrap()
                                .create_java_array("byte", &password_md5)
                                .unwrap(),
                        )
                        .unwrap(),
                    ],
                )
                .unwrap()
        };
        Bot { bot, id }
    }
}
pub struct BotConfiguration {
    instance: Instance,
}
impl crate::env::GetEnvTrait for BotConfiguration {
    fn get_instance(&self) -> j4rs::Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}
// builders
impl BotConfiguration {
    pub fn copy_configuration_from(bot: &Bot) -> Self {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&bot.get_configuration().get_instance(), "copy", &[])
            .unwrap();
        BotConfiguration { instance }
    }
    pub fn get_default() -> Self {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke_static("net.mamoe.mirai.utils.BotConfiguration", "getDefault", &[])
            .unwrap();
        BotConfiguration { instance }
    }
}
// getters
impl BotConfiguration {
    pub fn get_auto_reconnect_on_force_offline(&self) -> bool {
        return Jvm::attach_thread()
            .unwrap()
            .chain(&self.instance)
            .unwrap()
            .invoke("getAutoReconnectOnForceOffline", &[])
            .unwrap()
            .to_rust()
            .unwrap();
    }
    pub fn get_bot_logger_supplier(&self) -> Box<dyn Fn(&Bot) -> MiraiLogger + '_> {
        let bot_logger_supplier = |bot: &Bot| -> MiraiLogger {
            let tmp = Jvm::attach_thread()
                .unwrap()
                .invoke(&self.instance, "getBotLoggerSupplier", &[])
                .unwrap();
            MiraiLogger(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(
                        &tmp,
                        "invoke",
                        &[InvocationArg::try_from(
                            Jvm::attach_thread()
                                .unwrap()
                                .clone_instance(&bot.bot)
                                .unwrap(),
                        )
                        .unwrap()],
                    )
                    .unwrap(),
            )
        };
        return Box::new(bot_logger_supplier);
    }
    pub fn get_cache_dir(&self) -> PathBuf {
        let i: String = Jvm::attach_thread()
            .unwrap()
            .chain(&self.instance)
            .unwrap()
            .invoke("getCacheDir", &[])
            .unwrap()
            .invoke("toString", &[])
            .unwrap()
            .to_rust()
            .unwrap();
        return PathBuf::from(i);
    }
    pub fn get_contact_list_cache(&self) -> Option<ContactListCache> {
        return Some(ContactListCache {
            instance: Jvm::attach_thread()
                .unwrap()
                .invoke(&self.instance, "getContactListCache", &[])
                .unwrap(),
        });
    }
    pub fn get_device_info(&self) -> Option<impl Fn(Bot) -> DeviceInfo + '_> {
        let tmp = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getDeviceInfo", &[])
            .unwrap();
        let bot_logger_supplier = move |bot: Bot| -> DeviceInfo {
            DeviceInfo(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(
                        &tmp,
                        "invoke",
                        &[InvocationArg::try_from(
                            Jvm::attach_thread()
                                .unwrap()
                                .clone_instance(&bot.bot)
                                .unwrap(),
                        )
                        .unwrap()],
                    )
                    .unwrap(),
            )
        };
        return Some(bot_logger_supplier);
    }
    pub fn get_heartbeat_period_millis(&self) -> u64 {
        return Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "getHeartbeatPeriodMillis", &[])
                    .unwrap(),
            )
            .unwrap();
    }
    pub fn get_heartbeat_strategy(&self) -> HeartbeatStrategy {
        let hbs: String = Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "getHeartbeatStrategy", &[])
                    .unwrap(),
            )
            .unwrap();
        if hbs == "STAT_HB" {
            return HeartbeatStrategy::S;
        }
        if hbs == "REGISTER" {
            return HeartbeatStrategy::R;
        }
        if hbs == "NONE" {
            return HeartbeatStrategy::N;
        }
        println!("&self.instance is None!");
        return HeartbeatStrategy::S;
    }
    pub fn get_heartbeat_timeout_millis(&self) -> u64 {
        return Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "getHeartbeatTimeoutMillis", &[])
                    .unwrap(),
            )
            .unwrap();
    }
    pub fn get_highway_upload_coroutine_count(&self) -> usize {
        return Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "getHighwayUploadCoroutineCount", &[])
                    .unwrap(),
            )
            .unwrap();
    }
    pub fn get_login_cache_enabled(&self) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .chain(&self.instance)
            .unwrap()
            .invoke("getLoginCacheEnabled", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    pub fn get_login_solver(&self) -> Option<LoginSolver> {
        return Some(LoginSolver(
            Jvm::attach_thread()
                .unwrap()
                .invoke(&self.instance, "getLoginSolver", &[])
                .unwrap(),
        ));
    }
    pub fn get_network_logger_supplier(&self) -> Option<Instance> {
        return Some(
            Jvm::attach_thread()
                .unwrap()
                .invoke(&self.instance, "getNetworkLoggerSupplier", &[])
                .unwrap(),
        );
    }
    pub fn get_parent_coroutine_context(&self) -> Option<Instance> {
        return Some(
            Jvm::attach_thread()
                .unwrap()
                .invoke(&self.instance, "getParentCoroutineContext", &[])
                .unwrap(),
        );
    }
    pub fn get_protocol(&self) -> MiraiProtocol {
        let mp: String = Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "getProtocol", &[])
                    .unwrap(),
            )
            .unwrap();
        tools::protocol_str2enum(mp)
    }
    pub fn get_reconnection_retry_times(&self) -> i32 {
        return Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "getReconnectionRetryTimes", &[])
                    .unwrap(),
            )
            .unwrap();
    }
    pub fn get_stat_heartbeat_period_millis(&self) -> i64 {
        return Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "getStatHeartbeatPeriodMillis", &[])
                    .unwrap(),
            )
            .unwrap();
    }
    pub fn get_working_dir(&self) -> PathBuf {
        let i: String = Jvm::attach_thread()
            .unwrap()
            .chain(&self.instance)
            .unwrap()
            .invoke("getWorkingDir", &[])
            .unwrap()
            .invoke("toString", &[])
            .unwrap()
            .to_rust()
            .unwrap();
        return PathBuf::from(i);
    }
}
/// askers
impl BotConfiguration {
    pub fn is_convert_line_separator(&self) -> bool {
        return Jvm::attach_thread()
            .unwrap()
            .chain(&self.instance)
            .unwrap()
            .invoke("isConvertLineSeparator", &[])
            .unwrap()
            .to_rust()
            .unwrap();
    }
    pub fn is_showing_verbose_event_log(&self) -> bool {
        return Jvm::attach_thread()
            .unwrap()
            .chain(&self.instance)
            .unwrap()
            .invoke("isShowingVerboseEventLog", &[])
            .unwrap()
            .to_rust()
            .unwrap();
    }
}
/// setters
impl BotConfiguration {
    pub fn set_auto_reconnect_on_force_offline(&self, yes: bool) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setAutoReconnectOnForceOffline",
                &[InvocationArg::try_from(yes)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
    }
    pub fn set_bot_logger_supplier(&self) {
        todo!();
    }
    pub fn set_cache_dir(&self, path: &PathBuf) {
        let path = path.to_str().unwrap();
        let file = Jvm::attach_thread()
            .unwrap()
            .create_instance("java.io.File", &[InvocationArg::try_from(path).unwrap()])
            .unwrap();
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setCacheDir",
                &[InvocationArg::try_from(file).unwrap()],
            )
            .unwrap();
    }
    pub fn set_contact_list_cache(&self, cache: ContactListCache) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setContactListCache",
                &[InvocationArg::try_from(cache.instance).unwrap()],
            )
            .unwrap();
    }
    pub fn set_convert_line_separator(&self, yes: bool) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setConvertLineSeparator",
                &[InvocationArg::try_from(yes)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
    }
    pub fn set_device_info(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "setDeviceInfo", &[])
            .unwrap();
        todo!("");
    }
    pub fn set_heartbeat_period_millis(&self, millis: i64) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setHeartbeatPeriodMillis",
                &[InvocationArg::try_from(millis)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
    }
    pub fn set_heartbeat_strategy(&self, heartbeat_strategy: HeartbeatStrategy) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setHeartbeatStrategy",
                &[InvocationArg::try_from(
                    Jvm::attach_thread()
                        .unwrap()
                        .field(
                            &Jvm::attach_thread()
                                .unwrap()
                                .static_class(
                                    "net.mamoe.mirai.utils.BotConfiguration$HeartbeatStrategy",
                                )
                                .unwrap(),
                            match heartbeat_strategy {
                                HeartbeatStrategy::S => "STAT_HB",
                                HeartbeatStrategy::R => "REGISTER",
                                HeartbeatStrategy::N => "NONE",
                            },
                        )
                        .unwrap(),
                )
                .unwrap()],
            )
            .unwrap();
    }
    pub fn set_heartbeat_timeout_millis(&self, millis: i64) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setHeartbeatTimeoutMillis",
                &[InvocationArg::try_from(millis)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
    }
    pub fn set_highway_upload_coroutine_count(&self, count: i32) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setHighwayUploadCoroutineCount",
                &[InvocationArg::try_from(count)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
    }
    pub fn set_login_cache_enabled(&self, yes: bool) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setLoginCacheEnabled",
                &[InvocationArg::try_from(yes)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
    }
    pub fn set_login_solver(&self, login_solver: LoginSolver) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setLoginSolver",
                &[InvocationArg::try_from(login_solver.0).unwrap()],
            )
            .unwrap();
    }
    pub fn set_network_logger_supplier(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "setNetworkLoggerSupplier", &[])
            .unwrap();
        todo!();
    }
    pub fn set_parent_coroutine_context(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "setParentCoroutineContext", &[])
            .unwrap();
        todo!();
    }
    pub fn set_protocol(&self, protocol: MiraiProtocol) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setProtocol",
                &[tools::protocol_enum_r2j(protocol).unwrap()],
            )
            .unwrap();
    }
    pub fn set_reconnection_retry_times(&self, times: i32) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setReconnectionRetryTimes",
                &[InvocationArg::try_from(times)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
    }
    pub fn set_showing_verbose_event_log(&self, yes: bool) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setShowingVerboseEventLog",
                &[InvocationArg::try_from(yes)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
    }
    pub fn set_stat_heartbeat_period_millis(&self, millis: i64) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setStatHeartbeatPeriodMillis",
                &[InvocationArg::try_from(millis)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
    }
    pub fn set_working_dir(&self, path: &PathBuf) {
        let path = path.to_str().unwrap();
        let file = Jvm::attach_thread()
            .unwrap()
            .create_instance("java.io.File", &[InvocationArg::try_from(path).unwrap()])
            .unwrap();
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setWorkingDir",
                &[InvocationArg::try_from(file).unwrap()],
            )
            .unwrap();
    }
}
impl BotConfiguration {
    /// 在被挤下线时自动重连。
    pub fn auto_reconnect_on_force_offline(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "autoReconnectOnForceOffline", &[])
            .unwrap();
    }
    /// 禁止保存 account.secrets.
    /// 根据 Mirai 注释：
    ///     禁止保存 account.secrets.
    ///     account.secrets 保存账号的会话信息。
    ///     它可加速登录过程，也可能可以减少出现验证码的次数。
    ///     如果遇到一段时间后无法接收消息通知等同步问题时可尝试禁用。
    pub fn disable_account_secretes(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "disableAccountSecretes", &[])
            .unwrap();
    }
    /// 禁用好友列表和群成员列表的缓存。
    /// ~应该~**不**是默认行为。
    pub fn disable_contact_cache(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "disableContactCache", &[])
            .unwrap();
    }
    /// 启用好友列表和群成员列表的缓存。
    /// ~应该~是默认行为。
    pub fn enable_contact_cache(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "enableContactCache", &[])
            .unwrap();
    }
    /// 使用文件存储设备信息。
    /// 可以传入 None 参数，此时默认使用工作目录下的 device.json 文件。
    /// 在传入 None 参数的情况下，如果 device.json 文件不存在的话，
    /// Mirai 似乎会发出警告，然后随机生成一个设备信息。
    /// TODO: 测试传入参数的行为。
    pub fn file_based_device_info(&self, path: Option<&PathBuf>) {
        if let Some(path) = path {
            Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self.instance,
                    "fileBasedDeviceInfo",
                    &[InvocationArg::try_from(path.to_str().unwrap()).unwrap()],
                )
                .unwrap();
        } else {
            Jvm::attach_thread()
                .unwrap()
                .invoke(&self.instance, "fileBasedDeviceInfo", &[])
                .unwrap();
        }
    }
    /// 不显示 Bot 日志。不推荐。
    pub fn no_bot_log(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "noBotLog", &[])
            .unwrap();
    }
    /// 不显示网络日志。不推荐。
    pub fn no_network_log(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "noNetworkLog", &[])
            .unwrap();
    }
    /// 使用随机设备信息。
    /// 注意该函数~应该~不会持久化当前随机信息。
    pub fn random_device_info(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "randomDeviceInfo", &[])
            .unwrap();
    }
    fn redirect_log_to_directory(
        &self,
        path: Option<&PathBuf>,
        retain: Option<i64>,
        indentity: Option<Instance>,
        arg: &str,
        method_name: &str,
    ) {
        let path = if path.is_none() {
            Jvm::attach_thread()
                .unwrap()
                .create_instance("java.io.File", &[InvocationArg::try_from(arg).unwrap()])
                .unwrap()
        } else {
            Jvm::attach_thread()
                .unwrap()
                .create_instance(
                    "java.io.File",
                    &[InvocationArg::try_from(path.unwrap().to_str().unwrap()).unwrap()],
                )
                .unwrap()
        };
        let retain = InvocationArg::try_from(if retain.is_none() {
            (7.0 * 24.0 * h / ms).raw_value as i64
        } else {
            retain.unwrap()
        })
        .unwrap()
        .into_primitive()
        .unwrap();
        if indentity.is_none() {
            Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self.instance,
                    method_name,
                    &[InvocationArg::try_from(path).unwrap(), retain],
                )
                .unwrap();
        } else {
            Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self.instance,
                    method_name,
                    &[
                        InvocationArg::try_from(path).unwrap(),
                        InvocationArg::try_from(retain).unwrap(),
                        InvocationArg::try_from(indentity.unwrap()).unwrap(),
                    ],
                )
                .unwrap();
        };
    }
    fn redirect_log_to_file(
        &self,
        path: Option<&PathBuf>,
        indentity: Option<Instance>,
        arg: &str,
        method_name: &str,
    ) {
        let path = if path.is_none() {
            Jvm::attach_thread()
                .unwrap()
                .create_instance("java.io.File", &[InvocationArg::try_from(arg).unwrap()])
                .unwrap()
        } else {
            Jvm::attach_thread()
                .unwrap()
                .create_instance(
                    "java.io.File",
                    &[InvocationArg::try_from(path.unwrap().to_str().unwrap()).unwrap()],
                )
                .unwrap()
        };
        if indentity.is_none() {
            Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self.instance,
                    method_name,
                    &[InvocationArg::try_from(path).unwrap()],
                )
                .unwrap();
        } else {
            Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self.instance,
                    method_name,
                    &[
                        InvocationArg::try_from(path).unwrap(),
                        InvocationArg::try_from(indentity.unwrap()).unwrap(),
                    ],
                )
                .unwrap();
        };
    }
    /// 重定向 Bot 日志到指定目录。若目录不存在将会自动创建。
    pub fn redirect_bot_log_to_directory(
        &self,
        path: Option<&PathBuf>,
        retain: Option<i64>,
        indentity: Option<Instance>,
    ) {
        self.redirect_log_to_directory(
            path,
            retain,
            indentity,
            "logs",
            "redirectBotLogToDirectory",
        );
    }
    /// 重定向 Bot 日志到指定文件。日志将会逐行追加到此文件。若文件不存在将会自动创建。
    pub fn redirect_bot_log_to_file(&self, path: Option<&PathBuf>, indentity: Option<Instance>) {
        self.redirect_log_to_file(path, indentity, "mirai.log", "redirectBotLogToFile");
    }
    /// 重定向网络日志到指定目录。若目录不存在将会自动创建。
    /// 默认目录路径为 `$workingDir/logs/`.
    pub fn redirect_network_log_to_directory(
        &self,
        path: Option<&PathBuf>,
        retain: Option<i64>,
        indentity: Option<Instance>,
    ) {
        self.redirect_log_to_directory(
            path,
            retain,
            indentity,
            "logs",
            "redirectNetworkLogToDirectory",
        );
    }
    /// 重定向网络日志到指定文件。默认文件路径为 `$workingDir/mirai.log`.
    /// 日志将会逐行追加到此文件。若文件不存在将会自动创建。
    pub fn redirect_network_log_to_file(
        &self,
        path: Option<&PathBuf>,
        indentity: Option<Instance>,
    ) {
        self.redirect_log_to_file(path, indentity, "mirai.log", "redirectNetworkLogToFile");
    }
    // 用来复制的模板。
    // 暂时无用。
    // pub fn noNetworkLog(&self) {
    //     if let Some(bot_configuration) = &self.bot_configuration {
    //          Jvm::attach_thread().unwrap()
    //             .invoke(bot_configuration, "disableAccountSecretes", &[])
    //             .unwrap();
    //     }
    // }
}
/// bot builder
pub struct BotBuilder {
    pub env: Env,
    pub config: Option<BotConfiguration>,
    id: Option<i64>,
    password: Option<String>,
    password_md5: Option<[u8; 16]>,
}
#[derive(Deserialize, Serialize)]
struct BaseConfig {
    config_file: String,
}
#[derive(Deserialize, Serialize)]
struct EnvConfig {
    jar_paths: Vec<String>,
    java_opts: Vec<String>,
}
impl BotBuilder {
    fn internal_new_env() -> Env {
        let default_base_config = BaseConfig {
            config_file: "env_config.toml".to_string(),
        };
        let proj_dir = directories::ProjectDirs::from("rt", "lea", "mirai_j4rs");
        let dir = if let Some(proj_dir) = &proj_dir {
            proj_dir.config_dir()
        } else {
            Path::new(".")
        };
        if let Err(_) = std::fs::metadata(dir) {
            let _ = std::fs::create_dir_all(dir);
        }
        let mut dir_tmp = dir.to_path_buf();
        dir_tmp.push("base_config.toml");
        // 如果 `base_config.toml` 不存在则创建一个默认的。
        if let Ok(base_caonfig_file) = std::fs::metadata(&dir_tmp) {
            if base_caonfig_file.is_file() {
            } else {
                std::fs::remove_dir(&dir_tmp).unwrap();
                let _ = std::fs::File::create(&dir_tmp).unwrap();
                let contents = toml::to_string(&default_base_config).unwrap();
                let _ = std::fs::write(&dir_tmp, contents).unwrap();
            }
        } else {
            let _ = std::fs::File::create(&dir_tmp).unwrap();
            let contents = toml::to_string(&default_base_config).unwrap();
            let _ = std::fs::write(&dir_tmp, contents).unwrap();
        };
        let base_config: BaseConfig =
            toml::from_str(&std::fs::read_to_string(dir_tmp).unwrap()).unwrap();
        let env_config: EnvConfig =
            toml::from_str(&std::fs::read_to_string(base_config.config_file).unwrap()).unwrap();
        let env = Env::new(&env_config.jar_paths, &env_config.java_opts);
        env
    }
    pub fn new() -> Self {
        let env = Self::internal_new_env();
        let config = Some(env.new_bot_configuration());
        Self {
            env,
            config,
            id: None,
            password: None,
            password_md5: None,
        }
    }
    pub fn new_without_bot_configuration() -> Self {
        let env = Self::internal_new_env();
        Self {
            env,
            config: None,
            id: None,
            password: None,
            password_md5: None,
        }
    }
    pub fn id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }
    pub fn password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }
    pub fn password_md5(mut self, password_md5: [u8; 16]) -> Self {
        self.password_md5 = Some(password_md5);
        self
    }
    /// 在被挤下线时自动重连。
    pub fn auto_reconnect_on_force_offline(self) -> Self {
        if let Some(config) = &self.config {
            config.auto_reconnect_on_force_offline();
        }
        self
    }
    /// 禁止保存 account.secrets.
    /// 根据 Mirai 注释：
    ///     禁止保存 account.secrets.
    ///     account.secrets 保存账号的会话信息。
    ///     它可加速登录过程，也可能可以减少出现验证码的次数。
    ///     如果遇到一段时间后无法接收消息通知等同步问题时可尝试禁用。
    pub fn disable_account_secretes(self) -> Self {
        if let Some(config) = &self.config {
            config.disable_account_secretes();
        }
        self
    }
    /// 禁用好友列表和群成员列表的缓存。
    /// ~应该~**不**是默认行为。
    pub fn disable_contact_cache(self) -> Self {
        if let Some(config) = &self.config {
            config.disable_contact_cache();
        }
        self
    }
    /// 启用好友列表和群成员列表的缓存。
    /// ~应该~是默认行为。
    pub fn enable_contact_cache(self) -> Self {
        if let Some(config) = &self.config {
            config.enable_contact_cache();
        }
        self
    }
    /// 使用文件存储设备信息。
    /// 可以传入 None 参数，此时默认使用工作目录下的 device.json 文件。
    /// 在传入 None 参数的情况下，如果 device.json 文件不存在的话，
    /// Mirai 似乎会发出警告，然后随机生成一个设备信息。
    /// TODO: 测试传入参数的行为。
    pub fn file_based_device_info(self, path: Option<&PathBuf>) -> Self {
        if let Some(config) = &self.config {
            config.file_based_device_info(path);
        }
        self
    }
    /// 不显示 Bot 日志。不推荐。
    pub fn no_bot_log(self) -> Self {
        if let Some(config) = &self.config {
            config.no_bot_log();
        }
        self
    }
    /// 不显示网络日志。不推荐。
    pub fn no_network_log(self) -> Self {
        if let Some(config) = &self.config {
            config.no_network_log();
        }
        self
    }
    /// 使用随机设备信息。
    /// 注意该函数~应该~不会持久化当前随机信息。
    pub fn random_device_info(self) -> Self {
        if let Some(config) = &self.config {
            config.random_device_info();
        }
        self
    }
    /// 重定向 Bot 日志到指定目录。若目录不存在将会自动创建。
    pub fn redirect_bot_log_to_directory(
        self,
        path: Option<&PathBuf>,
        retain: Option<i64>,
        indentity: Option<Instance>,
    ) -> Self {
        if let Some(config) = &self.config {
            config.redirect_bot_log_to_directory(path, retain, indentity);
        }
        self
    }
    /// 重定向 Bot 日志到指定文件。日志将会逐行追加到此文件。若文件不存在将会自动创建。
    pub fn redirect_bot_log_to_file(
        self,
        path: Option<&PathBuf>,
        indentity: Option<Instance>,
    ) -> Self {
        if let Some(config) = &self.config {
            config.redirect_bot_log_to_file(path, indentity);
        }
        self
    }
    /// 重定向网络日志到指定目录。若目录不存在将会自动创建。
    /// 默认目录路径为工作目录下的 `logs/` 文件夹。
    pub fn redirect_network_log_to_directory(
        self,
        path: Option<&PathBuf>,
        retain: Option<i64>,
        indentity: Option<Instance>,
    ) -> Self {
        if let Some(config) = &self.config {
            config.redirect_network_log_to_directory(path, retain, indentity);
        }
        self
    }
    /// 重定向网络日志到指定文件。默认文件路径为工作目录下的 `mirai.log`.
    /// 日志将会逐行追加到此文件。若文件不存在将会自动创建。
    pub fn redirect_network_log_to_file(
        self,
        path: Option<&PathBuf>,
        indentity: Option<Instance>,
    ) -> Self {
        if let Some(config) = &self.config {
            config.redirect_network_log_to_file(path, indentity);
        }
        self
    }
    pub fn fix_protocol_version_fetch(
        self,
        protocol: enums::MiraiProtocol,
        version: String,
    ) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        println!("fix protocol version - tmp - fetch");
        let _ = jvm.invoke_static(
            "xyz.cssxsh.mirai.tool.FixProtocolVersion",
            "fetch",
            &[
                tools::protocol_enum_r2j(protocol).unwrap(),
                InvocationArg::try_from(version).unwrap(),
            ],
        );
        self
    }
    pub fn fix_protocol_version_load(self, protocol: enums::MiraiProtocol) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        println!("fix protocol version - tmp - load");
        let _ = jvm.invoke_static(
            "xyz.cssxsh.mirai.tool.FixProtocolVersion",
            "load",
            &[tools::protocol_enum_r2j(protocol).unwrap()],
        );
        self
    }
    pub fn fix_protocol_version_info(&self) -> HashMap<String, String> {
        let jvm = Jvm::attach_thread().unwrap();
        println!("fix protocol version - tmp - info");
        let map: MiraiMap<String, String> = MiraiMap {
            instance: jvm
                .invoke_static("xyz.cssxsh.mirai.tool.FixProtocolVersion", "info", &[])
                .unwrap(),
            _t: None,
        };
        map.to_hash_map()
    }
    pub fn fix_protocol_version_update(self) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        println!("fix protocol version - tmp - update");
        let _ = jvm.invoke_static("xyz.cssxsh.mirai.tool.FixProtocolVersion", "update", &[]);
        self
    }
    pub fn build(self) -> Bot {
        if let Some(password) = self.password {
            self.env
                .new_bot(self.id.expect("请提供 id!"), password, self.config)
        } else {
            self.env.new_bot(
                self.id.expect("请提供 id!"),
                self.password_md5.expect("请提供密码！"),
                self.config,
            )
        }
    }
}
