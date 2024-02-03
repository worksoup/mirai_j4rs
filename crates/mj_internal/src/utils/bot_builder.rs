use crate::event::bot::BotOfflineEvent;
use crate::{
    auth::bot_authorization::BotAuthorization,
    contact::Bot,
    utils::{
        login_solver::LoginSolverTrait,
        other::enums::{HeartbeatStrategy, MiraiProtocol},
        BotConfiguration, ContactListCache, Env, EnvConfig,
    },
};
use j4rs::Instance;
use std::path::PathBuf;

/// bot builder
pub struct BotBuilder {
    pub env: Env,
    pub config: Option<BotConfiguration>,
    id: Option<i64>,
    bot_authorization: Option<BotAuthorization>,
}

impl BotBuilder {
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
    pub fn new(env_config: &EnvConfig) -> Self {
        let env = Env::new(env_config);
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
    /// 在被挤下线时（[`BotOfflineEvent::Force`]）自动重连。默认为 false.
    /// 其他情况掉线都默认会自动重连，详见 [`BotOfflineEvent::reconnect`].
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
    pub fn protocol(mut self, protocol: MiraiProtocol) -> Self {
        if let Some(config) = &self.config {
            config.set_protocol(protocol);
        } else {
            let config = self.env.new_bot_configuration();
            config.set_protocol(protocol);
            self.config = Some(config);
        }
        self
    }
    /// 设置工作目录。
    pub fn working_dir(mut self, path: &str) -> Self {
        if let Some(config) = &self.config {
            config.set_working_dir(path);
        } else {
            let config = self.env.new_bot_configuration();
            config.set_working_dir(path);
            self.config = Some(config);
        }
        self
    }
    /// 日志记录器。
    pub fn bot_logger_supplier(mut self) -> Self {
        if let Some(config) = &self.config {
            config.set_bot_logger_supplier();
        } else {
            let config = self.env.new_bot_configuration();
            config.set_bot_logger_supplier();
            self.config = Some(config);
        }
        self
    }
    pub fn cache_dir(mut self, cache_dir: &str) -> Self {
        if let Some(config) = &self.config {
            config.set_cache_dir(cache_dir);
        } else {
            let config = self.env.new_bot_configuration();
            config.set_cache_dir(cache_dir);
            self.config = Some(config);
        }
        self
    }
    pub fn contact_list_cache(mut self, cache: ContactListCache) -> Self {
        if let Some(config) = &self.config {
            config.set_contact_list_cache(cache);
        } else {
            let config = self.env.new_bot_configuration();
            config.set_contact_list_cache(cache);
            self.config = Some(config);
        }
        self
    }
    fn set_convert_line_separator(mut self, yes: bool) -> Self {
        if let Some(config) = &self.config {
            config.set_convert_line_separator(yes);
        } else {
            let config = self.env.new_bot_configuration();
            config.set_convert_line_separator(yes);
            self.config = Some(config);
        }
        self
    }
    pub fn convert_line_separator(self) -> Self {
        self.set_convert_line_separator(true)
    }
    pub fn dont_convert_line_separator(self) -> Self {
        self.set_convert_line_separator(false)
    }
    pub fn device_info(mut self) -> Self {
        if let Some(config) = &self.config {
            config.set_device_info();
        } else {
            let config = self.env.new_bot_configuration();
            config.set_device_info();
            self.config = Some(config);
        }
        self
    }
    pub fn heartbeat_period_millis(mut self, millis: i64) -> Self {
        if let Some(config) = &self.config {
            config.set_heartbeat_period_millis(millis);
        } else {
            let config = self.env.new_bot_configuration();
            config.set_heartbeat_period_millis(millis);
            self.config = Some(config);
        }
        self
    }
    pub fn heartbeat_strategy(mut self, heartbeat_strategy: HeartbeatStrategy) -> Self {
        if let Some(config) = &self.config {
            config.set_heartbeat_strategy(heartbeat_strategy);
        } else {
            let config = self.env.new_bot_configuration();
            config.set_heartbeat_strategy(heartbeat_strategy);
            self.config = Some(config);
        }
        self
    }
    pub fn heartbeat_timeout_millis(mut self, millis: i64) -> Self {
        if let Some(config) = &self.config {
            config.set_heartbeat_timeout_millis(millis);
        } else {
            let config = self.env.new_bot_configuration();
            config.set_heartbeat_timeout_millis(millis);
            self.config = Some(config);
        }
        self
    }
    pub fn highway_upload_coroutine_count(mut self, count: i32) -> Self {
        if let Some(config) = &self.config {
            config.set_highway_upload_coroutine_count(count);
        } else {
            let config = self.env.new_bot_configuration();
            config.set_highway_upload_coroutine_count(count);
            self.config = Some(config);
        }
        self
    }
    fn set_login_cache_enabled(mut self, yes: bool) -> Self {
        if let Some(config) = &self.config {
            config.set_login_cache_enabled(yes);
        } else {
            let config = self.env.new_bot_configuration();
            config.set_login_cache_enabled(yes);
            self.config = Some(config);
        }
        self
    }
    fn enable_login_cache(self) -> Self {
        self.set_login_cache_enabled(true)
    }
    fn disable_login_cache(self) -> Self {
        self.set_login_cache_enabled(false)
    }
    pub fn login_solver<T>(mut self, solver: T) -> Self
    where
        T: LoginSolverTrait,
    {
        let config = &mut self.config;
        if let Some(config) = config.as_mut() {
            config.set_login_solver(solver);
        } else {
            let mut config = self.env.new_bot_configuration();
            config.set_login_solver(solver);
            self.config = Some(config);
        }
        self
    }
    pub fn network_logger_supplier(mut self) -> Self {
        if let Some(config) = &self.config {
            config.set_network_logger_supplier();
        } else {
            let config = self.env.new_bot_configuration();
            config.set_network_logger_supplier();
            self.config = Some(config);
        }
        self
    }
    /// 最多尝试多少次重连。
    pub fn reconnection_retry_times(mut self, count: i32) -> Self {
        if let Some(config) = &self.config {
            config.set_reconnection_retry_times(count);
        } else {
            let config = self.env.new_bot_configuration();
            config.set_reconnection_retry_times(count);
            self.config = Some(config);
        }
        self
    }
    fn set_showing_verbose_event_log(mut self, yes: bool) -> Self {
        if let Some(config) = &self.config {
            config.set_showing_verbose_event_log(yes);
        } else {
            let config = self.env.new_bot_configuration();
            config.set_showing_verbose_event_log(yes);
            self.config = Some(config);
        }
        self
    }
    pub fn show_verbose_event_log(self) -> Self {
        self.set_showing_verbose_event_log(true)
    }
    pub fn dont_show_verbose_event_log(self) -> Self {
        self.set_showing_verbose_event_log(false)
    }
    pub fn stat_heartbeat_period_millis(mut self, millis: i64) -> Self {
        if let Some(config) = &self.config {
            config.set_stat_heartbeat_period_millis(millis);
        } else {
            let config = self.env.new_bot_configuration();
            config.set_stat_heartbeat_period_millis(millis);
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
