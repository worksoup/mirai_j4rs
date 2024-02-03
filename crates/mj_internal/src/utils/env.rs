use crate::auth::bot_authorization::BotAuthorization;
use crate::contact::Bot;
use crate::event::EventChannel;
use crate::utils::BotConfiguration;
use j4rs::{ClasspathEntry, Instance, InvocationArg, JavaOpt, Jvm, JvmBuilder};
use mj_base::env::{FromInstance, GetInstanceTrait};
use mj_base::utils::{instance_is_null, java_iter_to_rust_vec};
use serde::{Deserialize, Serialize};

pub struct Env {
    jvm: Jvm,
    instance: Instance,
}

#[derive(Deserialize, Serialize)]
pub struct EnvConfig {
    pub jar_paths: Vec<String>,
    pub java_opts: Vec<String>,
}

impl Env {
    pub fn new(env_config: &EnvConfig) -> Self {
        let EnvConfig {
            jar_paths,
            java_opts,
        } = env_config;
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
            .build()
        {
            jvm
        } else {
            Jvm::attach_thread().expect("Jvm 创建失败！")
        };
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
        BotConfiguration::from_instance(bot_configuration)
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
