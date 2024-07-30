use crate::contact::ActiveRankRecord;
use crate::utils::backend::BotBackend;
use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{java_all, TryFromInstanceTrait};

#[java_all]
pub struct MiraiList<B: BotBackend, T> {
    instance: Instance,
    vec: Option<Vec<T>>,
    _backend: B,
}
impl<B: BotBackend> MiraiList<B, ActiveRankRecord<B>> {
    ///这个函数记得改改。
    pub fn to_vector(&self) -> &Option<Vec<ActiveRankRecord<B>>> {
        &self.vec
    }
    ///这个函数记得改改。
    pub fn refresh_vector(&mut self) {
        let jvm = Jvm::attach_thread().unwrap();
        let it = jvm
            .invoke(&self.instance, "listIterator", InvocationArg::empty())
            .unwrap();
        let mut vec = Vec::<ActiveRankRecord<B>>::new();
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
