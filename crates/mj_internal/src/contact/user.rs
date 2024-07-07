use mj_helper_macro::mj_all;

use crate::contact::{
    ContactOrBotTrait, ContactTrait, Friend, Member, Stranger, UserOrBotTrait, UserTrait,
};

#[mj_all("contact.User")]
pub enum User {
    Member(Member),
    Friend(Friend),
    Stranger(Stranger),
}
impl UserTrait for User {}
impl ContactTrait for User {}
impl ContactOrBotTrait for User {}
impl UserOrBotTrait for User {}
