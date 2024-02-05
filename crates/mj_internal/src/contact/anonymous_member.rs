use crate::contact::{ContactOrBotTrait, ContactTrait, MemberTrait, UserOrBotTrait, UserTrait};
use j4rs::{Instance, Jvm};
use mj_base::env::FromInstance;
use mj_macro::{AsInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, AsInstanceDerive)]
pub struct AnonymousMember {
    pub(crate) instance: Instance,
}

impl AnonymousMember {
    pub fn get_anonymous_id(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let id = jvm.invoke(&self.instance, "getAnonymousId", &[]).unwrap();
        jvm.to_rust(id).unwrap()
    }
}

impl FromInstance for AnonymousMember {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }
}

impl MemberTrait for AnonymousMember {}

impl ContactOrBotTrait for AnonymousMember {}

impl ContactTrait for AnonymousMember {}

impl UserOrBotTrait for AnonymousMember {}

impl UserTrait for AnonymousMember {}
