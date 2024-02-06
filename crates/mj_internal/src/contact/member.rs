use crate::contact::{
    AnonymousMember, ContactOrBotTrait, ContactTrait, MemberTrait, NormalMember, UserOrBotTrait,
    UserTrait,
};
use j4rs::{Instance, Jvm};
use mj_base::env::{AsInstanceTrait, FromInstance, GetInstanceTrait};
use mj_macro::{java_type, mj_all, AsInstanceDerive, GetInstanceDerive};

/// **注意**
///
/// [匿名成员][AnonymousMember]不支持发送消息（包括上传图片等）。
/// [Member] 本质上是一个枚举，如果需要发送消息请使用 `match` 等语句获取枚举中的 [NormalMember], 然后再发送消息。
///
/// 发送 [NormalMemberNudge] 同理。
#[derive(GetInstanceDerive, AsInstanceDerive)]
#[java_type("net.mamoe.mirai.contact.Member")]
pub enum Member {
    NormalMember(NormalMember),
    AnonymousMember(AnonymousMember),
}

impl FromInstance for Member {
    fn from_instance(instance: Instance) -> Self {
        // TODO: 这样的转换存在非常情况，并不科学。
        let jvm = Jvm::attach_thread().unwrap();
        let special_title: String = jvm
            .to_rust(jvm.invoke(&instance, "getSpecialTitle", &[]).unwrap())
            .unwrap();
        if special_title.as_str() != "匿名" {
            Member::NormalMember(NormalMember::from_instance(instance))
        } else {
            Member::AnonymousMember(AnonymousMember::from_instance(instance))
        }
    }
}

impl MemberTrait for Member {}

impl ContactOrBotTrait for Member {}

impl ContactTrait for Member {}

impl UserOrBotTrait for Member {}

impl UserTrait for Member {}
