use crate::message::message_trait::{
    CodableMessageTrait, MessageContentTrait, MessageHashCodeTrait, MessageTrait,
    SingleMessageTrait,
};
use crate::utils::backend::BotBackend;
use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::GetClassTypeTrait;
use mj_helper_macro::mj_all;
fn get_content_(instance: &Instance) -> String {
    let jvm = Jvm::attach_thread().unwrap();
    jvm.to_rust(
        jvm.invoke(instance, "getContent", InvocationArg::empty())
            .unwrap(),
    )
    .unwrap()
}
#[mj_all("message.data.PlainText")]
pub struct PlainText<B: BotBackend> {
    #[default(fn_name = get_content_)]
    content: String,
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> From<&str> for PlainText<B> {
    fn from(value: &str) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        PlainText {
            content: value.to_string(),
            instance: jvm
                .create_instance(
                    <Self as GetClassTypeTrait>::get_type_name(),
                    &[InvocationArg::try_from(value).unwrap()],
                )
                .unwrap(),
            _backend: B::default(),
        }
    }
}

impl<B: BotBackend> From<String> for PlainText<B> {
    fn from(value: String) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                <Self as GetClassTypeTrait>::get_type_name(),
                &[InvocationArg::try_from(&value).unwrap()],
            )
            .unwrap();
        PlainText {
            content: value,
            instance,
            _backend: B::default(),
        }
    }
}

impl<B: BotBackend> PlainText<B> {
    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}

impl<B: BotBackend> MessageTrait<B> for PlainText<B> {
    fn to_content(&self) -> String {
        self.get_content()
    }
    fn to_string(&self) -> String {
        self.get_content()
    }
}

impl<B: BotBackend> CodableMessageTrait<B> for PlainText<B> {
    fn to_code(&self) -> String {
        self.get_content()
    }
}

impl<B: BotBackend> SingleMessageTrait<B> for PlainText<B> {}

impl<B: BotBackend> MessageContentTrait<B> for PlainText<B> {}

impl<B: BotBackend> MessageHashCodeTrait for PlainText<B> {}
