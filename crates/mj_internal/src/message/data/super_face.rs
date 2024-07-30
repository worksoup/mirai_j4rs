use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{
    utils::instance_is_null, FromInstanceTrait, GetClassTypeTrait, GetInstanceTrait as _,
};
use mj_helper_macro::mj_all;

use crate::message::{
    data::face::Face,
    message_trait::{
        CodableMessageTrait, ConstrainSingleTrait, MessageContentTrait, MessageHashCodeTrait,
        MessageTrait, SingleMessageTrait,
    },
};
use crate::utils::backend::BotBackend;

#[mj_all("message.data.SuperFace")]
pub struct SuperFace<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> SuperFace<B> {
    fn _new(face_id: i32, id: &str, r#type: i32) -> Self {
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
                <Self as GetClassTypeTrait>::get_type_name(),
                &[face_id, id, r#type],
            )
            .unwrap();
        Self {
            instance,
            _backend: B::default(),
        }
    }
    pub fn get_face(&self) -> Face<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let face_id = jvm
            .invoke(&self.instance, "getFace", InvocationArg::empty())
            .unwrap();
        jvm.to_rust::<i32>(face_id).unwrap().into()
    }
    pub fn get_face_id(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        let face_id = jvm
            .invoke(&self.instance, "getFace", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(face_id).unwrap()
    }
    pub fn get_id(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let face_id = jvm
            .invoke(&self.instance, "getId", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(face_id).unwrap()
    }
    pub fn get_key(&self) {}
    pub fn get_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let face_id = jvm
            .invoke(&self.instance, "getName", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(face_id).unwrap()
    }
    pub fn get_type(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        let face_id = jvm
            .invoke(&self.instance, "getType", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(face_id).unwrap()
    }
}

impl<B: BotBackend> MessageHashCodeTrait for SuperFace<B> {}

impl<B: BotBackend> MessageTrait<B> for SuperFace<B> {}

impl<B: BotBackend> ConstrainSingleTrait<B> for SuperFace<B> {}

impl<B: BotBackend> SingleMessageTrait<B> for SuperFace<B> {}

impl<B: BotBackend> MessageContentTrait<B> for SuperFace<B> {}

impl<B: BotBackend> CodableMessageTrait<B> for SuperFace<B> {}

impl<B: BotBackend> TryFrom<Face<B>> for SuperFace<B> {
    type Error = (); //TODO: 合适的错误类型。

    fn try_from(face: Face<B>) -> Result<Self, Self::Error> {
        let jvm = Jvm::attach_thread().unwrap();
        let face = InvocationArg::try_from(face.get_instance()).unwrap();
        let instance = jvm
            .invoke_static(
                <Self as GetClassTypeTrait>::get_type_name(),
                "fromOrNull",
                &[face],
            )
            .unwrap();
        if !instance_is_null(&instance) {
            Ok(SuperFace::from_instance(instance))
        } else {
            Err(())
        }
    }
}
