use crate::message::message_trait::{
    CodableMessageTrait, ConstrainSingleTrait, MessageContentTrait, MessageHashCodeTrait,
    MessageTrait, SingleMessageTrait,
};
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::{FromInstance, GetClassTypeTrait, GetInstanceTrait};
use mj_macro::java_type;
use std::hint::unreachable_unchecked;

#[java_type("net.mamoe.mirai.message.data.PokeMessage")]
pub enum PokeMessage {
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

impl GetInstanceTrait for PokeMessage {
    fn get_instance(&self) -> Instance {
        let jvm = Jvm::attach_thread().unwrap();
        match self {
            PokeMessage::戳一戳 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "ChuoYiChuo")
                .unwrap(),
            PokeMessage::比心 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "BiXin")
                .unwrap(),
            PokeMessage::点赞 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "DianZan")
                .unwrap(),
            PokeMessage::心碎 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "XinSui")
                .unwrap(),
            PokeMessage::六六六 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "LiuLiuLiu")
                .unwrap(),
            PokeMessage::放大招 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "FangDaZhao")
                .unwrap(),
            PokeMessage::宝贝球 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "BaoBeiQiu")
                .unwrap(),
            PokeMessage::玫瑰花 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "Rose")
                .unwrap(),
            PokeMessage::召唤术 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "ZhaoHuanShu")
                .unwrap(),
            PokeMessage::让你皮 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "RangNiPi")
                .unwrap(),
            PokeMessage::结印 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "JieYin")
                .unwrap(),
            PokeMessage::手雷 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "ShouLei")
                .unwrap(),
            PokeMessage::勾引 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "GouYin")
                .unwrap(),
            PokeMessage::抓一下 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "ZhuaYiXia")
                .unwrap(),
            PokeMessage::碎屏 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "SuiPing")
                .unwrap(),
            PokeMessage::敲门 => jvm
                .static_class_field("net.mamoe.mirai.message.data.PokeMessage", "QiaoMen")
                .unwrap(),
        }
        // let (name, poke_type, id) = self.get_name_and_poke_type_and_id();
        // let (name, poke_type, id) = (
        //     InvocationArg::try_from(name).unwrap(),
        //     InvocationArg::try_from(poke_type).unwrap(),
        //     InvocationArg::try_from(id).unwrap(),
        // );
        // jvm.create_instance(
        //     "net.mamoe.mirai.message.data.PokeMessage",
        //     &[name, poke_type, id],
        // )
        // .unwrap()
    }
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
        match t {
            (a, -1) => match a {
                1 => PokeMessage::戳一戳,
                2 => PokeMessage::比心,
                3 => PokeMessage::点赞,
                4 => PokeMessage::心碎,
                5 => PokeMessage::六六六,
                6 => PokeMessage::放大招,
                _ => unsafe { unreachable_unchecked() },
            },
            (126, b) => match b {
                2011 => PokeMessage::宝贝球,
                2009 => PokeMessage::让你皮,
                2007 => PokeMessage::玫瑰花,
                2006 => PokeMessage::召唤术,
                2005 => PokeMessage::结印,
                2004 => PokeMessage::手雷,
                2003 => PokeMessage::勾引,
                2002 => PokeMessage::碎屏,
                2001 => PokeMessage::抓一下,
                2000 => PokeMessage::敲门,
                _ => unsafe { unreachable_unchecked() },
            },
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl PokeMessage {
    fn get_name_and_poke_type_and_id(&self) -> (&str, i32, i32) {
        // match self {
        //     PokeMessage::戳一戳 => ("戳一戳", 1, -1),
        //     PokeMessage::比心 => ("比心", 2, -1),
        //     PokeMessage::点赞 => ("点赞", 3, -1),
        //     PokeMessage::心碎 => ("心碎", 4, -1),
        //     PokeMessage::六六六 => ("666", 5, -1),
        //     PokeMessage::放大招 => ("放大招", 6, -1),
        //     PokeMessage::宝贝球 => ("宝贝球", 126, 2011),
        //     PokeMessage::让你皮 => ("让你皮", 126, 2009),
        //     PokeMessage::玫瑰花 => ("玫瑰花", 126, 2007),
        //     PokeMessage::召唤术 => ("召唤术", 126, 2006),
        //     PokeMessage::结印 => ("结印", 126, 2005),
        //     PokeMessage::手雷 => ("手雷", 126, 2004),
        //     PokeMessage::勾引 => ("勾引", 126, 2003),
        //     PokeMessage::碎屏 => ("碎屏", 126, 2002),
        //     PokeMessage::抓一下 => ("抓一下", 126, 2001),
        //     PokeMessage::敲门 => ("敲门", 126, 2000),
        // }
        (self.get_name(), self.get_poke_type(), self.get_id())
    }
    pub fn get_name(&self) -> &str {
        match self {
            PokeMessage::戳一戳 => "戳一戳",
            PokeMessage::比心 => "比心",
            PokeMessage::点赞 => "点赞",
            PokeMessage::心碎 => "心碎",
            PokeMessage::六六六 => "666",
            PokeMessage::放大招 => "放大招",
            PokeMessage::宝贝球 => "宝贝球",
            PokeMessage::让你皮 => "让你皮",
            PokeMessage::玫瑰花 => "玫瑰花",
            PokeMessage::召唤术 => "召唤术",
            PokeMessage::结印 => "结印",
            PokeMessage::手雷 => "手雷",
            PokeMessage::勾引 => "勾引",
            PokeMessage::碎屏 => "碎屏",
            PokeMessage::抓一下 => "抓一下",
            PokeMessage::敲门 => "敲门",
        }
    }
    pub fn get_poke_type(&self) -> i32 {
        match self {
            PokeMessage::戳一戳 => 1,
            PokeMessage::比心 => 2,
            PokeMessage::点赞 => 3,
            PokeMessage::心碎 => 4,
            PokeMessage::六六六 => 5,
            PokeMessage::放大招 => 6,
            PokeMessage::宝贝球
            | PokeMessage::让你皮
            | PokeMessage::玫瑰花
            | PokeMessage::召唤术
            | PokeMessage::结印
            | PokeMessage::手雷
            | PokeMessage::勾引
            | PokeMessage::碎屏
            | PokeMessage::抓一下
            | PokeMessage::敲门 => 126,
        }
    }
    pub fn get_id(&self) -> i32 {
        match self {
            PokeMessage::戳一戳
            | PokeMessage::比心
            | PokeMessage::点赞
            | PokeMessage::心碎
            | PokeMessage::六六六
            | PokeMessage::放大招 => -1,
            PokeMessage::宝贝球 => 2011,
            PokeMessage::让你皮 => 2009,
            PokeMessage::玫瑰花 => 2007,
            PokeMessage::召唤术 => 2006,
            PokeMessage::结印 => 2005,
            PokeMessage::手雷 => 2004,
            PokeMessage::勾引 => 2003,
            PokeMessage::碎屏 => 2002,
            PokeMessage::抓一下 => 2001,
            PokeMessage::敲门 => 2000,
        }
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
