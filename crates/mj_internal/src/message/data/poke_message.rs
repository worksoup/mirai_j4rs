use crate::message::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MessageContentTrait, MessageHashCodeTrait,
    MessageTrait, SingleMessageTrait,
};
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::{AsInstanceTrait, FromInstance, GetClassTypeTrait, GetInstanceTrait};
use mj_macro::{java_type, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};
use std::hint::unreachable_unchecked;

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
#[java_type("net.mamoe.mirai.message.data.PokeMessage")]
pub struct PokeMessage {
    r#enum: PokeMessageEnum,
    instance: Instance,
}
impl FromInstance for PokeMessage {
    fn from_instance(instance: Instance) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let t: (i32, i32) = (
            jvm.to_rust(jvm.invoke(&instance, "getPokeType", &[]).unwrap())
                .unwrap(),
            jvm.to_rust(jvm.invoke(&instance, "getId", &[]).unwrap())
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
        let instance = match value {
            PokeMessageEnum::戳一戳 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "ChuoYiChuo")
                .unwrap(),
            PokeMessageEnum::比心 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "BiXin")
                .unwrap(),
            PokeMessageEnum::点赞 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "DianZan")
                .unwrap(),
            PokeMessageEnum::心碎 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "XinSui")
                .unwrap(),
            PokeMessageEnum::六六六 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "LiuLiuLiu")
                .unwrap(),
            PokeMessageEnum::放大招 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "FangDaZhao")
                .unwrap(),
            PokeMessageEnum::宝贝球 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "BaoBeiQiu")
                .unwrap(),
            PokeMessageEnum::玫瑰花 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "Rose")
                .unwrap(),
            PokeMessageEnum::召唤术 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "ZhaoHuanShu")
                .unwrap(),
            PokeMessageEnum::让你皮 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "RangNiPi")
                .unwrap(),
            PokeMessageEnum::结印 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "JieYin")
                .unwrap(),
            PokeMessageEnum::手雷 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "ShouLei")
                .unwrap(),
            PokeMessageEnum::勾引 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "GouYin")
                .unwrap(),
            PokeMessageEnum::抓一下 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "ZhuaYiXia")
                .unwrap(),
            PokeMessageEnum::碎屏 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "SuiPing")
                .unwrap(),
            PokeMessageEnum::敲门 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "QiaoMen")
                .unwrap(),
        };
        PokeMessage::from_instance(instance)
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
