//TODO : message_chain_builder
use super::MessageChain;
use crate::{
    contact::{bot::Env, group::Group},
    env::GetEnvTrait,
};
use j4rs::{InvocationArg, Jvm};
pub trait MessageTrait
where
    Self: GetEnvTrait,
{
    fn to_display_string(&self, group: Group) -> String {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(
                        &self.get_instance(),
                        "getDisplay",
                        &[InvocationArg::try_from(group.get_instance()).unwrap()],
                    )
                    .unwrap(),
            )
            .unwrap()
    }
    fn to_content_text(&self) -> String {
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
    fn plus<T>(&self, message: T) -> MessageChain
    where
        T: MessageTrait,
    {
        // j4rs 旧版本中有 bug, 所以只能如下写法。见 https://github.com/astonbitecode/j4rs/issues/71
        let jvm = Jvm::attach_thread().unwrap();
        let msg1 = InvocationArg::try_from(self.get_instance()).unwrap(); // j4rs <= 0.17.1
        let msg2 = InvocationArg::try_from(message.get_instance()).unwrap();
        let instance = jvm // j4rs <= 0.17.1
            .invoke_static("rt.lea.Utils", "callPlus", &[msg1, msg2]) // j4rs <= 0.17.1
            .unwrap(); // j4rs <= 0.17.1
                       // let instance = jvm.invoke(&self.get_instance(), "plus", &[msg2]).unwrap(); // j4rs above 0.17.1
        MessageChain { instance }
    }
}
pub trait CodableMessageTrait
where
    Self: MessageTrait,
{
    fn to_mirai_code(&self) -> String {
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
    fn append_mirai_code_to(&self) -> String {
        //  Jvm::attach_thread().unwrap()
        //     .to_rust(
        //          Jvm::attach_thread().unwrap()
        //             .invoke(&self.get_instance(), "serializeToMiraiCode", &[])
        //             .unwrap(),
        //     )
        //     .unwrap()
        todo!()
    }
}
pub trait SingleMessageTrait
where
    Self: MessageTrait,
{
}
pub trait MessageChainTrait
where
    Self: MessageTrait + CodableMessageTrait,
{
    fn contains(&self) {
        todo!()
    }
    fn deserialize_from_json_string(_env: &Env, _json: String) -> MessageChain {
        todo!()
    }
    fn deserialize_from_mirai_code(_env: &Env, _json: String) -> MessageChain {
        todo!()
    }
    fn get(&self) //-> impl SingleMessageTrait
    {
        todo!()
    }
    fn serialize_to_json_string(_env: &Env, _chain: &MessageChain) -> String {
        todo!()
    }
    fn serialize_to_string(_env: &Env, _chain: &MessageChain) -> String {
        todo!()
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
    fn share() {
        todo!()
    }
}
pub trait ServiceMessageTrait
where
    Self: RichMessageTrait + CodableMessageTrait,
{
    fn get_service_id() {
        todo!()
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
        todo!()
    }
}
pub trait MarketFace
where
    Self: ConstrainSingleTrait + MessageContentTrait,
{
    fn get_name() -> String {
        todo!()
    }
}
pub trait MessageMetaDataTrait
where
    Self: SingleMessageTrait,
{
}
impl<T> MessageTrait for T
where
    T: MessageMetaDataTrait,
{
    fn to_content_text(&self) -> String {
        String::new()
    }
}
/// # TODO
pub trait CustomMessageTrait
where
    Self: SingleMessageTrait,
{
    //TODO
}
pub trait AudioTrait
where
    Self: SingleMessageTrait + ConstrainSingleTrait,
{
}
