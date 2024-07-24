use crate::utils::backend::Overflow;
use std::marker::PhantomData;
use std::path::Path;

use j4rs::{ClasspathEntry, Instance, JavaOpt, Jvm, JvmBuilder};

use jbuchong::java_type;
use jbuchong::{AsInstanceTrait, GetClassTypeTrait, GetInstanceTrait, TryFromInstanceTrait};

use crate::utils::backend::{BotBackend, Mirai};
use crate::{
    auth::bot_authorization::BotAuthorization,
    utils::{login_solver::LoginSolverTrait, BotConfiguration},
};

/// # [`BotBuilder`]
/// 用来获得 [`Bot`].
///
/// 可以使用 [`default`](BotBuilder::default), [`new`](BotBuilder::new) 或 [`create`](BotBuilder::create)
/// 来获取该类型。
///
/// [`create`](crate::utils::bot_builder::BotBuilder::create) 通过 `working_dir`, `jar_paths` 和 `java_opts` 获得该类型。
///
/// [`new`](crate::utils::bot_builder::BotBuilder::new)  即 `BotBuilder::create(working_dir, vec![], vec![])`.
///
/// [`default`](crate::utils::bot_builder::BotBuilder::default) 即 `BotBuilder::new(".")`.
#[java_type("net.mamoe.mirai.BotFactory", Backend = Mirai)]
#[java_type("top.mrxiaom.overflow.BotBuilder", Backend = Overflow)]
pub struct BotBuilder<Backend: BotBackend> {
    instance: Instance,
    jvm: Jvm,
    config: Option<BotConfiguration>,
    id: Option<i64>,
    bot_authorization: Option<BotAuthorization>,
    _backend: PhantomData<Backend>,
}
impl Default for BotBuilder<Mirai> {
    fn default() -> Self {
        Self::new(".")
    }
}
impl<Backend: BotBackend> BotBuilder<Backend> {
    fn create_jvm<P: AsRef<Path>>(
        working_dir: P,
        jar_paths: &Vec<String>,
        java_opts: &Vec<String>,
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
}
pub use mirai_backend::*;
pub use overflow_backend::*;
mod mirai_backend {
    use crate::auth::bot_authorization::BotAuthorization;
    use crate::contact::Bot;
    use crate::utils::backend::Mirai;
    use crate::utils::bot_builder::BotBuilder;
    use crate::utils::login_solver::LoginSolverTrait;
    use crate::utils::other::enums::{HeartbeatStrategy, MiraiProtocol};
    use crate::utils::{BotConfiguration, ContactListCache};
    use j4rs::{Instance, InvocationArg, Jvm};
    use jbuchong::{AsInstanceTrait, GetClassTypeTrait, GetInstanceTrait, TryFromInstanceTrait};
    use std::marker::PhantomData;
    use std::path::{Path, PathBuf};

    impl BotBuilder<Mirai> {
        pub fn new<P: AsRef<Path>>(working_dir: P) -> Self {
            Self::create(working_dir, &vec![], &vec![])
        }
        pub fn create<P: AsRef<Path>>(
            working_dir: P,
            jar_paths: &Vec<String>,
            java_opts: &Vec<String>,
        ) -> Self {
            let working_dir = std::fs::canonicalize(working_dir).expect("目录无法解析。");
            let jvm = Self::create_jvm(&working_dir, jar_paths, java_opts);
            let instance = jvm
                .field(
                    &jvm.static_class(<Self as GetClassTypeTrait>::get_type_name())
                        .unwrap(),
                    "INSTANCE",
                )
                .unwrap();
            let config = BotConfiguration::default();
            config.set_working_dir(working_dir.to_str().unwrap());
            Self {
                instance,
                jvm,
                config: Some(config),
                id: None,
                bot_authorization: None,
                _backend: PhantomData,
            }
        }
        pub fn get_config(&self) -> &BotConfiguration {
            unsafe { self.config.as_ref().unwrap_unchecked() }
        }
    }
    impl BotBuilder<Mirai> {
        pub fn id(mut self, id: i64) -> Self {
            self.id = Some(id);
            self
        }
        pub fn authorization(mut self, bot_authorization: BotAuthorization) -> Self {
            self.bot_authorization = Some(bot_authorization);
            self
        }
        /// 在被挤下线时（[`BotOfflineEvent::Force`](crate::event::bot_offline::BotOfflineEvent::Force)）自动重连。默认为 false.
        /// 其他情况掉线都默认会自动重连，详见 [`BotOfflineEvent::get_reconnect`](crate::event::bot_offline::BotOfflineEvent).
        pub fn auto_reconnect_on_force_offline(self) -> Self {
            self.get_config().auto_reconnect_on_force_offline();
            self
        }
        /// 禁止保存 account.secrets.
        /// 根据 Mirai 注释：
        ///     禁止保存 account.secrets.
        ///     account.secrets 保存账号的会话信息。
        ///     它可加速登录过程，也可能可以减少出现验证码的次数。
        ///     如果遇到一段时间后无法接收消息通知等同步问题时可尝试禁用。
        pub fn disable_account_secretes(self) -> Self {
            self.get_config().disable_account_secretes();
            self
        }
        /// 禁用好友列表和群成员列表的缓存。
        /// ~应该~**不**是默认行为。
        pub fn disable_contact_cache(self) -> Self {
            self.get_config().disable_contact_cache();
            self
        }
        /// 启用好友列表和群成员列表的缓存。
        /// ~应该~是默认行为。
        pub fn enable_contact_cache(self) -> Self {
            self.get_config().enable_contact_cache();
            self
        }
        /// 使用文件存储设备信息。
        /// 可以传入 None 参数，此时默认使用工作目录下的 device.json 文件。
        /// 在传入 None 参数的情况下，如果 device.json 文件不存在的话，
        /// Mirai 似乎会发出警告，然后随机生成一个设备信息。
        /// TODO: 测试传入参数的行为。
        pub fn file_based_device_info(self, path: Option<&PathBuf>) -> Self {
            self.get_config().file_based_device_info(path);
            self
        }
        /// 不显示 Bot 日志。不推荐。
        pub fn no_bot_log(self) -> Self {
            self.get_config().no_bot_log();
            self
        }
        /// 不显示网络日志。不推荐。
        pub fn no_network_log(self) -> Self {
            self.get_config().no_network_log();
            self
        }
        /// 使用随机设备信息。
        /// 注意该函数~应该~不会持久化当前随机信息。
        pub fn random_device_info(self) -> Self {
            self.get_config().random_device_info();
            self
        }
        /// 重定向 Bot 日志到指定目录。若目录不存在将会自动创建。
        pub fn redirect_bot_log_to_directory(
            self,
            path: Option<&PathBuf>,
            retain: Option<i64>,
            identity: Option<Instance>,
        ) -> Self {
            self.get_config()
                .redirect_bot_log_to_directory(path, retain, identity);
            self
        }
        /// 重定向 Bot 日志到指定文件。日志将会逐行追加到此文件。若文件不存在将会自动创建。
        pub fn redirect_bot_log_to_file(
            self,
            path: Option<&PathBuf>,
            identity: Option<Instance>,
        ) -> Self {
            self.get_config().redirect_bot_log_to_file(path, identity);
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
            self.get_config()
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
            self.get_config()
                .redirect_network_log_to_file(path, identity);
            self
        }
        /// 使用协议类型。
        pub fn protocol(self, protocol: MiraiProtocol) -> Self {
            self.get_config().set_protocol(protocol);
            self
        }
        /// 设置工作目录。
        pub fn working_dir(self, path: &str) -> Self {
            self.get_config().set_working_dir(path);
            self
        }
        /// 日志记录器。
        pub fn bot_logger_supplier(self) -> Self {
            self.get_config().set_bot_logger_supplier();
            self
        }
        pub fn cache_dir(self, cache_dir: &str) -> Self {
            self.get_config().set_cache_dir(cache_dir);
            self
        }
        pub fn contact_list_cache(self, cache: ContactListCache) -> Self {
            self.get_config().set_contact_list_cache(cache);
            self
        }
        fn set_convert_line_separator(self, yes: bool) -> Self {
            self.get_config().set_convert_line_separator(yes);
            self
        }
        pub fn convert_line_separator(self) -> Self {
            self.set_convert_line_separator(true)
        }
        pub fn dont_convert_line_separator(self) -> Self {
            self.set_convert_line_separator(false)
        }
        pub fn device_info(self) -> Self {
            self.get_config().set_device_info();
            self
        }
        pub fn heartbeat_period_millis(self, millis: i64) -> Self {
            self.get_config().set_heartbeat_period_millis(millis);
            self
        }
        pub fn heartbeat_strategy(self, heartbeat_strategy: HeartbeatStrategy) -> Self {
            self.get_config().set_heartbeat_strategy(heartbeat_strategy);
            self
        }
        pub fn heartbeat_timeout_millis(self, millis: i64) -> Self {
            self.get_config().set_heartbeat_timeout_millis(millis);
            self
        }
        pub fn highway_upload_coroutine_count(self, count: i32) -> Self {
            self.get_config().set_highway_upload_coroutine_count(count);
            self
        }
        fn set_login_cache_enabled(self, yes: bool) -> Self {
            self.get_config().set_login_cache_enabled(yes);
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
            let mut config = unsafe { self.config.take().unwrap_unchecked() };
            config.set_login_solver(solver);
            self.config = Some(config);
            self
        }
        pub fn network_logger_supplier(self) -> Self {
            self.get_config().set_network_logger_supplier();
            self
        }
        /// 最多尝试多少次重连。
        pub fn reconnection_retry_times(self, count: i32) -> Self {
            self.get_config().set_reconnection_retry_times(count);
            self
        }
        fn set_showing_verbose_event_log(self, yes: bool) -> Self {
            self.get_config().set_showing_verbose_event_log(yes);
            self
        }
        pub fn show_verbose_event_log(self) -> Self {
            self.set_showing_verbose_event_log(true)
        }
        pub fn dont_show_verbose_event_log(self) -> Self {
            self.set_showing_verbose_event_log(false)
        }
        pub fn stat_heartbeat_period_millis(self, millis: i64) -> Self {
            self.get_config().set_stat_heartbeat_period_millis(millis);
            self
        }
        /// 一些额外的工作。
        /// 对于某些操作本库不可能完成，那么你可以通过此函数做一些额外的工作。
        /// 传入闭包的三个参数分别是
        /// `j4rs::Jvm`, Mirai 中的 `BotFactory` 类和 Mirai 中的 `BotConfiguration` 类。
        /// 你可以通过 j4rs 库来完成一些额外的工作。
        pub fn extra(self, extra: impl FnOnce(&Jvm, &Instance, &Instance)) -> Self {
            extra(&self.jvm, &self.instance, self.get_config().as_instance());
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
                let protocol = self.get_config().get_protocol();
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
                        InvocationArg::try_from(
                            unsafe { self.config.unwrap_unchecked() }.get_instance(),
                        )
                        .unwrap(),
                    ],
                )
                .unwrap();
            Bot::try_from_instance(bot).unwrap()
        }
    }
}
mod overflow_backend {
    use crate::utils::backend::Overflow;
    use crate::utils::bot_builder::BotBuilder;
    use j4rs::{Instance, InvocationArg, Jvm};
    use jbuchong::GetClassTypeTrait;
    use std::marker::PhantomData;
    use std::path::Path;

    impl BotBuilder<Overflow> {
        pub fn positive<P: AsRef<Path>>(working_dir: P, host: &str) -> Self {
            Self::positive_create(working_dir, &vec![], &vec![], host)
        }
        pub fn reversed<P: AsRef<Path>>(working_dir: P, port: i32) -> Self {
            Self::reversed_create(working_dir, &vec![], &vec![], port)
        }
        fn create(jvm: &Jvm) -> Instance {
            jvm.field(
                &jvm.static_class(<Self as GetClassTypeTrait>::get_type_name())
                    .unwrap(),
                "Companion",
            )
            .unwrap()
        }
        pub fn positive_create<P: AsRef<Path>>(
            working_dir: P,
            jar_paths: &Vec<String>,
            java_opts: &Vec<String>,
            host: &str,
        ) -> Self {
            let jvm = Self::create_jvm(&working_dir, jar_paths, java_opts);
            let instance = Self::create(&jvm);
            let instance = jvm
                .invoke(
                    &instance,
                    "positive",
                    &[InvocationArg::try_from(host).unwrap()],
                )
                .unwrap();
            Self {
                instance,
                jvm,
                config: None,
                id: None,
                bot_authorization: None,
                _backend: PhantomData,
            }
        }
        pub fn reversed_create<P: AsRef<Path>>(
            working_dir: P,
            jar_paths: &Vec<String>,
            java_opts: &Vec<String>,
            port: i32,
        ) -> Self {
            let jvm = Self::create_jvm(&working_dir, jar_paths, java_opts);
            let instance = Self::create(&jvm);
            let instance = jvm
                .invoke(
                    &instance,
                    "reversed",
                    &[InvocationArg::try_from(port)
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap();
            Self {
                instance,
                jvm,
                config: None,
                id: None,
                bot_authorization: None,
                _backend: PhantomData,
            }
        }
    }
    impl BotBuilder<Overflow> {
        
    }
}
