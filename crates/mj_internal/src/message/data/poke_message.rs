use std::hint::unreachable_unchecked;

use crate::message::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MessageContentTrait, MessageHashCodeTrait,
    MessageTrait, SingleMessageTrait,
};
use crate::utils::backend::BotBackend;
use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{GetClassTypeTrait, TryFromInstanceTrait};
use mj_helper_macro::mj_all;

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
fn get_enum_(instance: &Instance) -> PokeMessageEnum {
    let jvm = Jvm::attach_thread().unwrap();
    let t: (i32, i32) = (
        jvm.to_rust(
            jvm.invoke(instance, "getPokeType", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap(),
        jvm.to_rust(
            jvm.invoke(instance, "getId", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap(),
    );
    match t {
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
    }
}
#[mj_all("message.data.PokeMessage")]
pub struct PokeMessage<B: BotBackend> {
    #[default(fn_name = get_enum_)]
    r#enum: PokeMessageEnum,
    instance: Instance,
    _backend: B,
}
impl<B: BotBackend> From<PokeMessage<B>> for PokeMessageEnum {
    fn from(value: PokeMessage<B>) -> Self {
        value.r#enum
    }
}
impl<B: BotBackend> From<PokeMessageEnum> for PokeMessage<B> {
    fn from(value: PokeMessageEnum) -> PokeMessage<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let type_name = <PokeMessage<B> as GetClassTypeTrait>::get_type_name();
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
        PokeMessage::try_from_instance(jvm.static_class_field(type_name, field).unwrap()).unwrap()
    }
}
impl<B: BotBackend> PokeMessage<B> {
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

impl<B: BotBackend> MessageTrait<B> for PokeMessage<B> {
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

impl<B: BotBackend> SingleMessageTrait<B> for PokeMessage<B> {}

impl<B: BotBackend> MessageContentTrait<B> for PokeMessage<B> {}

impl<B: BotBackend> ConstrainSingleTrait<B> for PokeMessage<B> {}

impl<B: BotBackend> CodableMessageTrait<B> for PokeMessage<B> {}

impl<B: BotBackend> MessageHashCodeTrait for PokeMessage<B> {}
