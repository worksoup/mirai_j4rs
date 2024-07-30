use j4rs::{Instance, InvocationArg, Jvm};
use mj_helper_macro::mj_event;

use crate::event::{BotEventTrait, BotPassiveEventTrait};
use crate::utils::backend::BotBackend;

/// 见 [`BotOfflineEvent`].
pub trait BotOfflineEventTrait<B: BotBackend>: BotEventTrait<B> {
    /// 为 `true` 时会尝试重连，仅 [`BotOfflineEvent::Force`] 默认为 `false`, 其他默认为 `true`.
    fn get_reconnect(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(self.as_instance(), "getReconnect", InvocationArg::empty())
            .unwrap();
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
pub struct Active<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> BotOfflineEventTrait<B> for Active<B> {}
impl<B: BotBackend> BotEventTrait<B> for Active<B> {}
/// 被挤下线。
///
/// 默认不会自动重连，在配置时调用
/// [`BotBuilder::auto_reconnect_on_force_offline`](crate::utils::bot_builder::BotBuilder::auto_reconnect_on_force_offline) 或
/// [`BotConfiguration::auto_reconnect_on_force_offline`](crate::utils::BotConfiguration::auto_reconnect_on_force_offline) 或
/// [`BotConfiguration::set_auto_reconnect_on_force_offline`](crate::utils::BotConfiguration::set_auto_reconnect_on_force_offline)
/// 来改变默认行为。
#[mj_event("event.events.BotOfflineEvent$Force")]
pub struct Force<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> BotOfflineEventTrait<B> for Force<B> {}
impl<B: BotBackend> BotEventTrait<B> for Force<B> {}
impl<B: BotBackend> BotPassiveEventTrait<B> for Force<B> {}

/// 被服务器断开。
#[mj_event("event.events.BotOfflineEvent$MsfOffline")]
pub struct MsfOffline<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> BotOfflineEventTrait<B> for MsfOffline<B> {}
impl<B: BotBackend> BotEventTrait<B> for MsfOffline<B> {}
impl<B: BotBackend> CauseAwareTrait for MsfOffline<B> {}
impl<B: BotBackend> BotPassiveEventTrait<B> for MsfOffline<B> {}

/// 因网络问题而掉线。
#[mj_event("event.events.BotOfflineEvent$Dropped")]
pub struct Dropped<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> BotOfflineEventTrait<B> for Dropped<B> {}
impl<B: BotBackend> BotEventTrait<B> for Dropped<B> {}
impl<B: BotBackend> BotPassiveEventTrait<B> for Dropped<B> {}

/// 服务器主动要求更换另一个服务器。
#[mj_event("event.events.BotOfflineEvent$RequireReconnect")]
pub struct RequireReconnect<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> BotOfflineEventTrait<B> for RequireReconnect<B> {}
impl<B: BotBackend> BotEventTrait<B> for RequireReconnect<B> {}
impl<B: BotBackend> CauseAwareTrait for RequireReconnect<B> {}
impl<B: BotBackend> BotPassiveEventTrait<B> for RequireReconnect<B> {}

/// [`Bot`](crate::contact::Bot) 离线时广播的事件。Bot 离线不会关闭 Bot, 只会关闭 Bot 的网络层。
#[mj_event]
pub enum BotOfflineEvent<B: BotBackend> {
    /// 见 [`Active`].
    Active(Active<B>),
    /// 见 [`Force`].
    Force(Force<B>),
    /// 见 [`MsfOffline`].
    MsfOffline(MsfOffline<B>),
    /// 见 [`Dropped`].
    Dropped(Dropped<B>),
    /// 见 [`RequireReconnect`].
    RequireReconnect(RequireReconnect<B>),
}

impl<B: BotBackend> BotOfflineEventTrait<B> for BotOfflineEvent<B> {}
impl<B: BotBackend> BotEventTrait<B> for BotOfflineEvent<B> {}
