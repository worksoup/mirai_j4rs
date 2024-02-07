use mj_macro::mj_all;

use crate::contact::{
    AnonymousMember, ContactOrBotTrait, ContactTrait, MemberTrait, NormalMember, UserOrBotTrait,
    UserTrait,
};

/// **注意**
///
/// [匿名成员](AnonymousMember)不支持发送消息（包括上传图片等）。
/// [Member] 本质上是一个枚举，如果需要发送消息请使用 `match` 等语句获取枚举中的 [`NormalMember`], 然后再发送消息。
///
#[mj_all("contact.Member")]
pub enum Member {
    NormalMember(NormalMember),
    AnonymousMember(AnonymousMember),
}

impl MemberTrait for Member {}

impl ContactOrBotTrait for Member {}

impl ContactTrait for Member {}

impl UserOrBotTrait for Member {}

impl UserTrait for Member {}
