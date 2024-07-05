use crate::contact::ActiveRankRecord;
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::{FromInstanceTrait, GetInstanceTrait};

pub struct MiraiList<T> {
    instance: Instance,
    vec: Option<Vec<T>>,
}
impl<T> FromInstanceTrait for MiraiList<T> {
    fn from_instance(instance: Instance) -> Self {
        Self {
            instance,
            vec: None,
        }
    }
}
impl MiraiList<ActiveRankRecord> {
    ///这个函数记得改改。
    pub fn to_vector(&self) -> &Option<Vec<ActiveRankRecord>> {
        &self.vec
    }
    ///这个函数记得改改。
    pub fn refresh_vector(&mut self) {
        let jvm = Jvm::attach_thread().unwrap();
        let it = jvm
            .invoke(&self.instance, "listIterator", InvocationArg::empty())
            .unwrap();
        let mut vec = Vec::<ActiveRankRecord>::new();
        while jvm
            .chain(&it)
            .unwrap()
            .invoke("hasNext", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
        {
            let v = jvm.invoke(&it, "next", InvocationArg::empty()).unwrap();
            vec.push(ActiveRankRecord::from_instance(v))
        }
        self.vec = Some(vec);
    }
}

impl<T> GetInstanceTrait for MiraiList<T> {
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}
