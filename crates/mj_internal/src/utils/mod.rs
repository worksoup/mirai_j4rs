pub mod contact;
pub mod login_solver;
pub mod other;

use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::GetClassTypeTrait;
use mj_base::{
    env::{FromInstance, GetInstanceTrait},
    utils::instance_is_null,
};
use mj_closures::{
    comparator::Comparator, consumer::Consumer, function::Function, predicate::Predicate,
};
use std::{cmp::Ordering, marker::PhantomData};
pub trait MiraiRsCollectionTrait {
    type Element;
    fn get_size(&self) -> i32;
    fn is_empty(&self) -> bool;
    fn contains(&self, element: &Self::Element) -> bool;
    fn contains_all(&self, elements: Self) -> bool;
}

pub trait MiraiRsIterableTrait: Iterator {}

/// 对应 Stream<AbsoluteFileFolder>
pub struct JavaStream<T: FromInstance + GetClassTypeTrait> {
    pub(crate) instance: Instance,
    pub(crate) _unused: PhantomData<T>,
}

impl<T: FromInstance + GetClassTypeTrait> GetInstanceTrait for JavaStream<T> {
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}

impl<T: FromInstance + GetClassTypeTrait> FromInstance for JavaStream<T> {
    fn from_instance(instance: Instance) -> Self {
        JavaStream {
            instance,
            _unused: PhantomData::default(),
        }
    }
}

impl<T: FromInstance + GetClassTypeTrait> JavaStream<T> {
    pub fn sorted_array_by<F>(&self, compare: F) -> Vec<T>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut array = self.to_vec();
        array.sort_by(compare);
        array
    }
    pub fn filter<P>(&self, p: &P) -> JavaStream<T>
    where
        P: Fn(T) -> bool,
        T: FromInstance,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let p = Predicate::new(p);
        let predicate = InvocationArg::try_from(p.to_instance()).unwrap();
        let instance = jvm.invoke(&self.instance, "filter", &[predicate]).unwrap();
        let _ = p.drop_and_to_raw();
        JavaStream::from_instance(instance)
    }

    pub fn map<B: FromInstance + GetClassTypeTrait, F>(&self, f: &F) -> JavaStream<B>
    where
        F: Fn(T) -> B,
        T: FromInstance,
        B: GetInstanceTrait + FromInstance,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let f = Function::new(f);
        let mapper = InvocationArg::try_from(f.to_instance()).unwrap();
        let instance = jvm.invoke(&self.instance, "map", &[mapper]).unwrap();
        let _ = f.drop_and_to_raw();
        JavaStream::from_instance(instance)
    }

    pub fn for_each<F>(&self, f: &F)
    where
        F: Fn(T),
    {
        let jvm = Jvm::attach_thread().unwrap();
        let f = Consumer::new(f);
        let predicate = InvocationArg::try_from(f.to_instance()).unwrap();
        let _ = jvm.invoke(&self.instance, "filter", &[predicate]).unwrap();
        let _ = f.drop_and_to_raw();
    }

    pub fn count(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "count", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }

    pub fn flat_map<U: FromInstance + GetClassTypeTrait, F>(&self, f: &F) -> JavaStream<U>
    where
        F: Fn(T) -> JavaStream<U>,
        T: FromInstance,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let f = Function::new(f);
        let mapper = InvocationArg::try_from(f.to_instance()).unwrap();
        let instance = jvm.invoke(&self.instance, "flatMap", &[mapper]).unwrap();
        let _ = f.drop_and_to_raw();
        JavaStream::from_instance(instance)
    }

    pub fn skip(&self, n: i64) -> JavaStream<T> {
        let jvm = Jvm::attach_thread().unwrap();
        let n = InvocationArg::try_from(n)
            .unwrap()
            .into_primitive()
            .unwrap();
        let instance = jvm.invoke(&self.instance, "skip", &[n]).unwrap();
        JavaStream::from_instance(instance)
    }

    pub fn limit(&self, max_size: i64) -> JavaStream<T> {
        let jvm = Jvm::attach_thread().unwrap();
        let max_size = InvocationArg::try_from(max_size)
            .unwrap()
            .into_primitive()
            .unwrap();
        let instance = jvm.invoke(&self.instance, "limit", &[max_size]).unwrap();
        JavaStream::from_instance(instance)
    }

    pub fn max_by<F>(&self, f: &F) -> Option<T>
    where
        F: Fn(T, T) -> Ordering,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let f = Comparator::new(f);
        let compare = InvocationArg::try_from(f.to_instance()).unwrap();
        let instance = jvm.invoke(&self.instance, "max", &[compare]).unwrap();
        let _ = f.drop_and_to_raw();
        if !instance_is_null(&instance) {
            Some(T::from_instance(instance))
        } else {
            None
        }
    }

    pub fn min_by<F>(&self, f: &F) -> Option<T>
    where
        F: Fn(T, T) -> Ordering,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let f = Comparator::new(f);
        let compare = InvocationArg::try_from(f.to_instance()).unwrap();
        let instance = jvm.invoke(&self.instance, "min", &[compare]).unwrap();
        let _ = f.drop_and_to_raw();
        if !instance_is_null(&instance) {
            Some(T::from_instance(instance))
        } else {
            None
        }
    }
    pub fn to_vec(&self) -> Vec<T> {
        let jvm = Jvm::attach_thread().unwrap();
        let mut array = Vec::new();
        let instance = jvm.invoke(&self.instance, "toList", &[]).unwrap();
        let instance = jvm.invoke(&instance, "iterator", &[]).unwrap();
        loop {
            let has_next: bool = jvm
                .to_rust(jvm.invoke(&instance, "hasNext", &[]).unwrap())
                .unwrap();
            if has_next {
                let next = jvm.invoke(&instance, "next", &[]).unwrap();
                array.push(T::from_instance(T::cast_to_this_type(next)))
            } else {
                break;
            }
        }
        array
    }
}
