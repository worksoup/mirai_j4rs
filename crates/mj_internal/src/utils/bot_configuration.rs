use std::path::PathBuf;

use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{
    java_all, utils::instance_is_null, FromInstanceTrait, GetClassTypeTrait, GetInstanceTrait,
};

use mj_helper_macro::{error_msg_suppressor, java_fn, mj_all};

use crate::utils::backend::Mirai;
use crate::{
    contact::Bot,
    utils::{
        data_wrapper::{DataWrapper, PrimitiveConvert},
        login_solver::{LoginSolver, LoginSolverTrait},
        other::enums::{HeartbeatStrategy, MiraiProtocol},
        DeviceInfo, MiraiLogger,
    },
};

#[java_all]
pub struct ContactListCache {
    instance: Instance,
}

impl ContactListCache {
    #[java_fn]
    pub fn get_save_interval_millis(&self) -> i64 {}
    #[java_fn]
    pub fn set_save_interval_millis(&self, millis: DataWrapper<i64, PrimitiveConvert>) {}
    #[java_fn]
    pub fn get_friend_list_cache_enabled(&self) -> bool {}
    #[java_fn]
    pub fn set_friend_list_cache_enabled(&self, bool: DataWrapper<bool, PrimitiveConvert>) {}
    #[java_fn]
    pub fn get_group_member_list_cache_enabled(&self) -> bool {}
    #[java_fn]
    pub fn set_group_member_list_cache_enabled(
        &self,
        enabled: DataWrapper<bool, PrimitiveConvert>,
    ) {
    }
}
#[mj_all("utils.BotConfiguration")]
pub struct BotConfiguration {
    instance: Instance,
    _login_solver_holder: Option<()>,
}
// builders
impl BotConfiguration {
    #[java_fn("copy")]
    fn copy_internal(&self) -> Self {}
    pub fn copy_configuration_from(bot: &Bot<Mirai>) -> Self {
        bot.get_configuration().copy_internal()
    }
}
impl Default for BotConfiguration {
    #[java_fn("getDefault")]
    fn default() -> Self {}
}
// getters
impl BotConfiguration {
    #[java_fn]
    pub fn get_auto_reconnect_on_force_offline(&self) -> bool {}
    pub fn get_bot_logger_supplier(&self) -> Box<dyn Fn(&Bot<Mirai>) -> MiraiLogger + '_> {
        let bot_logger_supplier = |bot: &Bot<Mirai>| -> MiraiLogger {
            let tmp = Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self.instance,
                    "getBotLoggerSupplier",
                    InvocationArg::empty(),
                )
                .unwrap();
            MiraiLogger(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(
                        &tmp,
                        "invoke",
                        &[InvocationArg::try_from(bot.get_instance()).unwrap()],
                    )
                    .unwrap(),
            )
        };
        Box::new(bot_logger_supplier)
    }
    #[java_fn]
    pub fn get_cache_dir(&self) -> PathBuf {
        let i: String = error_msg_suppressor!("jvm.chain(&instance)")
            .unwrap()
            .invoke("toString", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap();
        PathBuf::from(i)
    }
    #[java_fn]
    pub fn get_contact_list_cache(&self) -> Option<ContactListCache> {
        Some(ContactListCache::from_instance(error_msg_suppressor!(
            "instance"
        )))
    }
    pub fn get_device_info(&self) -> Option<impl Fn(Bot<Mirai>) -> DeviceInfo + '_> {
        let tmp = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getDeviceInfo", InvocationArg::empty())
            .unwrap();
        let bot_logger_supplier = move |bot: Bot<Mirai>| -> DeviceInfo {
            DeviceInfo(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(
                        &tmp,
                        "invoke",
                        &[InvocationArg::try_from(bot.get_instance()).unwrap()],
                    )
                    .unwrap(),
            )
        };
        Some(bot_logger_supplier)
    }
    #[java_fn]
    pub fn get_heartbeat_period_millis(&self) -> i64 {}

    #[java_fn]
    pub fn get_heartbeat_strategy(&self) -> HeartbeatStrategy {
        let jvm: Jvm = error_msg_suppressor!("jvm");
        let instance: Instance = error_msg_suppressor!("instance");
        let hbs: String = jvm.to_rust(instance).unwrap();
        match hbs.as_str() {
            "STAT_HB" => HeartbeatStrategy::S,
            "REGISTER" => HeartbeatStrategy::R,
            "NONE" => HeartbeatStrategy::N,
            _ => {
                println!("&self.instance is None!");
                HeartbeatStrategy::S
            }
        }
    }
    #[java_fn]
    pub fn get_heartbeat_timeout_millis(&self) -> i64 {}
    #[java_fn]
    pub fn get_highway_upload_coroutine_count(&self) -> i32 {}
    #[java_fn]
    pub fn get_login_cache_enabled(&self) -> bool {}
    pub fn get_login_solver(&self) -> Option<LoginSolver> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getLoginSolver", InvocationArg::empty())
            .unwrap();
        if !instance_is_null(&instance) {
            Some(LoginSolver::from_instance(instance))
        } else {
            None
        }
    }
    pub fn get_network_logger_supplier(&self) -> Option<Instance> {
        return Some(
            Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self.instance,
                    "getNetworkLoggerSupplier",
                    InvocationArg::empty(),
                )
                .unwrap(),
        );
    }
    pub fn get_parent_coroutine_context(&self) -> Option<Instance> {
        return Some(
            Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self.instance,
                    "getParentCoroutineContext",
                    InvocationArg::empty(),
                )
                .unwrap(),
        );
    }
    #[java_fn]
    pub fn get_protocol(&self) -> MiraiProtocol {
        let jvm: Jvm = error_msg_suppressor!("jvm");
        let instance: Instance = error_msg_suppressor!("instance");
        let mp: String = jvm.to_rust(instance).unwrap();
        crate::utils::other::protocol_str2enum(mp)
    }
    #[java_fn]
    pub fn get_reconnection_retry_times(&self) -> i32 {}
    #[java_fn]
    pub fn get_stat_heartbeat_period_millis(&self) -> i64 {}
    #[java_fn]
    pub fn get_working_dir(&self) -> PathBuf {
        let jvm: Jvm = error_msg_suppressor!("jvm");
        let instance: Instance = error_msg_suppressor!("instance");
        let i: String = jvm
            .chain(&instance)
            .unwrap()
            .invoke("toString", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap();
        PathBuf::from(i)
    }
}

/// isXXX()
impl BotConfiguration {
    #[java_fn]
    pub fn is_convert_line_separator(&self) -> bool {}
    #[java_fn]
    pub fn is_showing_verbose_event_log(&self) -> bool {}
}

/// setters
impl BotConfiguration {
    #[java_fn]
    pub fn set_auto_reconnect_on_force_offline(&self, yes: DataWrapper<bool, PrimitiveConvert>) {}
    pub fn set_bot_logger_supplier(&self) {
        todo!("not impl yet: set_bot_logger_supplier");
    }

    #[java_fn("setCacheDir")]
    fn set_cache_dir_internal(&self, file: Instance) {}
    pub fn set_cache_dir(&self, path: &str) {
        let file = Jvm::attach_thread()
            .unwrap()
            .create_instance("java.io.File", &[InvocationArg::try_from(path).unwrap()])
            .unwrap();
        self.set_cache_dir_internal(file)
    }
    #[java_fn]
    pub fn set_contact_list_cache(&self, cache: ContactListCache) {}
    #[java_fn]
    pub fn set_convert_line_separator(&self, yes: bool) {}
    pub fn set_device_info(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "setDeviceInfo", InvocationArg::empty())
            .unwrap();
        todo!("set_device_info");
    }
    #[java_fn]
    pub fn set_heartbeat_period_millis(&self, millis: DataWrapper<i64, PrimitiveConvert>) {}
    pub fn set_heartbeat_strategy(&self, heartbeat_strategy: HeartbeatStrategy) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setHeartbeatStrategy",
                &[InvocationArg::from(
                    Jvm::attach_thread()
                        .unwrap()
                        .field(
                            &Jvm::attach_thread()
                                .unwrap()
                                .static_class(
                                    (<Self as GetClassTypeTrait>::get_type_name().to_string()
                                        + "$HeartbeatStrategy")
                                        .as_str(),
                                )
                                .unwrap(),
                            match heartbeat_strategy {
                                HeartbeatStrategy::S => "STAT_HB",
                                HeartbeatStrategy::R => "REGISTER",
                                HeartbeatStrategy::N => "NONE",
                            },
                        )
                        .unwrap(),
                )],
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
        //java fn :"setLoginSolver".
        todo!()
        // let jvm = Jvm::attach_thread().unwrap();
        // jvm.invoke(
        //     &self.instance,
        //     "setLoginSolver",
        //     &[InvocationArg::try_from(todo!()).unwrap()],
        // )
        // .unwrap();
        // // 防止 drop.
        // self._login_solver_holder = todo!()
    }
    pub fn set_network_logger_supplier(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setNetworkLoggerSupplier",
                InvocationArg::empty(),
            )
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
    pub fn set_working_dir(&self, path: &str) {
        let file = Jvm::attach_thread()
            .unwrap()
            .create_instance("java.io.File", &[InvocationArg::try_from(path).unwrap()])
            .unwrap();
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "setWorkingDir",
                &[InvocationArg::from(file)],
            )
            .unwrap();
    }
}

impl BotConfiguration {
    /// 在被挤下线时自动重连。
    pub fn auto_reconnect_on_force_offline(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "autoReconnectOnForceOffline",
                InvocationArg::empty(),
            )
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
            .invoke(
                &self.instance,
                "disableAccountSecretes",
                InvocationArg::empty(),
            )
            .unwrap();
    }
    /// 禁用好友列表和群成员列表的缓存。
    /// ~应该~**不**是默认行为。
    pub fn disable_contact_cache(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "disableContactCache",
                InvocationArg::empty(),
            )
            .unwrap();
    }
    /// 启用好友列表和群成员列表的缓存。
    /// ~应该~是默认行为。
    pub fn enable_contact_cache(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "enableContactCache", InvocationArg::empty())
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
                .invoke(
                    &self.instance,
                    "fileBasedDeviceInfo",
                    InvocationArg::empty(),
                )
                .unwrap();
        }
    }
    /// 不显示 Bot 日志。不推荐。
    pub fn no_bot_log(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "noBotLog", InvocationArg::empty())
            .unwrap();
    }
    /// 不显示网络日志。不推荐。
    pub fn no_network_log(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "noNetworkLog", InvocationArg::empty())
            .unwrap();
    }
    /// 使用随机设备信息。
    /// 注意该函数<s>应该</s>不会持久化当前随机信息。
    pub fn random_device_info(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "randomDeviceInfo", InvocationArg::empty())
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
        let path = if let Some(path) = path {
            Jvm::attach_thread()
                .unwrap()
                .create_instance(
                    "java.io.File",
                    &[InvocationArg::try_from(path.to_str().unwrap()).unwrap()],
                )
                .unwrap()
        } else {
            Jvm::attach_thread()
                .unwrap()
                .create_instance("java.io.File", &[InvocationArg::try_from(arg).unwrap()])
                .unwrap()
        };
        let retain = InvocationArg::try_from(retain.unwrap_or(7i64 * 24 * 60 * 60 * 1000))
            .unwrap()
            .into_primitive()
            .unwrap();
        if let Some(identity) = identity {
            Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self.instance,
                    method_name,
                    &[
                        InvocationArg::from(path),
                        retain,
                        InvocationArg::from(identity),
                    ],
                )
                .unwrap();
        } else {
            Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self.instance,
                    method_name,
                    &[InvocationArg::from(path), retain],
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
        let path = if let Some(path) = path {
            Jvm::attach_thread()
                .unwrap()
                .create_instance(
                    "java.io.File",
                    &[InvocationArg::try_from(path.to_str().unwrap()).unwrap()],
                )
                .unwrap()
        } else {
            Jvm::attach_thread()
                .unwrap()
                .create_instance("java.io.File", &[InvocationArg::try_from(arg).unwrap()])
                .unwrap()
        };
        if let Some(identity) = identity {
            Jvm::attach_thread()
                .unwrap()
                .invoke(
                    &self.instance,
                    method_name,
                    &[InvocationArg::from(path), InvocationArg::from(identity)],
                )
                .unwrap();
        } else {
            Jvm::attach_thread()
                .unwrap()
                .invoke(&self.instance, method_name, &[InvocationArg::from(path)])
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
    //             .invoke(bot_configuration, "disableAccountSecretes", InvocationArg::empty())
    //             .unwrap();
    //     }
    // }
}
