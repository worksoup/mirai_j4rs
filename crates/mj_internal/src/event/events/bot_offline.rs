use j4rs::{Instance, Jvm};

use mj_macro::mj_event;

use crate::event::{BotEventTrait, BotPassiveEventTrait};

/// 见 [`BotOfflineEvent`].
pub trait BotOfflineEventTrait: BotEventTrait {
    /// 为 `true` 时会尝试重连，仅 [`BotOfflineEvent::Force`] 默认为 `false`, 其他默认为 `true`.
    fn get_reconnect(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(self.as_instance(), "getReconnect", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
}
pub trait CauseAwareTrait {}
/// 主动离线。
///
/// 在调用 [`Bot::close`](crate::contact::Bot::close) 时，如果 [`Bot`](crate::contact::Bot) 连接正常，将会广播此事件。
///
/// 主动广播此事件也可以让 Bot 离线，但不建议这么做。
#[mj_event("event.events.BotOfflineEvent$Active")]
pub struct Active {
    instance: Instance,
}

impl BotOfflineEventTrait for Active {}
impl BotEventTrait for Active {}
/// 被挤下线。
///
/// 默认不会自动重连，在配置时调用
/// [`BotBuilder::auto_reconnect_on_force_offline`](crate::utils::bot_builder::BotBuilder::auto_reconnect_on_force_offline) 或
/// [`BotConfiguration::auto_reconnect_on_force_offline`](crate::utils::BotConfiguration::auto_reconnect_on_force_offline) 或
/// [`BotConfiguration::set_auto_reconnect_on_force_offline`](crate::utils::BotConfiguration::set_auto_reconnect_on_force_offline)
/// 来改变默认行为。
#[mj_event("event.events.BotOfflineEvent$Force")]
pub struct Force {
    instance: Instance,
}

impl BotOfflineEventTrait for Force {}
impl BotEventTrait for Force {}
impl BotPassiveEventTrait for Force {}

/// 被服务器断开。
#[mj_event("event.events.BotOfflineEvent$MsfOffline")]
pub struct MsfOffline {
    instance: Instance,
}

impl BotOfflineEventTrait for MsfOffline {}
impl BotEventTrait for MsfOffline {}
impl CauseAwareTrait for MsfOffline {}
impl BotPassiveEventTrait for MsfOffline {}

/// 因网络问题而掉线。
#[mj_event("event.events.BotOfflineEvent$Dropped")]
pub struct Dropped {
    instance: Instance,
}

impl BotOfflineEventTrait for Dropped {}
impl BotEventTrait for Dropped {}
impl BotPassiveEventTrait for Dropped {}

/// 服务器主动要求更换另一个服务器。
#[mj_event("event.events.BotOfflineEvent$RequireReconnect")]
pub struct RequireReconnect {
    instance: Instance,
}

impl BotOfflineEventTrait for RequireReconnect {}
impl BotEventTrait for RequireReconnect {}
impl CauseAwareTrait for RequireReconnect {}
impl BotPassiveEventTrait for RequireReconnect {}

/// [`Bot`](crate::contact::Bot) 离线时广播的事件。Bot 离线不会关闭 Bot, 只会关闭 Bot 的网络层。
#[mj_event]
pub enum BotOfflineEvent {
    /// 见 [`Active`].
    Active(Active),
    /// 见 [`Force`].
    Force(Force),
    /// 见 [`MsfOffline`].
    MsfOffline(MsfOffline),
    /// 见 [`Dropped`].
    Dropped(Dropped),
    /// 见 [`RequireReconnect`].
    RequireReconnect(RequireReconnect),
}

impl BotOfflineEventTrait for BotOfflineEvent {}
impl BotEventTrait for BotOfflineEvent {}
