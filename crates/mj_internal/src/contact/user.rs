use mj_helper_macro::mj_all;

use crate::contact::{
    ContactOrBotTrait, ContactTrait, Friend, Member, Stranger, UserOrBotTrait, UserTrait,
};
use crate::utils::backend::BotBackend;

#[mj_all("contact.User")]
pub enum User<B: BotBackend> {
    Member(Member<B>),
    Friend(Friend<B>),
    Stranger(Stranger<B>),
}
impl<B: BotBackend> UserTrait<B> for User<B> {}
impl<B: BotBackend> ContactTrait<B> for User<B> {}
impl<B: BotBackend> ContactOrBotTrait<B> for User<B> {}
impl<B: BotBackend> UserOrBotTrait<B> for User<B> {}
