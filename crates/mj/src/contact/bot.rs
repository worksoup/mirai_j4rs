use crate::{
    auth::bot_authorization::BotAuthorization,
    contact::{
        contact_trait::{ContactOrBotTrait, NudgeSupportedTrait, UserOrBotTrait},
        group::{Group, MiraiMap},
        ContactList, Friend, OtherClient, Stranger,
    },
    error::MiraiRsError,
    event::EventChannel,
    message::action::nudges::BotNudge,
    utils::{
        contact::friend_group::FriendGroups,
        login_solver::{
            DeviceVerificationRequests, DeviceVerificationResult, LoginSolverTrait,
            QrCodeLoginListener,
        },
        other::enums::{AvatarSpec, HeartbeatStrategy, MiraiProtocol},
    },
};
use j4rs::{ClasspathEntry, Instance, InvocationArg, JavaOpt, Jvm, JvmBuilder};
use mj_base::{
    env::{FromInstance, GetEnvTrait},
    utils::{instance_is_null, java_iter_to_rust_vec},
};
use mj_closures::{kt_func_1::KtFunc1Raw, kt_func_2::KtFunc2Raw};
use mj_macro::GetInstanceDerive;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub struct Bot {
    pub(crate) instance: Instance,
    pub(crate) id: i64,
}

impl FromInstance for Bot {
    fn from_instance(instance: Instance) -> Self {
        let id = Jvm::attach_thread()
            .unwrap()
            .chain(&instance)
            .unwrap()
            .invoke("getId", &[])
            .unwrap()
            .to_rust()
            .unwrap();
        Bot { instance, id }
    }
}

impl GetEnvTrait for Bot {
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}

impl Bot {
    pub fn close(self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "close", &[])
            .unwrap();
    }
    pub fn close_and_join(self, err: MiraiRsError) {
        // TODO
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "closeAndJoin", &[])
            .unwrap();
    }
    pub fn get_as_friend(&self) -> Friend {
        let id = self.id;
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getAsFriend", &[])
            .unwrap();
        Friend {
            bot: self.get_instance(),
            instance,
            id,
        }
    }
    pub fn get_as_stranger(&self) -> Stranger {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getAsStranger", &[])
            .unwrap();
        Stranger::from_instance(instance)
    }
    pub fn get_configuration(&self) -> BotConfiguration {
        let bot_configuration = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getConfiguration", &[])
            .unwrap();
        BotConfiguration {
            instance: bot_configuration,
            _login_solver_holder: None,
        }
    }
    pub fn get_event_channel(&self) -> EventChannel {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getEventChannel", &[])
            .unwrap();
        EventChannel { jvm, instance }
    }
    pub fn get_friend(&self, id: i64) -> Option<Friend> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "getFriend",
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        if !instance_is_null(&instance) {
            Some(Friend {
                bot: self.get_instance(),
                instance,
                id,
            })
        } else {
            None
        }
    }
    pub fn get_friend_groups(&self) -> FriendGroups {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getFriendGroups", &[]).unwrap();
        FriendGroups { instance }
    }
    pub fn get_friends(&self) -> ContactList<Friend> {
        ContactList::from_instance(
            Jvm::attach_thread()
                .unwrap()
                .invoke(&self.instance, "getFriends", &[])
                .unwrap(),
        )
    }
    pub fn get_group(&self, id: i64) -> Option<Group> {
        Group::new(self, id)
    }
    pub fn get_groups(&self) -> ContactList<Group> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getGroups", &[])
            .unwrap();
        ContactList::from_instance(instance)
    }
    pub fn get_logger() -> MiraiLogger {
        todo!("get logger")
    }
    pub fn get_other_clients(&self) -> ContactList<OtherClient> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getOtherClients", &[])
            .unwrap();
        ContactList::from_instance(instance)
    }
    pub fn get_stranger(&self, id: i64) -> Option<Stranger> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "getStranger",
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        if !instance_is_null(&instance) {
            Some(Stranger::from_instance(instance))
        } else {
            None
        }
    }
    pub fn get_strangers(&self) -> ContactList<Stranger> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getStrangers", &[])
            .unwrap();
        ContactList::from_instance(instance)
    }
    pub fn is_online(&self) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .chain(&self.instance)
            .unwrap()
            .invoke("isOnline", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    pub fn join(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "join", &[])
            .unwrap();
    }
    pub fn login(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "login", &Vec::new())
            .unwrap();
    }
}

impl ContactOrBotTrait for Bot {
    fn get_id(&self) -> i64 {
        self.id
    }
    fn get_avatar_url(&self, size: Option<AvatarSpec>) -> String {
        let size: i32 = if let Some(size) = size {
            size.into()
        } else {
            AvatarSpec::XL.into()
        };
        // 这里 Mirai 源码中应该是 http 而不是 https.
        let mut url = "https://q.qlogo.cn/g?b=qq&nk=".to_string();
        url.push_str(self.get_id().to_string().as_str());
        url.push_str("&s=");
        url.push_str(size.to_string().as_str());
        url
    }
}

impl UserOrBotTrait for Bot {}

impl NudgeSupportedTrait for Bot {
    type NudgeType = BotNudge;
}

pub struct MiraiLogger(Instance);

pub struct DeviceInfo(Instance);

/// 该结构体未实现 [`LoginSolverTrait`], 如需使用相关功能请调用本实例的 `get_instance` 方法，获得 `Instance` 后直接操作。
#[derive(GetInstanceDerive)]
pub struct LoginSolver {
    instance: Instance,
}

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
    pub fn new_bot_configuration(&self) -> BotConfiguration {
        let bot_configuration = Jvm::attach_thread()
            .unwrap()
            .invoke_static("net.mamoe.mirai.utils.BotConfiguration", "getDefault", &[])
            .unwrap();
        BotConfiguration {
            instance: bot_configuration,
            _login_solver_holder: None,
        }
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
        if !instance_is_null(&bot) {
            Some(Bot { instance: bot, id })
        } else {
            None
        }
    }
    pub fn get_bots(&self) -> Vec<Bot> {
        let jvm = &self.jvm;
        let instance = jvm
            .invoke_static("net.mamoe.mirai.Bot$Companion", "getInstances", &[])
            .unwrap();
        let iter = jvm.invoke(&instance, "iterator", &[]).unwrap();
        java_iter_to_rust_vec(&jvm, iter)
    }
    //默认是global的。
    pub fn event_channel(&self) -> EventChannel {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = Jvm::attach_thread()
            .unwrap()
            .static_class("net.mamoe.mirai.event.GlobalEventChannel$INSTANCE")
            .unwrap();
        EventChannel { jvm, instance }
    }
    pub fn new_bot(&self, id: i64, bot_authorization: BotAuthorization) -> Bot {
        let bot = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "newBot",
                &vec![
                    InvocationArg::try_from(id)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(bot_authorization.get_instance()).unwrap(),
                ],
            )
            .unwrap();
        Bot { instance: bot, id }
    }
    pub fn new_bot_with_configuration(
        &self,
        id: i64,
        bot_authorization: BotAuthorization,
        bot_configuration: BotConfiguration,
    ) -> Bot {
        let bot = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "newBot",
                &[
                    InvocationArg::try_from(id)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(bot_authorization.get_instance()).unwrap(),
                    InvocationArg::try_from(bot_configuration.get_instance()).unwrap(),
                ],
            )
            .unwrap();
        Bot { instance: bot, id }
    }
}

pub struct BotConfiguration {
    instance: Instance,
    _login_solver_holder: Option<(KtFunc2Raw, KtFunc2Raw, KtFunc1Raw, KtFunc2Raw)>,
}

impl GetEnvTrait for BotConfiguration {
    fn get_instance(&self) -> Instance {
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
        BotConfiguration {
            instance,
            _login_solver_holder: None,
        }
    }
    pub fn get_default() -> Self {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke_static("net.mamoe.mirai.utils.BotConfiguration", "getDefault", &[])
            .unwrap();
        BotConfiguration {
            instance,
            _login_solver_holder: None,
        }
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
                                .clone_instance(&bot.instance)
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
                                .clone_instance(&bot.instance)
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
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getLoginSolver", &[])
            .unwrap();
        if !instance_is_null(&instance) {
            Some(LoginSolver { instance })
        } else {
            None
        }
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
        crate::utils::other::protocol_str2enum(mp)
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

/// isXXX()
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
        todo!("not impl yet: set_bot_logger_supplier");
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
        todo!("set_device_info");
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
    pub fn set_login_solver<T>(&mut self, _: T)
    where
        T: LoginSolverTrait,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let (instance, _1, _2, _3, _4) = T::__instance();
        jvm.invoke(
            &self.instance,
            "setLoginSolver",
            &[InvocationArg::try_from(instance).unwrap()],
        )
        .unwrap();
        // 防止 drop.
        self._login_solver_holder = Some((_1, _2, _3, _4))
    }
    pub fn set_network_logger_supplier(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "setNetworkLoggerSupplier", &[])
            .unwrap();
        todo!("set_network_logger_supplier");
    }

    pub fn set_protocol(&self, protocol: MiraiProtocol) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setProtocol",
                &[crate::utils::other::protocol_enum_r2j(&protocol).unwrap()],
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
    /// 注意该函数<s>应该</s>不会持久化当前随机信息。
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
        identity: Option<Instance>,
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
            7i64 * 24 * 60 * 60 * 1000
        } else {
            retain.unwrap()
        })
        .unwrap()
        .into_primitive()
        .unwrap();
        if identity.is_none() {
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
                        InvocationArg::try_from(identity.unwrap()).unwrap(),
                    ],
                )
                .unwrap();
        };
    }
    fn redirect_log_to_file(
        &self,
        path: Option<&PathBuf>,
        identity: Option<Instance>,
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
        if identity.is_none() {
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
                        InvocationArg::try_from(identity.unwrap()).unwrap(),
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
        identity: Option<Instance>,
    ) {
        self.redirect_log_to_directory(path, retain, identity, "logs", "redirectBotLogToDirectory");
    }
    /// 重定向 Bot 日志到指定文件。日志将会逐行追加到此文件。若文件不存在将会自动创建。
    pub fn redirect_bot_log_to_file(&self, path: Option<&PathBuf>, identity: Option<Instance>) {
        self.redirect_log_to_file(path, identity, "mirai.log", "redirectBotLogToFile");
    }
    /// 重定向网络日志到指定目录。若目录不存在将会自动创建。
    /// 默认目录路径为 `$workingDir/logs/`.
    pub fn redirect_network_log_to_directory(
        &self,
        path: Option<&PathBuf>,
        retain: Option<i64>,
        identity: Option<Instance>,
    ) {
        self.redirect_log_to_directory(
            path,
            retain,
            identity,
            "logs",
            "redirectNetworkLogToDirectory",
        );
    }
    /// 重定向网络日志到指定文件。默认文件路径为 `$workingDir/mirai.log`.
    /// 日志将会逐行追加到此文件。若文件不存在将会自动创建。
    pub fn redirect_network_log_to_file(&self, path: Option<&PathBuf>, identity: Option<Instance>) {
        self.redirect_log_to_file(path, identity, "mirai.log", "redirectNetworkLogToFile");
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
    bot_authorization: Option<BotAuthorization>,
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
    fn internal_new_env(config_dir: &Path) -> Env {
        let default_base_config = BaseConfig {
            config_file: "env_config.toml".to_string(),
        };
        if let Err(_) = std::fs::metadata(config_dir) {
            let _ = std::fs::create_dir_all(config_dir);
        }
        let mut dir_tmp = config_dir.to_path_buf();
        dir_tmp.push("base_config.toml");
        // 如果 `base_config.toml` 不存在则创建一个默认的。
        if let Ok(base_config_file) = std::fs::metadata(&dir_tmp) {
            if !base_config_file.is_file() {
                // std::fs::remove_dir(&dir_tmp).unwrap();
                // let _ = std::fs::File::create(&dir_tmp).unwrap();
                // let contents = toml::to_string(&default_base_config).unwrap();
                // let _ = std::fs::write(&dir_tmp, contents).unwrap();
                panic!("`base_config.toml` 不是文件！")
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
    // pub fn new() -> Self {
    //     let config_dir = Path::new(".");
    //     let env = Self::internal_new_env(config_dir);
    //     let config = Some(env.new_bot_configuration());
    //     Self {
    //         env,
    //         config,
    //         id: None,
    //         password: None,
    //         password_md5: None,
    //     }
    // }
    pub fn new(config_dir: &Path) -> Self {
        let env = Self::internal_new_env(config_dir);
        Self {
            env,
            config: None,
            id: None,
            bot_authorization: None,
        }
    }
    pub fn id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }
    pub fn authorization(mut self, bot_authorization: BotAuthorization) -> Self {
        self.bot_authorization = Some(bot_authorization);
        self
    }
    /// 在被挤下线时自动重连。
    pub fn auto_reconnect_on_force_offline(mut self) -> Self {
        if let Some(config) = &self.config {
            config.auto_reconnect_on_force_offline();
        } else {
            let config = self.env.new_bot_configuration();
            config.auto_reconnect_on_force_offline();
            self.config = Some(config);
        }
        self
    }
    /// 禁止保存 account.secrets.
    /// 根据 Mirai 注释：
    ///     禁止保存 account.secrets.
    ///     account.secrets 保存账号的会话信息。
    ///     它可加速登录过程，也可能可以减少出现验证码的次数。
    ///     如果遇到一段时间后无法接收消息通知等同步问题时可尝试禁用。
    pub fn disable_account_secretes(mut self) -> Self {
        if let Some(config) = &self.config {
            config.disable_account_secretes();
        } else {
            let config = self.env.new_bot_configuration();
            config.disable_account_secretes();
            self.config = Some(config);
        }
        self
    }
    /// 禁用好友列表和群成员列表的缓存。
    /// ~应该~**不**是默认行为。
    pub fn disable_contact_cache(mut self) -> Self {
        if let Some(config) = &self.config {
            config.disable_contact_cache();
        } else {
            let config = self.env.new_bot_configuration();
            config.disable_contact_cache();
            self.config = Some(config);
        }
        self
    }
    /// 启用好友列表和群成员列表的缓存。
    /// ~应该~是默认行为。
    pub fn enable_contact_cache(mut self) -> Self {
        if let Some(config) = &self.config {
            config.enable_contact_cache();
        } else {
            let config = self.env.new_bot_configuration();
            config.enable_contact_cache();
            self.config = Some(config);
        }
        self
    }
    /// 使用文件存储设备信息。
    /// 可以传入 None 参数，此时默认使用工作目录下的 device.json 文件。
    /// 在传入 None 参数的情况下，如果 device.json 文件不存在的话，
    /// Mirai 似乎会发出警告，然后随机生成一个设备信息。
    /// TODO: 测试传入参数的行为。
    pub fn file_based_device_info(mut self, path: Option<&PathBuf>) -> Self {
        if let Some(config) = &self.config {
            config.file_based_device_info(path);
        } else {
            let config = self.env.new_bot_configuration();
            config.file_based_device_info(path);
            self.config = Some(config);
        }
        self
    }
    /// 不显示 Bot 日志。不推荐。
    pub fn no_bot_log(mut self) -> Self {
        if let Some(config) = &self.config {
            config.no_bot_log();
        } else {
            let config = self.env.new_bot_configuration();
            config.no_bot_log();
            self.config = Some(config);
        }
        self
    }
    /// 不显示网络日志。不推荐。
    pub fn no_network_log(mut self) -> Self {
        if let Some(config) = &self.config {
            config.no_network_log();
        } else {
            let config = self.env.new_bot_configuration();
            config.no_network_log();
            self.config = Some(config);
        }
        self
    }
    /// 使用随机设备信息。
    /// 注意该函数~应该~不会持久化当前随机信息。
    pub fn random_device_info(mut self) -> Self {
        if let Some(config) = &self.config {
            config.random_device_info();
        } else {
            let config = self.env.new_bot_configuration();
            config.random_device_info();
            self.config = Some(config);
        }
        self
    }
    /// 重定向 Bot 日志到指定目录。若目录不存在将会自动创建。
    pub fn redirect_bot_log_to_directory(
        mut self,
        path: Option<&PathBuf>,
        retain: Option<i64>,
        identity: Option<Instance>,
    ) -> Self {
        if let Some(config) = &self.config {
            config.redirect_bot_log_to_directory(path, retain, identity);
        } else {
            let config = self.env.new_bot_configuration();
            config.redirect_bot_log_to_directory(path, retain, identity);
            self.config = Some(config);
        }
        self
    }
    /// 重定向 Bot 日志到指定文件。日志将会逐行追加到此文件。若文件不存在将会自动创建。
    pub fn redirect_bot_log_to_file(
        mut self,
        path: Option<&PathBuf>,
        identity: Option<Instance>,
    ) -> Self {
        if let Some(config) = &self.config {
            config.redirect_bot_log_to_file(path, identity);
        } else {
            let config = self.env.new_bot_configuration();
            config.redirect_bot_log_to_file(path, identity);
            self.config = Some(config);
        }
        self
    }
    /// 重定向网络日志到指定目录。若目录不存在将会自动创建。
    /// 默认目录路径为工作目录下的 `logs/` 文件夹。
    pub fn redirect_network_log_to_directory(
        mut self,
        path: Option<&PathBuf>,
        retain: Option<i64>,
        identity: Option<Instance>,
    ) -> Self {
        if let Some(config) = &self.config {
            config.redirect_network_log_to_directory(path, retain, identity);
        } else {
            let config = self.env.new_bot_configuration();
            config.redirect_network_log_to_directory(path, retain, identity);
            self.config = Some(config);
        }
        self
    }
    /// 重定向网络日志到指定文件。默认文件路径为工作目录下的 `mirai.log`.
    /// 日志将会逐行追加到此文件。若文件不存在将会自动创建。
    pub fn redirect_network_log_to_file(
        mut self,
        path: Option<&PathBuf>,
        identity: Option<Instance>,
    ) -> Self {
        if let Some(config) = &self.config {
            config.redirect_network_log_to_file(path, identity);
        } else {
            let config = self.env.new_bot_configuration();
            config.redirect_network_log_to_file(path, identity);
            self.config = Some(config);
        }
        self
    }
    /// 使用协议类型。
    pub fn set_protocol(mut self, protocol: MiraiProtocol) -> BotBuilder {
        if let Some(config) = &self.config {
            config.set_protocol(protocol);
        } else {
            let config = self.env.new_bot_configuration();
            config.set_protocol(protocol);
            self.config = Some(config);
        }
        self
    }
    pub fn build(self) -> Bot {
        let id = if let Some(id) = self.id {
            id
        } else {
            panic!("请提供 id!")
        };
        let bot_authorization = if let Some(bot_authorization) = self.bot_authorization {
            bot_authorization
        } else {
            eprintln!("没有提供登录方式！将尝试使用二维码登录！");
            let bot_authorization = BotAuthorization::QrCode;
            let is_qr_login_supported = if let Some(bot_config) = self.config.as_ref() {
                let p = bot_config.get_protocol();
                p.is_qr_login_supported()
            } else {
                MiraiProtocol::A.is_qr_login_supported()
            };
            if is_qr_login_supported {
                bot_authorization
            } else {
                panic!("当前协议不支持二维码登录！")
            }
        };
        if let Some(config) = self.config {
            self.env
                .new_bot_with_configuration(id, bot_authorization, config)
        } else {
            self.env.new_bot(id, bot_authorization)
        }
    }
}
