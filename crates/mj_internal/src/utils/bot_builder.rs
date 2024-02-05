use crate::event::bot::BotOfflineEvent;
use crate::{
    auth::bot_authorization::BotAuthorization,
    contact::Bot,
    utils::{
        login_solver::LoginSolverTrait,
        other::enums::{HeartbeatStrategy, MiraiProtocol},
        BotConfiguration, ContactListCache,
    },
};
use j4rs::{ClasspathEntry, Instance, InvocationArg, JavaOpt, Jvm, JvmBuilder};
use mj_base::env::{FromInstance, GetInstanceTrait};
use std::path::{Path, PathBuf};

/// bot builder
pub struct BotBuilder {
    instance: Instance,
    jvm: Jvm,
    config: BotConfiguration,
    id: Option<i64>,
    bot_authorization: Option<BotAuthorization>,
}
impl Default for BotBuilder {
    fn default() -> Self {
        Self::new(".")
    }
}
impl BotBuilder {
    pub fn new<P: AsRef<Path>>(working_dir: P) -> Self {
        let working_dir = std::fs::canonicalize(working_dir).expect("目录无法解析。");
        let all_jars = fs_extra::dir::get_dir_content(&working_dir).unwrap().files;
        let filtered_jars: Vec<String> = all_jars
            .into_iter()
            .filter(|jar_full_path| {
                let name = jar_full_path
                    .split(std::path::MAIN_SEPARATOR)
                    .last()
                    .unwrap_or(jar_full_path);
                name.ends_with(".jar")
            })
            .collect();
        let default = format!(
            "-Djava.library.path={}",
            working_dir.join("lib").to_str().unwrap()
        );
        let opt = if cfg!(windows) {
            default
        } else {
            format!("{}:/usr/lib:/lib", default)
        };
        Self::create(working_dir, filtered_jars, vec![opt])
    }

    fn create_jvm<P: AsRef<Path>>(
        working_dir: P,
        jar_paths: Vec<String>,
        java_opts: Vec<String>,
    ) -> Jvm {
        let entries = jar_paths
            .iter()
            .map(|j| ClasspathEntry::new(j))
            .collect::<Vec<_>>();
        let opts = java_opts
            .iter()
            .map(|j| JavaOpt::new(j))
            .collect::<Vec<_>>();
        let jvm = if let Ok(jvm) = JvmBuilder::new()
            .classpath_entries(entries)
            .java_opts(opts)
            .with_base_path(working_dir.as_ref().to_str().unwrap())
            .build()
        {
            jvm
        } else {
            Jvm::attach_thread().expect("Jvm 创建失败！")
        };
        jvm
    }
    pub fn create<P: AsRef<Path>>(
        working_dir: P,
        jar_paths: Vec<String>,
        java_opts: Vec<String>,
    ) -> Self {
        let jvm = Self::create_jvm(&working_dir, jar_paths, java_opts);
        let instance = jvm
            .field(
                &jvm.static_class("net.mamoe.mirai.BotFactory").unwrap(),
                "INSTANCE",
            )
            .unwrap();
        let config = BotConfiguration::default();
        config.set_working_dir(working_dir.as_ref().to_str().unwrap());
        Self {
            instance,
            jvm,
            config,
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
    /// 在被挤下线时（[`BotOfflineEvent::Force`]）自动重连。默认为 false.
    /// 其他情况掉线都默认会自动重连，详见 [`BotOfflineEvent::reconnect`].
    pub fn auto_reconnect_on_force_offline(self) -> Self {
        self.config.auto_reconnect_on_force_offline();
        self
    }
    /// 禁止保存 account.secrets.
    /// 根据 Mirai 注释：
    ///     禁止保存 account.secrets.
    ///     account.secrets 保存账号的会话信息。
    ///     它可加速登录过程，也可能可以减少出现验证码的次数。
    ///     如果遇到一段时间后无法接收消息通知等同步问题时可尝试禁用。
    pub fn disable_account_secretes(self) -> Self {
        self.config.disable_account_secretes();
        self
    }
    /// 禁用好友列表和群成员列表的缓存。
    /// ~应该~**不**是默认行为。
    pub fn disable_contact_cache(self) -> Self {
        self.config.disable_contact_cache();
        self
    }
    /// 启用好友列表和群成员列表的缓存。
    /// ~应该~是默认行为。
    pub fn enable_contact_cache(self) -> Self {
        self.config.enable_contact_cache();
        self
    }
    /// 使用文件存储设备信息。
    /// 可以传入 None 参数，此时默认使用工作目录下的 device.json 文件。
    /// 在传入 None 参数的情况下，如果 device.json 文件不存在的话，
    /// Mirai 似乎会发出警告，然后随机生成一个设备信息。
    /// TODO: 测试传入参数的行为。
    pub fn file_based_device_info(self, path: Option<&PathBuf>) -> Self {
        self.config.file_based_device_info(path);
        self
    }
    /// 不显示 Bot 日志。不推荐。
    pub fn no_bot_log(self) -> Self {
        self.config.no_bot_log();
        self
    }
    /// 不显示网络日志。不推荐。
    pub fn no_network_log(self) -> Self {
        self.config.no_network_log();
        self
    }
    /// 使用随机设备信息。
    /// 注意该函数~应该~不会持久化当前随机信息。
    pub fn random_device_info(self) -> Self {
        self.config.random_device_info();
        self
    }
    /// 重定向 Bot 日志到指定目录。若目录不存在将会自动创建。
    pub fn redirect_bot_log_to_directory(
        self,
        path: Option<&PathBuf>,
        retain: Option<i64>,
        identity: Option<Instance>,
    ) -> Self {
        self.config
            .redirect_bot_log_to_directory(path, retain, identity);
        self
    }
    /// 重定向 Bot 日志到指定文件。日志将会逐行追加到此文件。若文件不存在将会自动创建。
    pub fn redirect_bot_log_to_file(
        self,
        path: Option<&PathBuf>,
        identity: Option<Instance>,
    ) -> Self {
        self.config.redirect_bot_log_to_file(path, identity);
        self
    }
    /// 重定向网络日志到指定目录。若目录不存在将会自动创建。
    /// 默认目录路径为工作目录下的 `logs/` 文件夹。
    pub fn redirect_network_log_to_directory(
        self,
        path: Option<&PathBuf>,
        retain: Option<i64>,
        identity: Option<Instance>,
    ) -> Self {
        self.config
            .redirect_network_log_to_directory(path, retain, identity);
        self
    }
    /// 重定向网络日志到指定文件。默认文件路径为工作目录下的 `mirai.log`.
    /// 日志将会逐行追加到此文件。若文件不存在将会自动创建。
    pub fn redirect_network_log_to_file(
        self,
        path: Option<&PathBuf>,
        identity: Option<Instance>,
    ) -> Self {
        self.config.redirect_network_log_to_file(path, identity);
        self
    }
    /// 使用协议类型。
    pub fn protocol(self, protocol: MiraiProtocol) -> Self {
        self.config.set_protocol(protocol);
        self
    }
    /// 设置工作目录。
    pub fn working_dir(self, path: &str) -> Self {
        self.config.set_working_dir(path);
        self
    }
    /// 日志记录器。
    pub fn bot_logger_supplier(self) -> Self {
        self.config.set_bot_logger_supplier();
        self
    }
    pub fn cache_dir(self, cache_dir: &str) -> Self {
        self.config.set_cache_dir(cache_dir);
        self
    }
    pub fn contact_list_cache(self, cache: ContactListCache) -> Self {
        self.config.set_contact_list_cache(cache);
        self
    }
    fn set_convert_line_separator(self, yes: bool) -> Self {
        self.config.set_convert_line_separator(yes);
        self
    }
    pub fn convert_line_separator(self) -> Self {
        self.set_convert_line_separator(true)
    }
    pub fn dont_convert_line_separator(self) -> Self {
        self.set_convert_line_separator(false)
    }
    pub fn device_info(self) -> Self {
        self.config.set_device_info();
        self
    }
    pub fn heartbeat_period_millis(self, millis: i64) -> Self {
        self.config.set_heartbeat_period_millis(millis);
        self
    }
    pub fn heartbeat_strategy(self, heartbeat_strategy: HeartbeatStrategy) -> Self {
        self.config.set_heartbeat_strategy(heartbeat_strategy);
        self
    }
    pub fn heartbeat_timeout_millis(self, millis: i64) -> Self {
        self.config.set_heartbeat_timeout_millis(millis);
        self
    }
    pub fn highway_upload_coroutine_count(self, count: i32) -> Self {
        self.config.set_highway_upload_coroutine_count(count);
        self
    }
    fn set_login_cache_enabled(self, yes: bool) -> Self {
        self.config.set_login_cache_enabled(yes);
        self
    }
    pub fn enable_login_cache(self) -> Self {
        self.set_login_cache_enabled(true)
    }
    pub fn disable_login_cache(self) -> Self {
        self.set_login_cache_enabled(false)
    }
    pub fn login_solver<T>(mut self, solver: T) -> Self
    where
        T: LoginSolverTrait,
    {
        self.config.set_login_solver(solver);
        self
    }
    pub fn network_logger_supplier(self) -> Self {
        self.config.set_network_logger_supplier();
        self
    }
    /// 最多尝试多少次重连。
    pub fn reconnection_retry_times(self, count: i32) -> Self {
        self.config.set_reconnection_retry_times(count);
        self
    }
    fn set_showing_verbose_event_log(self, yes: bool) -> Self {
        self.config.set_showing_verbose_event_log(yes);
        self
    }
    pub fn show_verbose_event_log(self) -> Self {
        self.set_showing_verbose_event_log(true)
    }
    pub fn dont_show_verbose_event_log(self) -> Self {
        self.set_showing_verbose_event_log(false)
    }
    pub fn stat_heartbeat_period_millis(self, millis: i64) -> Self {
        self.config.set_stat_heartbeat_period_millis(millis);
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
            let protocol = self.config.get_protocol();
            if protocol.is_qr_login_supported() {
                bot_authorization
            } else {
                panic!("当前协议 `{:?}` 不支持二维码登录！", protocol)
            }
        };
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
                    InvocationArg::try_from(self.config.get_instance()).unwrap(),
                ],
            )
            .unwrap();
        Bot::from_instance(bot)
    }
}
