//TODO : message_chain_builder
use super::MessageChain;
use crate::message::AbsoluteFolder;
use crate::{contact::contact_trait::ContactTrait, env::GetEnvTrait};
use j4rs::{InvocationArg, Jvm};

pub trait MessageTrait
    where
        Self: GetEnvTrait,
{
    fn to_content(&self) -> String {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.get_instance(), "contentToString", &[])
                    .unwrap(),
            )
            .unwrap()
    }

    fn to_string(&self) -> String {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.get_instance(), "toString", &[])
                    .unwrap(),
            )
            .unwrap()
    }

    fn equals_to_message<T: MessageTrait>(
        &self,
        message: T,
        ignore_case: bool,
        strict: bool,
    ) -> bool {
        todo!()
    }

    fn equals_to_string<T: MessageTrait>(&self, message: T, ignore_case: bool) -> bool {
        todo!()
    }

    fn plus<T>(&self, message: T) -> MessageChain
        where
            T: MessageTrait,
    {
        // j4rs 旧版本中有 bug, 所以只能如下写法。见 https://github.com/astonbitecode/j4rs/issues/71
        // 再注：不是同一个 bug, 所以新版本还是要这么写。
        let jvm = Jvm::attach_thread().unwrap();
        let msg1 = InvocationArg::try_from(self.get_instance()).unwrap(); // j4rs <= 0.17.1
        let msg2 = InvocationArg::try_from(message.get_instance()).unwrap();
        let instance = jvm // j4rs <= 0.17.1
            .invoke_static("rt.lea.LumiaUtils", "callPlus", &[msg1, msg2]) // j4rs <= 0.17.1
            .unwrap(); // j4rs <= 0.17.1
        // let instance = jvm.invoke(&self.get_instance(), "plus", &[msg2]).unwrap(); // j4rs above 0.17.1
        MessageChain { instance }
    }
}

pub trait CodableMessageTrait: MessageTrait {
    fn to_code(&self) -> String {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.get_instance(), "serializeToMiraiCode", &[])
                    .unwrap(),
            )
            .unwrap()
    }
    fn append_code_to(&self) -> String {
        // TODO StringBuilder

        //  Jvm::attach_thread().unwrap()
        //     .to_rust(
        //          Jvm::attach_thread().unwrap()
        //             .invoke(&self.get_instance(), "serializeToMiraiCode", &[])
        //             .unwrap(),
        //     )
        //     .unwrap()
        todo!("StringBuilder")
    }
}

impl<Codable> MessageTrait for Codable
    where
        Codable: CodableMessageTrait,
{
    default fn to_content(&self) -> String {
        MessageTrait::to_content(self)
    }
    default fn to_string(&self) -> String {
        MessageTrait::to_string(self)
    }
    default fn equals_to_message<T: MessageTrait>(
        &self,
        message: T,
        ignore_case: bool,
        strict: bool,
    ) -> bool {
        MessageTrait::equals_to_message(self, message, ignore_case, strict)
    }
    default fn equals_to_string<T: MessageTrait>(&self, message: T, ignore_case: bool) -> bool {
        MessageTrait::equals_to_string(self, message, ignore_case)
    }
    default fn plus<T>(&self, message: T) -> MessageChain
        where
            T: MessageTrait,
    {
        MessageTrait::plus(self, message)
    }
}

pub trait SingleMessageTrait: MessageTrait {}

pub trait MessageChainTrait
    where
        Self: MessageTrait + CodableMessageTrait,
{
    fn deserialize_from_json(json: String) -> MessageChain {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static(
                "net.mamoe.mirai.message.data.MessageChain",
                "deserializeFromJsonString",
                &[InvocationArg::try_from(json).unwrap()],
            )
            .unwrap();
        MessageChain { instance }
    }
    fn deserialize_from_code<T: ContactTrait>(code: String, contact: T) -> MessageChain {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static(
                "net.mamoe.mirai.message.data.MessageChain",
                "deserializeFromMiraiCode",
                &[
                    InvocationArg::try_from(code).unwrap(),
                    InvocationArg::try_from(contact.get_instance()).unwrap(),
                ],
            )
            .unwrap();
        MessageChain { instance }
    }
    fn get(&self) // TODO -> impl SingleMessageTrait
    {
        todo!()
    }
    fn serialize_to_json_string(chain: &MessageChain) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static(
                "net.mamoe.mirai.message.data.MessageChain",
                "serializeToJsonString",
                &[InvocationArg::try_from(chain.get_instance()).unwrap()],
            )
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
    fn serialize_to_string(chain: &MessageChain) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static(
                "net.mamoe.mirai.message.data.MessageChain",
                "serializeToString",
                &[InvocationArg::try_from(chain.get_instance()).unwrap()],
            )
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
}

pub trait MessageContentTrait
    where
        Self: SingleMessageTrait,
{}

pub trait RichMessageTrait
    where
        Self: MessageContentTrait,
{
    fn get_key(&self) {
        todo!()
    }
    fn share(&self) -> () {}
}

pub trait ServiceMessageTrait
    where
        Self: RichMessageTrait + CodableMessageTrait,
{
    fn get_service_id(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        let id = jvm
            .invoke(&self.get_instance(), "getServiceId", &[])
            .unwrap();
        jvm.to_rust(id).unwrap()
    }
}

pub trait ConstrainSingleTrait
    where
        Self: SingleMessageTrait,
{
    fn get_key() -> () {
        todo!()
    }
    fn get_content() -> () {
        todo!("又是他妈的不知道哪儿来的")
    }
}

pub trait MarketFace
    where
        Self: ConstrainSingleTrait + MessageContentTrait,
{
    fn get_id(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("getId", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_key() -> () {
        todo!()
    }
    fn get_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("getName", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
}

pub trait MessageMetaDataTrait
    where
        Self: SingleMessageTrait,
{}

// impl MessageTrait for MessageMetaDataTrait
// {
//     fn to_content(&self) -> String {
//         String::new()
//     }
// }

/// TODO
pub trait CustomMessageTrait: SingleMessageTrait {
    //TODO
}

pub trait AudioTrait: SingleMessageTrait + ConstrainSingleTrait {}

pub trait MessageHashCodeTrait: GetEnvTrait {
    fn hash_code(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("hashCode", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
}

pub trait AbsoluteFileFloder: Sized + GetEnvTrait {
    fn delete(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("delete", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn exists(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("exists", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_absolute_path(&self) -> String // 这里应该是远程文件，所以先不用 PathBuf
    {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("getAbsolutePath", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_contact(&self) /*-> impl FileSupportedTrait */
    {
        todo!()
    }
    fn get_extension<T: AbsoluteFileFloder>(file_or_folder: T) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&file_or_folder.get_instance())
            .unwrap()
            .invoke("getExtension", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_id(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("getId", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_last_modified_time(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("getLastModifiedTime", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("getName", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_name_without_extension<T: AbsoluteFileFloder>(file_or_folder: T) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&file_or_folder.get_instance())
            .unwrap()
            .invoke("getNameWithoutExtension", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_parent(&self) -> AbsoluteFolder {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.get_instance(), "getParent", &[])
            .unwrap();
        AbsoluteFolder { instance }
    }
    fn get_upload_time(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("getUploadTime", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_upload_id(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("getUploadId", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn is_file(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("isFile", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn is_folder(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("isFolder", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn refresh(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("refresh", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn refreshed(&self) -> Self;
    fn rename_to(&self, name: &str) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("renameTo", &[InvocationArg::try_from(name).unwrap()])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn to_string(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("toString", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
}
