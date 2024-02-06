use crate::contact::{ContactOrBotTrait, ContactTrait, MemberTrait, UserOrBotTrait, UserTrait};
use j4rs::{Instance, Jvm};
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.contact.AnonymousMember")]
pub struct AnonymousMember {
    instance: Instance,
}

impl AnonymousMember {
    pub fn get_anonymous_id(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let id = jvm.invoke(&self.instance, "getAnonymousId", &[]).unwrap();
        jvm.to_rust(id).unwrap()
    }
}

impl MemberTrait for AnonymousMember {}

impl ContactOrBotTrait for AnonymousMember {}

impl ContactTrait for AnonymousMember {}

impl UserOrBotTrait for AnonymousMember {}

impl UserTrait for AnonymousMember {}
