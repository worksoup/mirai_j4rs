use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{FromInstanceTrait, GetClassTypeTrait, GetInstanceTrait};
use mj_helper_macro::mj_all;

use crate::message::message_trait::{
    AudioTrait, ConstrainSingleTrait, MessageContentTrait, MessageTrait, SingleMessageTrait,
};
use crate::utils::backend::BotBackend;

#[mj_all("message.data.Audio")]
pub enum Audio <B: BotBackend>{
    OnlineAudio(OnlineAudio<B>),
    OfflineAudio(OfflineAudio<B>),
}
impl<B: BotBackend> AudioTrait<B> for Audio<B> {}
impl<B: BotBackend> SingleMessageTrait<B> for Audio<B> {}
impl<B: BotBackend> MessageContentTrait<B> for Audio<B> {}
impl<B: BotBackend> ConstrainSingleTrait<B> for Audio<B> {}
impl<B: BotBackend> MessageTrait<B> for Audio<B> {}

#[mj_all("message.data.OfflineAudio")]
pub struct OfflineAudio<B: BotBackend> {
    instance: Instance,
    _backend: B,
}
impl<B: BotBackend> OfflineAudio<B> {
    pub fn new() -> Self {
        todo!()
    }
}
impl<B: BotBackend> Default for OfflineAudio<B> {
    fn default() -> Self {
        Self::new()
    }
}

impl<B: BotBackend> TryFrom<OnlineAudio<B>> for OfflineAudio<B> {
    // TODO: 需要合适的错误类型。
    type Error = ();

    fn try_from(online_audio: OnlineAudio<B>) -> Result<Self, Self::Error> {
        let jvm = Jvm::attach_thread().unwrap();
        let online_audio = InvocationArg::try_from(online_audio.get_instance()).unwrap();
        let instance = jvm
            .invoke_static(
                (<Self as GetClassTypeTrait>::get_type_name().to_string() + "$Factory$INSTANCE")
                    .as_str(),
                "",
                &[online_audio],
            )
            .unwrap();
        Ok(Self::from_instance(instance))
    }
}

impl<B: BotBackend> SingleMessageTrait<B> for OfflineAudio<B> {}

impl<B: BotBackend> MessageTrait<B> for OfflineAudio<B> {}

impl<B: BotBackend> ConstrainSingleTrait<B> for OfflineAudio<B> {}

impl<B: BotBackend> MessageContentTrait<B> for OfflineAudio<B> {}

impl<B: BotBackend> AudioTrait<B> for OfflineAudio<B> {}

#[mj_all("message.data.OnlineAudio")]
pub struct OnlineAudio<B: BotBackend> {
    instance: Instance,
    _backend: B,
}
impl<B: BotBackend> OnlineAudio<B> {
    pub fn get_length(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = self.get_instance().unwrap();
        let instance = jvm
            .invoke(&instance, "getLength", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn get_url_for_download(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = self.get_instance().unwrap();
        let instance = jvm
            .invoke(&instance, "getUrlForDownload", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
}
impl<B: BotBackend> AudioTrait<B> for OnlineAudio<B> {}
impl<B: BotBackend> SingleMessageTrait<B> for OnlineAudio<B> {}
impl<B: BotBackend> MessageContentTrait<B> for OnlineAudio<B> {}
impl<B: BotBackend> ConstrainSingleTrait<B> for OnlineAudio<B> {}
impl<B: BotBackend> MessageTrait<B> for OnlineAudio<B> {}
