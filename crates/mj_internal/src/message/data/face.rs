use crate::message::{
    data::super_face::SuperFace,
    message_trait::{
        CodableMessageTrait, MessageContentTrait, MessageHashCodeTrait, MessageTrait,
        SingleMessageTrait,
    },
};
use crate::utils::backend::BotBackend;
use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::GetClassTypeTrait;
use mj_helper_macro::mj_all;

include!("face_res.rs");
#[mj_all("message.data.Face")]
pub struct Face<B: BotBackend> {
    #[default(fn_name=get_name_)]
    name: String,
    #[default(fn_name=get_id_)]
    id: i32,
    instance: Instance,
    _backend: B,
}
fn get_name_(instance: &Instance) -> String {
    let jvm = Jvm::attach_thread().unwrap();
    jvm.to_rust(
        jvm.invoke(instance, "getName", InvocationArg::empty())
            .unwrap(),
    )
    .unwrap()
}

fn get_id_(instance: &Instance) -> i32 {
    let jvm = Jvm::attach_thread().unwrap();
    jvm.to_rust(
        jvm.invoke(instance, "getId", InvocationArg::empty())
            .unwrap(),
    )
    .unwrap()
}

impl<B: BotBackend> Face<B> {
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl<B: BotBackend> From<i32> for Face<B> {
    fn from(id: i32) -> Self {
        let face = FaceEnum::from(id);
        Self::from(face)
    }
}

impl<B: BotBackend> From<FaceEnum> for Face<B> {
    fn from(face: FaceEnum) -> Self {
        let name = format!("[{:?}]", face);
        let id = face.into();
        let instance = Jvm::attach_thread()
            .unwrap()
            .create_instance(
                <Self as GetClassTypeTrait>::get_type_name(),
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        Face {
            name,
            id,
            instance,
            _backend: B::default(),
        }
    }
}

impl<B: BotBackend> From<SuperFace<B>> for Face<B> {
    fn from(super_face: SuperFace<B>) -> Self {
        super_face.get_face()
    }
}

impl<B: BotBackend> MessageTrait<B> for Face<B> {
    fn to_content(&self) -> String {
        self.name.clone()
    }
    fn to_string(&self) -> String {
        MessageTrait::<B>::to_content(self)
    }
}

impl<B: BotBackend> CodableMessageTrait<B> for Face<B> {
    fn to_code(&self) -> String {
        format!("[mirai:face:{}]", self.id)
    }
}

impl<B: BotBackend> SingleMessageTrait<B> for Face<B> {}

impl<B: BotBackend> MessageContentTrait<B> for Face<B> {}

impl<B: BotBackend> MessageHashCodeTrait for Face<B> {}
