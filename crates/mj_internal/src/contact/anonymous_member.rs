use j4rs::{Instance, InvocationArg, Jvm};
use mj_helper_macro::mj_all;

use crate::contact::{ContactOrBotTrait, ContactTrait, MemberTrait, UserOrBotTrait, UserTrait};
use crate::utils::backend::BotBackend;

#[mj_all("contact.AnonymousMember")]
pub struct AnonymousMember<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> AnonymousMember<B> {
    pub fn get_anonymous_id(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let id = jvm
            .invoke(&self.instance, "getAnonymousId", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(id).unwrap()
    }
}

impl<B: BotBackend> MemberTrait<B> for AnonymousMember<B> {}

impl<B: BotBackend> ContactOrBotTrait<B> for AnonymousMember<B> {
}

impl<B: BotBackend> ContactTrait<B> for AnonymousMember<B> {}

impl<B: BotBackend> UserOrBotTrait<B> for AnonymousMember<B> {}

impl<B: BotBackend> UserTrait<B> for AnonymousMember<B> {}
