use j4rs::{Instance, InvocationArg};
use mj_helper_macro::{java_fn, mj_all};

use crate::contact::{ContactOrBotTrait, ContactTrait, MemberTrait, UserOrBotTrait, UserTrait};
use crate::utils::backend::BotBackend;

#[mj_all("contact.AnonymousMember")]
pub struct AnonymousMember<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> AnonymousMember<B> {
    #[java_fn]
    pub fn get_anonymous_id(&self) -> String {}
}

impl<B: BotBackend> MemberTrait<B> for AnonymousMember<B> {}

impl<B: BotBackend> ContactOrBotTrait<B> for AnonymousMember<B> {}

impl<B: BotBackend> ContactTrait<B> for AnonymousMember<B> {}

impl<B: BotBackend> UserOrBotTrait<B> for AnonymousMember<B> {}

impl<B: BotBackend> UserTrait<B> for AnonymousMember<B> {}
