use j4rs::{InvocationArg, Jvm};

use mj_base::env::{AsInstanceTrait, GetClassTypeTrait, GetInstanceTrait};
use mj_base::utils::primitive_byte_array_to_string;

use crate::{
    contact::ContactTrait,
    message::data::{MessageChain, SingleMessage},
};

//TODO : message_chain_builder
pub trait MessageTrait
where
    Self: GetInstanceTrait + AsInstanceTrait,
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
    // Mirai 中的实验性 api, 暂不提供。
    // fn append_code_to(&self) -> String {
    //     let jvm = Jvm::attach_thread().unwrap();
    //     jvm.to_rust(
    //         Jvm::attach_thread()
    //             .unwrap()
    //             .invoke(&self.get_instance(), "appendMiraiCodeTo", &[])
    //             .unwrap(),
    //     )
    //     .unwrap()
    // }
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
                <MessageChain as GetClassTypeTrait>::get_type_name().as_str(),
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
                <MessageChain as GetClassTypeTrait>::get_type_name().as_str(),
                "deserializeFromMiraiCode",
                &[
                    InvocationArg::try_from(code).unwrap(),
                    InvocationArg::try_from(contact.get_instance()).unwrap(),
                ],
            )
            .unwrap();
        MessageChain { instance }
    }
    fn get(&self /*, key: MessageKey*/) -> SingleMessage {
        todo!("MessageKey")
    }
    fn serialize_to_json_string(chain: &MessageChain) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static(
                <MessageChain as GetClassTypeTrait>::get_type_name().as_str(),
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
                <MessageChain as GetClassTypeTrait>::get_type_name().as_str(),
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
{
}

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
        todo!("该函数应为 RichMessage 的函数。")
    }
}

pub trait MarketFaceTrait
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
{
}

// impl MessageTrait for MessageMetaDataTrait
// {
//     fn to_content(&self) -> String {
//         String::new()
//     }
// }

pub trait CustomMessageTrait: SingleMessageTrait {
    //TODO
}

pub trait AudioTrait: MessageContentTrait + ConstrainSingleTrait {
    fn get_codec() {
        todo!()
    }
    fn get_extra_data(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = self.get_instance();
        let instance = jvm.invoke(&instance, "getExtraData", &[]).unwrap();
        let instance = primitive_byte_array_to_string(&jvm, instance);
        jvm.to_rust(instance).unwrap()
    }
    fn get_file_md5(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = self.get_instance();
        let instance = jvm.invoke(&instance, "getFileMd5", &[]).unwrap();
        let instance = primitive_byte_array_to_string(&jvm, instance);
        jvm.to_rust(instance).unwrap()
    }
    fn get_file_size(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = self.get_instance();
        let instance = jvm.invoke(&instance, "getFileSize", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
    fn get_file_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = self.get_instance();
        // 这里就是 Filename 而非 FileName.
        let instance = jvm.invoke(&instance, "getFilename", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
}

pub trait MessageHashCodeTrait: GetInstanceTrait {
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
