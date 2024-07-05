use std::hint::unreachable_unchecked;

use j4rs::{Instance, InvocationArg, Jvm};

use mj_base::env::{FromInstanceTrait, GetClassTypeTrait};
use mj_macro::{java_type, AsInstanceDerive, GetInstanceDerive};

use crate::message::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MessageContentTrait, MessageHashCodeTrait,
    MessageTrait, SingleMessageTrait,
};

#[derive(Clone, Copy)]
pub enum PokeMessageEnum {
    戳一戳,
    比心,
    点赞,
    心碎,
    六六六,
    放大招,
    宝贝球,
    玫瑰花,
    召唤术,
    让你皮,
    结印,
    手雷,
    勾引,
    抓一下,
    碎屏,
    敲门,
}

impl PokeMessageEnum {
    pub fn get_name(&self) -> &str {
        match self {
            PokeMessageEnum::戳一戳 => "戳一戳",
            PokeMessageEnum::比心 => "比心",
            PokeMessageEnum::点赞 => "点赞",
            PokeMessageEnum::心碎 => "心碎",
            PokeMessageEnum::六六六 => "666",
            PokeMessageEnum::放大招 => "放大招",
            PokeMessageEnum::宝贝球 => "宝贝球",
            PokeMessageEnum::让你皮 => "让你皮",
            PokeMessageEnum::玫瑰花 => "玫瑰花",
            PokeMessageEnum::召唤术 => "召唤术",
            PokeMessageEnum::结印 => "结印",
            PokeMessageEnum::手雷 => "手雷",
            PokeMessageEnum::勾引 => "勾引",
            PokeMessageEnum::碎屏 => "碎屏",
            PokeMessageEnum::抓一下 => "抓一下",
            PokeMessageEnum::敲门 => "敲门",
        }
    }
    pub fn get_poke_type(&self) -> i32 {
        match self {
            PokeMessageEnum::戳一戳 => 1,
            PokeMessageEnum::比心 => 2,
            PokeMessageEnum::点赞 => 3,
            PokeMessageEnum::心碎 => 4,
            PokeMessageEnum::六六六 => 5,
            PokeMessageEnum::放大招 => 6,
            PokeMessageEnum::宝贝球
            | PokeMessageEnum::让你皮
            | PokeMessageEnum::玫瑰花
            | PokeMessageEnum::召唤术
            | PokeMessageEnum::结印
            | PokeMessageEnum::手雷
            | PokeMessageEnum::勾引
            | PokeMessageEnum::碎屏
            | PokeMessageEnum::抓一下
            | PokeMessageEnum::敲门 => 126,
        }
    }
    pub fn get_id(&self) -> i32 {
        match self {
            PokeMessageEnum::戳一戳
            | PokeMessageEnum::比心
            | PokeMessageEnum::点赞
            | PokeMessageEnum::心碎
            | PokeMessageEnum::六六六
            | PokeMessageEnum::放大招 => -1,
            PokeMessageEnum::宝贝球 => 2011,
            PokeMessageEnum::让你皮 => 2009,
            PokeMessageEnum::玫瑰花 => 2007,
            PokeMessageEnum::召唤术 => 2006,
            PokeMessageEnum::结印 => 2005,
            PokeMessageEnum::手雷 => 2004,
            PokeMessageEnum::勾引 => 2003,
            PokeMessageEnum::碎屏 => 2002,
            PokeMessageEnum::抓一下 => 2001,
            PokeMessageEnum::敲门 => 2000,
        }
    }
}
#[derive(AsInstanceDerive, GetInstanceDerive)]
#[java_type("message.data.PokeMessage")]
pub struct PokeMessage {
    r#enum: PokeMessageEnum,
    instance: Instance,
}
impl FromInstanceTrait for PokeMessage {
    fn from_instance(instance: Instance) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let t: (i32, i32) = (
            jvm.to_rust(
                jvm.invoke(&instance, "getPokeType", InvocationArg::empty())
                    .unwrap(),
            )
            .unwrap(),
            jvm.to_rust(
                jvm.invoke(&instance, "getId", InvocationArg::empty())
                    .unwrap(),
            )
            .unwrap(),
        );
        let r#enum = match t {
            (a, -1) => match a {
                1 => PokeMessageEnum::戳一戳,
                2 => PokeMessageEnum::比心,
                3 => PokeMessageEnum::点赞,
                4 => PokeMessageEnum::心碎,
                5 => PokeMessageEnum::六六六,
                6 => PokeMessageEnum::放大招,
                _ => unsafe { unreachable_unchecked() },
            },
            (126, b) => match b {
                2011 => PokeMessageEnum::宝贝球,
                2009 => PokeMessageEnum::让你皮,
                2007 => PokeMessageEnum::玫瑰花,
                2006 => PokeMessageEnum::召唤术,
                2005 => PokeMessageEnum::结印,
                2004 => PokeMessageEnum::手雷,
                2003 => PokeMessageEnum::勾引,
                2002 => PokeMessageEnum::碎屏,
                2001 => PokeMessageEnum::抓一下,
                2000 => PokeMessageEnum::敲门,
                _ => unsafe { unreachable_unchecked() },
            },
            _ => unsafe { unreachable_unchecked() },
        };
        PokeMessage { r#enum, instance }
    }
}
impl From<PokeMessage> for PokeMessageEnum {
    fn from(value: PokeMessage) -> Self {
        value.r#enum
    }
}
impl From<PokeMessageEnum> for PokeMessage {
    fn from(value: PokeMessageEnum) -> PokeMessage {
        let jvm = Jvm::attach_thread().unwrap();
        let type_name = <PokeMessage as GetClassTypeTrait>::get_type_name();
        let field = match value {
            PokeMessageEnum::戳一戳 => "ChuoYiChuo",
            PokeMessageEnum::比心 => "BiXin",
            PokeMessageEnum::点赞 => "DianZan",
            PokeMessageEnum::心碎 => "XinSui",
            PokeMessageEnum::六六六 => "LiuLiuLiu",
            PokeMessageEnum::放大招 => "FangDaZhao",
            PokeMessageEnum::宝贝球 => "BaoBeiQiu",
            PokeMessageEnum::玫瑰花 => "Rose",
            PokeMessageEnum::召唤术 => "ZhaoHuanShu",
            PokeMessageEnum::让你皮 => "RangNiPi",
            PokeMessageEnum::结印 => "JieYin",
            PokeMessageEnum::手雷 => "ShouLei",
            PokeMessageEnum::勾引 => "GouYin",
            PokeMessageEnum::抓一下 => "ZhuaYiXia",
            PokeMessageEnum::碎屏 => "SuiPing",
            PokeMessageEnum::敲门 => "QiaoMen",
        };
        PokeMessage::from_instance(jvm.static_class_field(type_name.as_str(), field).unwrap())
    }
}
impl PokeMessage {
    pub fn get_name(&self) -> &str {
        self.r#enum.get_name()
    }
    pub fn get_poke_type(&self) -> i32 {
        self.r#enum.get_poke_type()
    }
    pub fn get_id(&self) -> i32 {
        self.r#enum.get_id()
    }
}

impl MessageTrait for PokeMessage {
    fn to_content(&self) -> String {
        String::from("[戳一戳]")
    }

    fn to_string(&self) -> String {
        let mut str = String::from("[mirai:poke");
        str.push_str(self.get_name());
        str.push(',');
        str.push_str(self.get_poke_type().to_string().as_str());
        str.push(',');
        str.push_str(self.get_id().to_string().as_str());
        str.push(']');
        str
    }
}

impl SingleMessageTrait for PokeMessage {}

impl MessageContentTrait for PokeMessage {}

impl ConstrainSingleTrait for PokeMessage {}

impl CodableMessageTrait for PokeMessage {}

impl MessageHashCodeTrait for PokeMessage {}
