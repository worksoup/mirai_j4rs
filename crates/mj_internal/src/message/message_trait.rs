use j4rs::{InvocationArg, Jvm};

use jbuchong::utils::primitive_byte_array_to_string;
use jbuchong::{AsInstanceTrait, FromInstanceTrait, GetClassTypeTrait, GetInstanceTrait};

use crate::utils::backend::BotBackend;
use crate::{
    contact::ContactTrait,
    message::data::{MessageChain, SingleMessage},
};

//TODO : message_chain_builder
pub trait MessageTrait<B: BotBackend>
where
    Self: GetInstanceTrait + AsInstanceTrait,
{
    fn to_content(&self) -> String {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(
                        &self.get_instance().unwrap(),
                        "contentToString",
                        InvocationArg::empty(),
                    )
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
                    .invoke(
                        &self.get_instance().unwrap(),
                        "toString",
                        InvocationArg::empty(),
                    )
                    .unwrap(),
            )
            .unwrap()
    }

    fn equals_to_message<T: MessageTrait<B>>(
        &self,
        _message: T,
        _ignore_case: bool,
        _strict: bool,
    ) -> bool {
        todo!()
    }

    fn equals_to_string<T: MessageTrait<B>>(&self, _message: T, _ignore_case: bool) -> bool {
        todo!()
    }

    fn plus<T>(&self, message: T) -> MessageChain<B>
    where
        T: MessageTrait<B>,
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
        MessageChain::from_instance(instance)
    }
}

pub trait CodableMessageTrait<B: BotBackend>: MessageTrait<B> {
    fn to_code(&self) -> String {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(
                        &self.get_instance().unwrap(),
                        "serializeToMiraiCode",
                        InvocationArg::empty(),
                    )
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
    //             .invoke(&self.get_instance(), "appendMiraiCodeTo", InvocationArg::empty())
    //             .unwrap(),
    //     )
    //     .unwrap()
    // }
}

pub trait SingleMessageTrait<B: BotBackend>: MessageTrait<B> {}

pub trait MessageChainTrait<B: BotBackend>
where
    Self: MessageTrait<B> + CodableMessageTrait<B>,
{
    fn deserialize_from_json(json: String) -> MessageChain<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static(
                <MessageChain<B> as GetClassTypeTrait>::get_type_name(),
                "deserializeFromJsonString",
                &[InvocationArg::try_from(json).unwrap()],
            )
            .unwrap();
        MessageChain::from_instance(instance)
    }
    fn deserialize_from_code<T: ContactTrait<B>>(code: String, contact: T) -> MessageChain<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static(
                <MessageChain<B> as GetClassTypeTrait>::get_type_name(),
                "deserializeFromMiraiCode",
                &[
                    InvocationArg::try_from(code).unwrap(),
                    InvocationArg::try_from(contact.get_instance()).unwrap(),
                ],
            )
            .unwrap();
        MessageChain::from_instance(instance)
    }
    fn get(&self /*, key: MessageKey*/) -> SingleMessage<B> {
        todo!("MessageKey")
    }
    fn serialize_to_json_string(chain: &MessageChain<B>) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static(
                <MessageChain<B> as GetClassTypeTrait>::get_type_name(),
                "serializeToJsonString",
                &[InvocationArg::try_from(chain.get_instance()).unwrap()],
            )
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
    fn serialize_to_string(chain: &MessageChain<B>) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static(
                <MessageChain<B> as GetClassTypeTrait>::get_type_name(),
                "serializeToString",
                &[InvocationArg::try_from(chain.get_instance()).unwrap()],
            )
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
}

pub trait MessageContentTrait<B: BotBackend>
where
    Self: SingleMessageTrait<B>,
{
}

pub trait RichMessageTrait<B: BotBackend>
where
    Self: MessageContentTrait<B>,
{
    fn get_key(&self) {
        todo!()
    }
    fn share(&self) {}
}

pub trait ServiceMessageTrait<B: BotBackend>
where
    Self: RichMessageTrait<B> + CodableMessageTrait<B>,
{
    fn get_service_id(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        let id = jvm
            .invoke(
                &self.get_instance().unwrap(),
                "getServiceId",
                InvocationArg::empty(),
            )
            .unwrap();
        jvm.to_rust(id).unwrap()
    }
}

pub trait ConstrainSingleTrait<B: BotBackend>
where
    Self: SingleMessageTrait<B>,
{
    fn get_key() {
        todo!()
    }
    fn get_content() {
        todo!("该函数应为 RichMessage 的函数。")
    }
}

pub trait MarketFaceTrait<B: BotBackend>
where
    Self: ConstrainSingleTrait<B> + MessageContentTrait<B>,
{
    fn get_id(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance().unwrap())
            .unwrap()
            .invoke("getId", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_key() {
        todo!()
    }
    fn get_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance().unwrap())
            .unwrap()
            .invoke("getName", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
    }
}

pub trait MessageMetaDataTrait<B: BotBackend>
where
    Self: SingleMessageTrait<B>,
{
}

// impl MessageTrait for MessageMetaDataTrait
// {
//     fn to_content(&self) -> String {
//         String::new()
//     }
// }

pub trait CustomMessageTrait<B: BotBackend>: SingleMessageTrait<B> {
    //TODO
}

pub trait AudioTrait<B: BotBackend>: MessageContentTrait<B> + ConstrainSingleTrait<B> {
    fn get_codec() {
        todo!()
    }
    fn get_extra_data(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = self.get_instance().unwrap();
        let instance = jvm
            .invoke(&instance, "getExtraData", InvocationArg::empty())
            .unwrap();
        let instance = primitive_byte_array_to_string(&jvm, instance);
        jvm.to_rust(instance).unwrap()
    }
    fn get_file_md5(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = self.get_instance().unwrap();
        let instance = jvm
            .invoke(&instance, "getFileMd5", InvocationArg::empty())
            .unwrap();
        let instance = primitive_byte_array_to_string(&jvm, instance);
        jvm.to_rust(instance).unwrap()
    }
    fn get_file_size(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = self.get_instance().unwrap();
        let instance = jvm
            .invoke(&instance, "getFileSize", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
    fn get_file_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = self.get_instance().unwrap();
        // 这里就是 Filename 而非 FileName.
        let instance = jvm
            .invoke(&instance, "getFilename", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
}

pub trait MessageHashCodeTrait: GetInstanceTrait {
    fn hash_code(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance().unwrap())
            .unwrap()
            .invoke("hashCode", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
    }
}
