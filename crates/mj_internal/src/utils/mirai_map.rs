use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::{TryFromInstanceTrait, GetInstanceTrait};
use std::collections::HashMap;
use j4rs::errors::J4RsError;

pub struct MiraiMap<K, V> {
    pub(crate) instance: Instance,
    pub(crate) _t: Option<HashMap<K, V>>,
}
impl<K, V> TryFromInstanceTrait for MiraiMap<K, V> {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        Ok(MiraiMap { instance, _t: None })
    }
}
impl<K, V> MiraiMap<K, V> {
    //顺序复制为rust HashMap.
    pub fn to_hash_map_t(
        &self,
        cast: Box<
            dyn Fn(
                &Instance,                            //key
                &Instance,                            //value
                &Jvm,                                 //jvm
                &dyn Fn(&Instance, &str) -> Instance, //java中的类型转换。
            ) -> (K, V),
        >,
    ) -> HashMap<K, V>
    where
        K: Eq + PartialEq + std::hash::Hash,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let java_cast =
            |instance: &Instance, obj: &str| -> Instance { jvm.cast(&instance, obj).unwrap() };
        let mut map = HashMap::<K, V>::new();
        let entry_set = jvm
            .invoke(&self.instance, "entrySet", InvocationArg::empty())
            .unwrap();
        let it = jvm
            .invoke(&entry_set, "iterator", InvocationArg::empty())
            .unwrap();
        while jvm
            .chain(&it)
            .unwrap()
            .invoke("hasNext", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
        {
            let entry = jvm.invoke(&it, "next", InvocationArg::empty()).unwrap();
            let entry = java_cast(&entry, "java.util.Map$Entry");
            let k = jvm
                .invoke(&entry, "getKey", InvocationArg::empty())
                .unwrap();
            let v = jvm
                .invoke(&entry, "getValue", InvocationArg::empty())
                .unwrap();

            let ins = cast(&k, &v, &jvm, &java_cast);

            map.insert(ins.0, ins.1);
        }
        map
    }
}

//特化版本。
impl MiraiMap<i32, String> {
    pub fn to_hash_map(&self) -> HashMap<i32, String> {
        self.to_hash_map_t(Box::new(
            |k: &Instance,
             v: &Instance,
             jvm: &Jvm,
             cast: &dyn Fn(&Instance, &str) -> Instance|
             -> (i32, String) {
                let k: i64 = jvm.to_rust(cast(&k, "java.lang.Integer")).unwrap();
                let k: i32 = (k & i32::MAX as i64) as i32;
                let v: String = jvm.to_rust(cast(&v, "java.lang.String")).unwrap();
                (k, v)
            },
        ))
    }
}

//特化版本。
impl MiraiMap<String, i32> {
    pub fn to_hash_map(&self) -> HashMap<String, i32> {
        self.to_hash_map_t(Box::new(
            |k: &Instance,
             v: &Instance,
             jvm: &Jvm,
             cast: &dyn Fn(&Instance, &str) -> Instance|
             -> (String, i32) {
                let k: String = jvm.to_rust(cast(&k, "java.lang.String")).unwrap();
                let v: i64 = jvm.to_rust(cast(&v, "java.lang.Integer")).unwrap();
                let v: i32 = (v & i32::MAX as i64) as i32;
                (k, v)
            },
        ))
    }
}

//特化版本。该版本不应当使用。
impl MiraiMap<String, String> {
    pub fn to_hash_map(&self) -> HashMap<String, String> {
        self.to_hash_map_t(Box::new(
            |k: &Instance,
             v: &Instance,
             jvm: &Jvm,
             cast: &dyn Fn(&Instance, &str) -> Instance|
             -> (String, String) {
                let k: String = jvm
                    .to_rust(jvm.invoke(&k, "toString", InvocationArg::empty()).unwrap())
                    .unwrap();
                let v: String = jvm.to_rust(cast(&v, "java.lang.String")).unwrap();
                (k, v)
            },
        ))
    }
}

impl<K, V> GetInstanceTrait for MiraiMap<K, V> {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        Ok(Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap())
    }
}
