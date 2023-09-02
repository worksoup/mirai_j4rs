#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(return_position_impl_trait_in_trait)]
#![feature(core_intrinsics)]
#![feature(impl_trait_in_assoc_type)]

pub mod contact;
pub mod event;
pub mod message;

#[cfg(test)]
mod tests {
    use crate::{
        contact::bot::Env,
        message::{FaceEnum, ImageType},
    };
    use j4rs::Jvm;
    use std::{mem::transmute, path::PathBuf};

    #[test]
    fn it_works() {
        fn f1() -> String {
            let x: Box<dyn Fn() -> i32> = Box::new(|| 114514);
            let y = Box::into_raw(x);
            format!("{}", unsafe { std::mem::transmute::<_, i128>(y) })
        }
        let f1_c = f1().clone().parse::<i128>().unwrap();
        println!("{}", f1());
        let f1: Box<dyn Fn() -> i32> = unsafe { Box::from_raw(transmute(f1_c)) };
        println!("裸指针大小：{}", std::mem::size_of::<*const &u8>());
        println!("{}", f1());
        drop(f1);
        let f1: Box<dyn Fn() -> i32> = unsafe { Box::from_raw(transmute(f1_c)) };
        println!("{}", f1());
        let my_enum = FaceEnum::右太极;
        let name = format!("{:?}", my_enum);
        println!("{}", name);
        println!("{}", ImageType::PNG.get_format_name());
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn how_is_it_in_java() {
        let _env = Env::new(&vec!["../../MiraiRS.jar".to_string()],
        &vec!["-Djava.library.path=/home/leart/Works/Mirai/MiraiRS/rust_side/mirai_j4rs/target/release".to_string()]);
        let jvm = Jvm::attach_thread().unwrap();
        let integer_min = jvm
            .static_class_field("java.lang.Integer", "MIN_VALUE")
            .unwrap();
        let integer_max = jvm
            .static_class_field("java.lang.Integer", "MAX_VALUE")
            .unwrap();
        let integer_min: i64 = jvm.to_rust(integer_min).unwrap();
        let integer_max: i64 = jvm.to_rust(integer_max).unwrap();
        println!("min:{integer_min},\nmax:{integer_max}");
        println!("min:{},\nmax:{}", i64::MIN, i64::MAX);
        println!("min:{},\nmax:{}", i32::MIN, i32::MAX);
    }
}

mod env {
    use crate::contact::contact_trait::ContactOrBotTrait;
    use j4rs::Instance;

    pub trait GetClassTypeTrait {
        fn get_class_type() -> Instance;
    }
    pub trait GetEnvTrait {
        fn get_instance(&self) -> j4rs::Instance;
    }
    pub trait GetBotTrait {
        fn get_bot<'a>(&'a self) -> crate::contact::bot::Bot;
    }

    /// 通过 `j4rs::Instance` 获得当前结构体。
    pub trait FromInstance
    where
        Self: GetEnvTrait + ContactOrBotTrait,
    {
        type Item: GetEnvTrait + ContactOrBotTrait;
        fn from_instance(bot: Instance, instance: Instance, id: i64) -> Self::Item;
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
        /// Mirai 中定义：
        /// ```
        /// public enum class AvatarSpec(@MiraiInternalApi public val size: Int) : Comparable<AvatarSpec> {
        ///     //最高压缩等级
        ///     SMALLEST(40),
        ///
        ///     //群员列表中的显示大小, 实际上是 40 px, 但会比 [SMALLEST] 好一些
        ///     SMALL(41),
        ///
        ///     //联系人列表中的显示大小
        ///     MEDIUM(100),
        ///
        ///     //消息列表中的显示大小
        ///     LARGE(140),
        ///     
        ///     //联系人详情页面中的显示大小
        ///     LARGEST(640),
        ///
        ///     //原图
        ///     ORIGINAL(0);
        /// }
        /// ```
        #[derive(IntoPrimitive)]
        #[repr(i32)]
        pub enum AvatarSpec {
            XS = 40,      //SMALLEST(40),
            S = 41,       //SMALL(41),
            M = 100,      //MEDIUM(100),
            L = 140,      //LARGE(140),
            XL = 640,     //LARGEST(640),
            ORIGINAL = 0, //ORIGINAL(0);
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
