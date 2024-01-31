use crate::message::data::super_face::SuperFace;
use crate::message::message_trait::{
    CodableMessageTrait, MessageContentTrait, MessageHashCodeTrait, MessageTrait,
    SingleMessageTrait,
};
use j4rs::{Instance, InvocationArg, Jvm};
use mjbase::env::FromInstance;
use mjmacro::GetInstanceDerive;
include!("face_res.rs");
#[derive(GetInstanceDerive)]
pub struct Face {
    name: String,
    id: i32,
    instance: Instance,
}

impl FromInstance for Face {
    fn from_instance(instance: Instance) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        Face {
            name: jvm
                .to_rust(jvm.invoke(&instance, "getName", &[]).unwrap())
                .unwrap(),
            id: jvm
                .to_rust(jvm.invoke(&instance, "getId", &[]).unwrap())
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
                "net.mamoe.mirai.message.data.Face",
                &[InvocationArg::try_from(id).unwrap()],
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
// pub trait SetFace<T> {
//     fn set(&mut self, face: T);
// }
// impl SetFace<i32> for Face {
//     fn set(&mut self, face: i32) {
//         self.id = face;
//         self.instance = Jvm::attach_thread()
//             .unwrap()
//             .create_instance("net.mamoe.mirai.message.data.Face", &[InvocationArg::try_from(self.id).unwrap()])
//             .unwrap();
//         self.name = Jvm::attach_thread()
//             .unwrap()
//             .to_rust(
//                 Jvm::attach_thread()
//                     .unwrap()
//                     .invoke(&self.instance, "getName", &[])
//                     .unwrap(),
//             )
//             .unwrap()
//     }
// }
// impl SetFace<FaceEnum> for Face {
//     fn set(&mut self, face: FaceEnum) {
//         self.name = format!("[{:?}]", face);
//         self.id = face.into();
//         self.instance = Jvm::attach_thread()
//             .unwrap()
//             .create_instance("class_name", &[InvocationArg::try_from(self.id).unwrap()])
//             .unwrap();
//     }
// }

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
