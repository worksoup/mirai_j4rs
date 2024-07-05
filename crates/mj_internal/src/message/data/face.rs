use j4rs::{Instance, InvocationArg, Jvm};

use mj_base::env::{FromInstanceTrait, GetClassTypeTrait};
use mj_macro::{java_type, AsInstanceDerive, GetInstanceDerive};

use crate::message::{
    data::super_face::SuperFace,
    message_trait::{
        CodableMessageTrait, MessageContentTrait, MessageHashCodeTrait, MessageTrait,
        SingleMessageTrait,
    },
};

include!("face_res.rs");
#[derive(AsInstanceDerive, GetInstanceDerive)]
#[java_type("message.data.Face")]
pub struct Face {
    name: String,
    id: i32,
    instance: Instance,
}

impl FromInstanceTrait for Face {
    fn from_instance(instance: Instance) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        Face {
            name: jvm
                .to_rust(
                    jvm.invoke(&instance, "getName", InvocationArg::empty())
                        .unwrap(),
                )
                .unwrap(),
            id: jvm
                .to_rust(
                    jvm.invoke(&instance, "getId", InvocationArg::empty())
                        .unwrap(),
                )
                .unwrap(),
            instance,
        }
    }
}

impl Face {
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl From<i32> for Face {
    fn from(id: i32) -> Self {
        let face = FaceEnum::from(id);
        Self::from(face)
    }
}

impl From<FaceEnum> for Face {
    fn from(face: FaceEnum) -> Self {
        let name = format!("[{:?}]", face);
        let id = face.into();
        let instance = Jvm::attach_thread()
            .unwrap()
            .create_instance(
                <Self as GetClassTypeTrait>::get_type_name().as_str(),
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        Face { name, id, instance }
    }
}

impl From<SuperFace> for Face {
    fn from(super_face: SuperFace) -> Self {
        super_face.get_face().into()
    }
}

impl MessageTrait for Face {
    fn to_content(&self) -> String {
        self.name.clone()
    }
    fn to_string(&self) -> String {
        self.to_content()
    }
}

impl CodableMessageTrait for Face {
    fn to_code(&self) -> String {
        format!("[mirai:face:{}]", self.id)
    }
}

impl SingleMessageTrait for Face {}

impl MessageContentTrait for Face {}

impl MessageHashCodeTrait for Face {}
