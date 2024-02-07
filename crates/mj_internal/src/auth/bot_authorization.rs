use j4rs::{Instance, InvocationArg, Jvm};

use mj_base::env::{GetClassTypeTrait, GetInstanceTrait};
use mj_macro::java_type;

#[java_type("auth.BotAuthorization")]
pub enum BotAuthorization {
    Password(String),
    Md5([u8; 16]),
    QrCode,
}

// TODO: 测试是否可以直接直接转换。
impl GetInstanceTrait for BotAuthorization {
    fn get_instance(&self) -> Instance {
        let jvm = Jvm::attach_thread().unwrap();
        match self {
            BotAuthorization::Password(password) => jvm
                .invoke_static(
                    <Self as GetClassTypeTrait>::get_type_name().as_str(),
                    "byPassword",
                    &[InvocationArg::try_from(password).unwrap()],
                )
                .unwrap(),
            BotAuthorization::Md5(md5) => {
                let password_md5 = md5.map(|e| {
                    InvocationArg::try_from(e as i8)
                        .unwrap()
                        .into_primitive()
                        .unwrap()
                });
                let arg = jvm.create_java_array("byte", &password_md5).unwrap();
                let arg = InvocationArg::try_from(arg).unwrap();
                jvm.invoke_static(
                    <Self as GetClassTypeTrait>::get_type_name().as_str(),
                    "byPassword",
                    &[arg],
                )
                .unwrap()
            }
            BotAuthorization::QrCode => jvm
                .invoke_static(
                    <Self as GetClassTypeTrait>::get_type_name().as_str(),
                    "byQRCode",
                    &[],
                )
                .unwrap(),
        }
    }
}
