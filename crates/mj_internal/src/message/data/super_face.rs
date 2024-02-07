use j4rs::{Instance, InvocationArg, Jvm};

use mj_base::env::GetClassTypeTrait;
use mj_base::{
    env::{FromInstanceTrait, GetClassTypeTrait as _, GetInstanceTrait as _},
    utils::instance_is_null,
};
use mj_macro::mj_all;

use crate::message::{
    data::face::Face,
    message_trait::{
        CodableMessageTrait, ConstrainSingleTrait, MessageContentTrait, MessageHashCodeTrait,
        MessageTrait, SingleMessageTrait,
    },
};

#[mj_all("message.data.SuperFace")]
pub struct SuperFace {
    instance: Instance,
}

impl SuperFace {
    fn new(face_id: i32, id: &str, r#type: i32) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let face_id = InvocationArg::try_from(face_id)
            .unwrap()
            .into_primitive()
            .unwrap();
        let id = InvocationArg::try_from(id).unwrap();
        let r#type = InvocationArg::try_from(r#type)
            .unwrap()
            .into_primitive()
            .unwrap();
        let instance = jvm
            .create_instance(
                <Self as GetClassTypeTrait>::get_type_name().as_str(),
                &[face_id, id, r#type],
            )
            .unwrap();
        Self { instance }
    }
    pub fn get_face(&self) -> Face {
        let jvm = Jvm::attach_thread().unwrap();
        let face_id = jvm.invoke(&self.instance, "getFace", &[]).unwrap();
        jvm.to_rust::<i32>(face_id).unwrap().into()
    }
    pub fn get_face_id(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        let face_id = jvm.invoke(&self.instance, "getFace", &[]).unwrap();
        jvm.to_rust(face_id).unwrap()
    }
    pub fn get_id(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let face_id = jvm.invoke(&self.instance, "getId", &[]).unwrap();
        jvm.to_rust(face_id).unwrap()
    }
    pub fn get_key(&self) -> () {}
    pub fn get_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let face_id = jvm.invoke(&self.instance, "getName", &[]).unwrap();
        jvm.to_rust(face_id).unwrap()
    }
    pub fn get_type(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        let face_id = jvm.invoke(&self.instance, "getType", &[]).unwrap();
        jvm.to_rust(face_id).unwrap()
    }
}

impl MessageHashCodeTrait for SuperFace {}

impl MessageTrait for SuperFace {}

impl ConstrainSingleTrait for SuperFace {}

impl SingleMessageTrait for SuperFace {}

impl MessageContentTrait for SuperFace {}

impl CodableMessageTrait for SuperFace {}

impl TryFrom<Face> for SuperFace {
    type Error = (); //TODO: 合适的错误类型。

    fn try_from(face: Face) -> Result<Self, Self::Error> {
        let jvm = Jvm::attach_thread().unwrap();
        let face = InvocationArg::try_from(face.get_instance()).unwrap();
        let instance = jvm
            .invoke_static(<Self as GetClassTypeTrait>::get_type_name().as_str(), "fromOrNull", &[face])
            .unwrap();
        if !instance_is_null(&instance) {
            Ok(SuperFace::from_instance(instance))
        } else {
            Err(())
        }
    }
}
