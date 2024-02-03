use crate::message::message_trait::{
    AudioTrait, CodableMessageTrait, ConstrainSingleTrait, MessageContentTrait, MessageTrait,
    SingleMessageTrait,
};
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::{FromInstance, GetClassTypeTrait, GetInstanceTrait};
use mj_macro::{java_type, FromInstanceDerive, GetInstanceDerive};
use std::hint::unreachable_unchecked;

#[java_type("net.mamoe.mirai.message.data.Audio")]
pub enum Audio {
    OfflineAudio(OfflineAudio),
    OnlineAudio(OnlineAudio),
}
impl GetInstanceTrait for Audio {
    fn get_instance(&self) -> Instance {
        match self {
            Audio::OfflineAudio(a) => a.get_instance(),
            Audio::OnlineAudio(a) => a.get_instance(),
        }
    }
}
impl FromInstance for Audio {
    fn from_instance(instance: Instance) -> Self {
        if OfflineAudio::is_this_type(&instance) {
            Audio::OfflineAudio(OfflineAudio::from_instance(
                OfflineAudio::cast_to_this_type(instance),
            ))
        } else if OnlineAudio::is_this_type(&instance) {
            Audio::OnlineAudio(OnlineAudio::from_instance(OnlineAudio::cast_to_this_type(
                instance,
            )))
        } else {
            panic!("预料之外的错误：该语音既不是在线语音也不是离线语音。")
        }
    }
}
impl AudioTrait for Audio {}
impl SingleMessageTrait for Audio {}
impl MessageContentTrait for Audio {}
impl ConstrainSingleTrait for Audio {}
impl MessageTrait for Audio {}

#[derive(GetInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.message.data.OfflineAudio")]
pub struct OfflineAudio {
    instance: Instance,
}
impl OfflineAudio {
    pub fn new() -> Self {
        todo!()
    }
}

impl TryFrom<OnlineAudio> for OfflineAudio {
    // TODO: 需要合适的错误类型。
    type Error = ();

    fn try_from(online_audio: OnlineAudio) -> Result<Self, Self::Error> {
        let jvm = Jvm::attach_thread().unwrap();
        let online_audio = InvocationArg::try_from(online_audio.get_instance()).unwrap();
        let instance = jvm
            .invoke_static(
                "net.mamoe.mirai.message.data.OfflineAudio$Factory$INSTANCE",
                "",
                &[online_audio],
            )
            .unwrap();
        Ok(Self::from_instance(instance))
    }
}

impl SingleMessageTrait for OfflineAudio {}

impl MessageTrait for OfflineAudio {}

impl ConstrainSingleTrait for OfflineAudio {}

impl MessageContentTrait for OfflineAudio {}

impl AudioTrait for OfflineAudio {}

#[derive(GetInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.message.data.OnlineAudio")]
pub struct OnlineAudio {
    instance: Instance,
}
impl OnlineAudio {
    pub fn get_length(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = self.get_instance();
        let instance = jvm.invoke(&instance, "getLength", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn get_url_for_download(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = self.get_instance();
        let instance = jvm.invoke(&instance, "getUrlForDownload", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
}
impl AudioTrait for OnlineAudio {}
impl SingleMessageTrait for OnlineAudio {}
impl MessageContentTrait for OnlineAudio {}
impl ConstrainSingleTrait for OnlineAudio {}
impl MessageTrait for OnlineAudio {}
