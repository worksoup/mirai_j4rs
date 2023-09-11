#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(return_position_impl_trait_in_trait)]
#![feature(core_intrinsics)]
#![feature(impl_trait_in_assoc_type)]
#![feature(specialization)]

pub mod action;
pub mod contact;
pub mod error;
pub mod event;
pub mod file;
pub mod message;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::env::FromInstance;
    use crate::utils::ffi::Consumer;
    use j4rs::{ClasspathEntry, Instance, InvocationArg, Jvm, JvmBuilder};

    fn get_a_jvm_for_test() -> Jvm {
        JvmBuilder::new()
            .classpath_entry(ClasspathEntry::new(
                "/run/media/leart/5A98CD5F98CD3A71/Users/15102/Works/Mirai/MiraiRS/jvm_side.jar",
            ))
            .build()
            .unwrap()
    }

    #[test]
    fn closure_to_consumer_works() {
        struct X {
            instance: Instance,
        }
        impl X {
            fn fuck(&self) -> String {
                let jvm = Jvm::attach_thread().unwrap();
                jvm.chain(&self.instance)
                    .unwrap()
                    .invoke("getClass", &[])
                    .unwrap()
                    .invoke("toString", &[])
                    .unwrap()
                    .to_rust()
                    .unwrap()
            }
        }
        impl FromInstance for X {
            fn from_instance(instance: Instance) -> Self {
                X { instance }
            }
        }
        let jvm = get_a_jvm_for_test();
        let a = 2;
        let consumer = Consumer::new(|x: X| {
            println!("a = {a}\nThe class name is `{}`.", x.fuck());
        });
        let test_instance = InvocationArg::try_from(true).unwrap();
        consumer.accept(test_instance);
    }
}

mod env {
    use crate::contact::contact_trait::ContactOrBotTrait;
    use j4rs::Instance;

    pub trait GetClassTypeTrait {
        fn get_class_type() -> Instance;
    }

    pub trait GetEnvTrait {
        fn get_instance(&self) -> Instance;
    }

    pub trait GetBotTrait {
        fn get_bot(&self) -> crate::contact::bot::Bot;
    }

    /// 通过 `j4rs::Instance` 获得当前结构体。
    pub trait ContactFromInstance
        where
            Self: GetEnvTrait + ContactOrBotTrait,
    {
        type Item: GetEnvTrait + ContactOrBotTrait;
        fn from_instance(bot: Instance, instance: Instance, id: i64) -> Self::Item;
    }

    /// 通过 `j4rs::Instance` 获得当前结构体。
    pub trait FromInstance {
        fn from_instance(instance: Instance) -> Self;
    }
}

pub mod other {
    pub mod enums {
        use num_enum::IntoPrimitive;

        pub enum HeartbeatStrategy {
            /// `HeartbeatStrategy.STAT_HB`
            S,
            /// `HeartbeatStrategy.REGISTER`
            R,
            /// `HeartbeatStrategy.NONE`
            N,
        }

        pub enum MiraiProtocol {
            /// `MiraiProtocol.ANDROID_PHONE`
            A,
            /// `MiraiProtocol.ANDROID_PAD`
            P,
            /// `MiraiProtocol.ANDROID_WATCH`
            W,
            /// `MiraiProtocol.IPAD`
            I,
            /// `MiraiProtocol.MACOS`
            M,
        }

        #[derive(IntoPrimitive)]
        #[repr(i32)]
        pub enum AvatarSpec {
            /// SMALLEST(40), 最高压缩等级。
            XS = 40,
            /// SMALL(41), 群员列表中的显示大小, 实际上是 40 px, 但会比 `SMALLEST` 好一些。
            S = 41,
            /// MEDIUM(100), 联系人列表中的显示大小。
            M = 100,
            /// LARGE(140), 消息列表中的显示大小。
            L = 140,
            /// LARGEST(640), 联系人详情页面中的显示大小。
            XL = 640,
            /// ORIGINAL(0), 原图。
            ORIGINAL = 0,
        }
    }

    pub mod tools {
        use j4rs::{InvocationArg, Jvm};

        use super::enums::MiraiProtocol;

        pub fn protocol_enum_r2j(
            protocol: MiraiProtocol,
        ) -> Result<InvocationArg, std::convert::Infallible> {
            InvocationArg::try_from(
                Jvm::attach_thread()
                    .unwrap()
                    .field(
                        &Jvm::attach_thread()
                            .unwrap()
                            .static_class("net.mamoe.mirai.utils.BotConfiguration$MiraiProtocol")
                            .unwrap(),
                        match protocol {
                            MiraiProtocol::A => "ANDROID_PHONE",
                            MiraiProtocol::I => "IPAD",
                            MiraiProtocol::M => "MACOS",
                            MiraiProtocol::P => "ANDROID_PAD",
                            MiraiProtocol::W => "ANDROID_WATCH",
                        },
                    )
                    .unwrap(),
            )
        }

        pub fn protocol_str2enum(protocol: String) -> MiraiProtocol {
            match protocol.as_str() {
                "ANDROID_PHONE" => MiraiProtocol::A,
                "IPAD" => MiraiProtocol::I,
                "MACOS" => MiraiProtocol::M,
                "ANDROID_PAD" => MiraiProtocol::P,
                "ANDROID_WATCH" => MiraiProtocol::W,
                _ => {
                    println!("&self.instance is None!");
                    MiraiProtocol::A
                }
            }
        }
    }
}
