use j4rs::{Instance, InvocationArg, Jvm};
use j4rs::errors::J4RsError;
use jbuchong::GetClassTypeTrait;
use jbuchong::{
    {TryFromInstanceTrait, GetInstanceTrait},
    utils::java_iter_to_rust_vec,
};
use mj_helper_macro::mj_all;
use jbuchong::{AsInstanceDerive, GetInstanceDerive};

use crate::contact::Friend;

#[derive(AsInstanceDerive, GetInstanceDerive)]
pub struct FriendGroup {
    pub(crate) instance: Instance,
}

impl TryFromInstanceTrait for FriendGroup {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        Ok(Self { instance })
    }
}

impl FriendGroup {
    pub fn delete(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "delete", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn rename_to(&self, new_name: &str) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let new_name = InvocationArg::try_from(new_name).unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "delete", &[new_name]).unwrap())
            .unwrap()
    }
    pub fn move_in(&self, friend: Friend) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let friend = InvocationArg::try_from(friend.get_instance()).unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "delete", &[friend]).unwrap())
            .unwrap()
    }
    pub fn get_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getName", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn get_id(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getId", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn get_friends(&self) -> Vec<Friend> {
        let jvm = Jvm::attach_thread().unwrap();
        let collection = jvm
            .invoke(&self.instance, "getFriends", InvocationArg::empty())
            .unwrap();
        let iter = jvm
            .invoke(&collection, "iterator", InvocationArg::empty())
            .unwrap();
        java_iter_to_rust_vec(&jvm, iter)
    }
    pub fn get_count(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getCount", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
}

#[mj_all("contact.friendgroup.FriendGroups")]
pub struct FriendGroups {
    instance: Instance,
}

impl FriendGroups {
    pub fn to_vec(&self) -> Vec<FriendGroup> {
        let jvm = Jvm::attach_thread().unwrap();
        let collection = jvm
            .invoke(&self.instance, "asCollection", InvocationArg::empty())
            .unwrap();

        let iter = jvm
            .invoke(&collection, "iterator", InvocationArg::empty())
            .unwrap();
        java_iter_to_rust_vec(&jvm, iter)
    }
    pub fn create(name: String) -> FriendGroup {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static(
                <Self as GetClassTypeTrait>::get_type_name(),
                "create",
                &[InvocationArg::try_from(name).unwrap()],
            )
            .unwrap();
        FriendGroup { instance }
    }
    pub fn get(&self, id: i64) -> FriendGroup {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(
                &self.instance,
                "get",
                &[InvocationArg::try_from(id).unwrap()],
            )
            .unwrap();
        FriendGroup { instance }
    }
    pub fn get_default(&self) -> FriendGroup {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getDefault", InvocationArg::empty())
            .unwrap();
        FriendGroup { instance }
    }
}
