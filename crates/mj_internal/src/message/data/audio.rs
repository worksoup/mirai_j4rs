use j4rs::{Instance, InvocationArg, Jvm};

use mj_base::env::{AsInstanceTrait, FromInstanceTrait, GetClassTypeTrait, GetInstanceTrait};
use mj_macro::mj_all;

use crate::message::message_trait::{
    AudioTrait, ConstrainSingleTrait, MessageContentTrait, MessageTrait, SingleMessageTrait,
};

#[mj_all("message.data.Audio")]
pub enum Audio {
    OnlineAudio(OnlineAudio),
    OfflineAudio(OfflineAudio),
}
impl AudioTrait for Audio {}
impl SingleMessageTrait for Audio {}
impl MessageContentTrait for Audio {}
impl ConstrainSingleTrait for Audio {}
impl MessageTrait for Audio {}

#[mj_all("message.data.OfflineAudio")]
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
                (<Self as GetClassTypeTrait>::get_type_name().to_string() + "$Factory$INSTANCE")
                    .as_str(),
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

#[mj_all("message.data.OnlineAudio")]
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
    pub fn get_url_for_download(&self) -> String {
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
