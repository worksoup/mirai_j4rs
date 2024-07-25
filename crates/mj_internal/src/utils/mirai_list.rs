use crate::contact::ActiveRankRecord;
use j4rs::errors::J4RsError;
use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{GetInstanceTrait, TryFromInstanceTrait};

pub struct MiraiList<T> {
    instance: Instance,
    vec: Option<Vec<T>>,
}
impl<T> TryFromInstanceTrait for MiraiList<T> {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        Ok(Self {
            instance,
            vec: None,
        })
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
            if let Ok(v) = ActiveRankRecord::try_from_instance(v) {
                vec.push(v)
            }
        }
        self.vec = Some(vec);
    }
}

impl<T> GetInstanceTrait for MiraiList<T> {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        Ok(Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap())
    }
}
