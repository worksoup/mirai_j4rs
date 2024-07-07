use j4rs::errors::J4RsError;
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::TryFromInstanceTrait;
use mj_base::env::GetClassTypeTrait;
use mj_macro::{java_type, AsInstanceDerive, GetInstanceDerive};

use crate::message::message_trait::{
    CodableMessageTrait, MessageContentTrait, MessageHashCodeTrait, MessageTrait,
    SingleMessageTrait,
};

#[derive(AsInstanceDerive, GetInstanceDerive)]
#[java_type("message.data.PlainText")]
pub struct PlainText {
    content: String,
    instance: Instance,
}

impl From<&str> for PlainText {
    fn from(value: &str) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        PlainText {
            content: value.to_string(),
            instance: jvm
                .create_instance(
                    <Self as GetClassTypeTrait>::get_type_name().as_str(),
                    &[InvocationArg::try_from(value).unwrap()],
                )
                .unwrap(),
        }
    }
}

impl From<String> for PlainText {
    fn from(value: String) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                <Self as GetClassTypeTrait>::get_type_name().as_str(),
                &[InvocationArg::try_from(&value).unwrap()],
            )
            .unwrap();
        PlainText {
            content: value,
            instance,
        }
    }
}

impl PlainText {
    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}

impl MessageTrait for PlainText {
    fn to_content(&self) -> String {
        self.get_content()
    }
    fn to_string(&self) -> String {
        self.get_content()
    }
}

impl CodableMessageTrait for PlainText {
    fn to_code(&self) -> String {
        self.get_content()
    }
}

impl SingleMessageTrait for PlainText {}

impl MessageContentTrait for PlainText {}

impl MessageHashCodeTrait for PlainText {}

impl TryFromInstanceTrait for PlainText {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        let jvm = Jvm::attach_thread().unwrap();
        Ok(PlainText {
            content: jvm
                .to_rust(
                    jvm.invoke(&instance, "getContent", InvocationArg::empty())
                        .unwrap(),
                )
                .unwrap(),
            instance,
        })
    }
}
