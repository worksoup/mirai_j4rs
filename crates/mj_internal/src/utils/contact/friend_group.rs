use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{
    java_all, utils::java_iter_to_rust_vec, FromInstanceTrait, GetClassTypeTrait, GetInstanceTrait,
};
use mj_helper_macro::mj_all;

use crate::contact::Friend;
use crate::utils::backend::BotBackend;

#[java_all]
pub struct FriendGroup<B: BotBackend> {
    pub(crate) instance: Instance,
    _backend: B,
}

impl<B: BotBackend> FriendGroup<B> {
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
    pub fn move_in(&self, friend: Friend<B>) -> bool {
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
    pub fn get_friends(&self) -> Vec<Friend<B>> {
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
pub struct FriendGroups<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> FriendGroups<B> {
    pub fn to_vec(&self) -> Vec<FriendGroup<B>> {
        let jvm = Jvm::attach_thread().unwrap();
        let collection = jvm
            .invoke(&self.instance, "asCollection", InvocationArg::empty())
            .unwrap();

        let iter = jvm
            .invoke(&collection, "iterator", InvocationArg::empty())
            .unwrap();
        java_iter_to_rust_vec(&jvm, iter)
    }
    pub fn create(name: String) -> FriendGroup<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static(
                <Self as GetClassTypeTrait>::get_type_name(),
                "create",
                &[InvocationArg::try_from(name).unwrap()],
            )
            .unwrap();
        FriendGroup::from_instance(instance)
    }
    pub fn get(&self, id: i64) -> FriendGroup<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(
                &self.instance,
                "get",
                &[InvocationArg::try_from(id).unwrap()],
            )
            .unwrap();
        FriendGroup::from_instance(instance)
    }
    pub fn get_default(&self) -> FriendGroup<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getDefault", InvocationArg::empty())
            .unwrap();
        FriendGroup::from_instance(instance)
    }
}
