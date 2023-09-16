pub mod ffi;
pub(crate) mod internal;

use crate::env::{FromInstance, GetEnvTrait};
use crate::file::{AbsoluteFile, AbsoluteFileFolder, AbsoluteFolder};
use crate::message::message_trait::SingleMessageTrait;
use crate::utils::internal::is_instance_of;
use j4rs::{Instance, InvocationArg, Jvm};
use std::cmp::Ordering;
use std::marker::PhantomData;
use contact_derive::GetInstanceDerive;

pub trait MiraiRsCollectionTrait {
    type Element;
    fn get_size(&self) -> i32;
    fn is_empty(&self) -> bool;
    fn contains(&self, element: &Self::Element) -> bool;
    fn contains_all(&self, elements: Self) -> bool;
}

pub trait MiraiRsIterableTrait: Iterator {}

pub struct FileFolderStream<T> {
    instance: Instance,
    _unused: PhantomData<T>,
}

impl<T> GetEnvTrait for FileFolderStream<T> {
    fn get_instance(&self) -> j4rs::Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}

impl<T> FileFolderStream<T> {
    pub fn sorted_array_by<F>(&self, mut compare: F) -> Vec<AbsoluteFileFolder>
        where
            F: FnMut(&T, &T) -> Ordering,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let mut array = Vec::new();
        let instance = jvm.invoke(&self.instance, "toList", &[]).unwrap();
        loop {
            let has_next: bool = jvm
                .to_rust(jvm.invoke(&instance, "hasNxt", &[]).unwrap())
                .unwrap();
            if has_next {
                let next = jvm.invoke(&instance, "next", &[]).unwrap();
                if is_instance_of(&next, "net.mamoe.mirai.contact.file.AbsoluteFile") {
                    array.push(AbsoluteFileFolder::AbsoluteFile(AbsoluteFile {
                        instance: next,
                    }))
                } else {
                    array.push(AbsoluteFileFolder::AbsoluteFolder(AbsoluteFolder {
                        instance: next,
                    }))
                }
            } else {
                break;
            }
        }
        array
    }
    // 我觉得我可以直接假定没有重复文件。
    // pub fn distinct(&self) {
    //     ()
    // }
    pub fn close(self) {
        todo!()
    }
    pub fn filter<P>(&self, predicate: P) -> FileFolderStream<T>
        where
            P: FnMut(&T) -> bool,
    {
        todo!()
    }

    pub fn map<B, F>(&self, f: F) -> FileFolderStream<B>
        where
            F: FnMut(T) -> B,
    {
        todo!()
    }

    pub fn for_each<F>(&self, f: F)
        where
            F: FnMut(T),
    {
        todo!()
    }

    pub fn count(&self) -> i64 {
        todo!()
    }
    pub fn collect<B: FromIterator<T>>(&self) -> B {
        todo!()
    }
    pub fn find_first<P>(&self, predicate: P) -> Option<T>
        where
            P: FnMut(&T) -> bool,
    {
        todo!()
    }
    pub fn find_any<P>(&self, predicate: P) -> Option<T>
        where
            P: FnMut(&T) -> bool,
    {
        todo!()
    }

    pub fn flat_map<U, F>(&self, f: F) -> FileFolderStream<U>
        where
            F: FnMut(T) -> FileFolderStream<U>,
    {
        todo!()
    }

    pub fn skip(&self, n: i64) -> FileFolderStream<T> {
        todo!()
    }

    pub fn limit(&self, max_size: i64) -> FileFolderStream<T> {
        todo!()
    }

    pub fn max_by<F>(&self, compare: F) -> Option<T>
        where
            F: FnMut(&T, &T) -> Ordering,
    {
        todo!()
    }

    pub fn min_by<F>(&self, compare: F) -> Option<T>
        where
            F: FnMut(&T, &T) -> Ordering,
    {
        todo!()
    }
    pub fn to_vec(&self) -> Vec<T> {
        todo!()
    }
}