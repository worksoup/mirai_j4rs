use j4rs::{Instance, InvocationArg, Jvm};

use mj_base::env::{FromInstanceTrait, GetClassTypeTrait};
use mj_base::utils::instance_is_null;
use mj_base::MIRAI_PREFIX;
use mj_macro::mj_all;

use crate::utils::contact::file::AbsoluteFile;
use crate::{
    contact::FileSupportedTrait,
    message::message_trait::{
        CodableMessageTrait, ConstrainSingleTrait, MessageContentTrait, MessageTrait,
        SingleMessageTrait,
    },
};

///  # 文件消息。
///
///  注: [`FileMessage`] 不可二次发送，包括转发消息。
///
///  ## 文件操作
///  要下载这个文件, 可通过 [`to_absolute_file`](FileMessage::to_absolute_file) 获取到 [`AbsoluteFile`] 然后操作。
///
///  要获取到 [`FileMessage`]，可以通过 [`MessageEvent`](crate::event::MessageEventTrait) 获取，或通过 [`AbsoluteFile::to_message`] 得到。
// TODO: 实现 SendSupportedTrait, 限制某些消息的发送。
#[mj_all("message.data.FileMessage")]
pub struct FileMessage {
    instance: Instance,
}

impl FileMessage {
    /// 获取文件名。
    pub fn get_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getName", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    /// 获取文件大小。单位为字节。
    pub fn get_size(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getSize", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn get_file_id(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getId", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn get_internal_id(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getInternalId", InvocationArg::empty())
                .unwrap(),
        )
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
                <Self as GetClassTypeTrait>::get_type_name().as_str(),
                "create",
                &[file_id, internal_id, name, size],
            )
            .unwrap();
        FileMessage { instance }
    }
    /// 获取一个对应的 [`AbsoluteFile`]. 当目标群或好友不存在这个文件时返回 `None`.
    pub fn to_absolute_file<FileSupported: FileSupportedTrait>(
        &self,
        contact: FileSupported,
    ) -> Option<AbsoluteFile> {
        let jvm = Jvm::attach_thread().unwrap();
        // let instance = InvocationArg::try_from(self.get_instance()).unwrap();
        let contact = InvocationArg::try_from(
            jvm.cast(
                &contact.get_instance(),
                (MIRAI_PREFIX.to_string() + "contact.FileSupported").as_str(),
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
        //
        if instance_is_null(&instance) {
            None
        } else {
            Some(AbsoluteFile::from_instance(instance))
        }
    }
}

impl MessageTrait for FileMessage {}

impl SingleMessageTrait for FileMessage {}

impl MessageContentTrait for FileMessage {}

impl ConstrainSingleTrait for FileMessage {}

impl CodableMessageTrait for FileMessage {}
