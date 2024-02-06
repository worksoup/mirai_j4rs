use crate::contact::{
    ContactOrBotTrait, ContactTrait, Friend, Member, Stranger, UserOrBotTrait, UserTrait,
};
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.contact.User")]
pub enum User {
    Member(Member),
    Friend(Friend),
    #[fall]
    Stranger(Stranger),
}
impl UserTrait for User {}
impl ContactTrait for User {}
impl ContactOrBotTrait for User {}
impl UserOrBotTrait for User {}
