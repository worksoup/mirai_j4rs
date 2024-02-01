use crate::{
    contact::contact_trait::FileSupportedTrait,
    file::AbsoluteFile,
    message::message_trait::{
        CodableMessageTrait, ConstrainSingleTrait, MessageContentTrait, MessageTrait,
        SingleMessageTrait,
    },
};
use j4rs::{Instance, InvocationArg, Jvm};
use mj_macro::{FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, FromInstanceDerive)]
pub struct FileMessage {
    instance: Instance,
}

impl FileMessage {
    pub fn get_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getName", &[]).unwrap())
            .unwrap()
    }
    pub fn get_size(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getSize", &[]).unwrap())
            .unwrap()
    }
    pub fn get_file_id(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getId", &[]).unwrap())
            .unwrap()
    }
    pub fn get_internal_id(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getInternalId", &[]).unwrap())
            .unwrap()
    }
    pub fn new(file_id: String, internal_id: i32, name: String, size: i64) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let file_id = InvocationArg::try_from(&file_id).unwrap();
        let internal_id = InvocationArg::try_from(internal_id)
            .unwrap()
            .into_primitive()
            .unwrap();
        let name = InvocationArg::try_from(name).unwrap();
        let size = InvocationArg::try_from(size)
            .unwrap()
            .into_primitive()
            .unwrap();
        let instance = jvm
            .invoke_static(
                "net.mamoe.mirai.message.data.FileMessage",
                "create",
                &[file_id, internal_id, name, size],
            )
            .unwrap();
        FileMessage { instance }
    }
    pub fn to_absolute_file<FileSupported: FileSupportedTrait>(
        &self,
        contact: FileSupported,
    ) -> AbsoluteFile {
        let jvm = Jvm::attach_thread().unwrap();
        // let instance = InvocationArg::try_from(self.get_instance()).unwrap();
        let contact = InvocationArg::try_from(
            jvm.cast(
                &contact.get_instance(),
                "net.mamoe.mirai.contact.FileSupported",
            )
            .unwrap(),
        )
        .unwrap();
        let instance = jvm
            .invoke(&self.instance, "toAbsoluteFile", &[contact])
            .unwrap();
        // let instance = jvm
        //     .invoke_static(
        //         "rt.lea.LumiaUtils",
        //         "callToAbsoluteFile",
        //         &[instance, contact],
        //     )
        //     .unwrap();
        AbsoluteFile { instance }
    }
}

impl MessageTrait for FileMessage {}

impl SingleMessageTrait for FileMessage {}

impl MessageContentTrait for FileMessage {}

impl ConstrainSingleTrait for FileMessage {}

impl CodableMessageTrait for FileMessage {}
