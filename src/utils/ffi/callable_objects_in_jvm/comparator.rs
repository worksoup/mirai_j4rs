use crate::env::{FromInstance, GetEnvTrait};
use crate::utils::internal::data_wrapper::DataWrapper;
use crate::utils::internal::instance_from_i8_16;
use j4rs::{Instance, InvocationArg, Jvm};
use std::{cmp::Ordering, marker::PhantomData, mem::transmute, pin::Pin};

pub struct Comparator<'a, T, F>
    where
        T: FromInstance,
        F: Fn(T, T) -> Ordering,
{
    instance: Instance,
    internal_closure_raw: [i8; 16],
    _t: PhantomData<T>,
    _f: PhantomData<&'a F>,
}

impl<'a, T, F> GetEnvTrait for Comparator<'a, T, F>
    where
        T: FromInstance,
        F: Fn(T, T) -> Ordering,
{
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}

impl<'a, T, F> Comparator<'a, T, F>
    where
        T: FromInstance,
        F: Fn(T, T) -> Ordering,
{
    #[inline]
    fn internal_closure_as_i8_16(f: &'a F) -> [i8; 16] {
        let call_from_java = Box::new(|value: DataWrapper<Instance>| -> Instance {
            let value = value.get::<DataWrapper<(T, T)>>();
            let (val1, val2) = value.get_pair();
            let ordering = f(val1, val2);
            match ordering {
                Ordering::Less => Instance::try_from(InvocationArg::try_from(-1).unwrap()).unwrap(),
                Ordering::Equal => Instance::try_from(InvocationArg::try_from(0).unwrap()).unwrap(),
                Ordering::Greater => {
                    Instance::try_from(InvocationArg::try_from(1).unwrap()).unwrap()
                }
            }
        });
        let call_from_java_raw: *mut dyn Fn(DataWrapper<Instance>) -> Instance =
            Box::into_raw(call_from_java);
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    pub fn new(closure: &'a F) -> Pin<Box<Comparator<'a, T, F>>> {
        let jvm = Jvm::attach_thread().unwrap();
        let internal_closure_raw = Self::internal_closure_as_i8_16(closure);
        let lumia_function = instance_from_i8_16::<"rt.lea.LumiaFunction">(internal_closure_raw);
        let instance = jvm
            .create_instance(
                "rt.lea.LumiaComparator",
                &[InvocationArg::try_from(lumia_function).unwrap()],
            )
            .unwrap();
        let comparator: Comparator<T, F> = Comparator {
            instance,
            internal_closure_raw,
            _t: PhantomData::default(),
            _f: PhantomData::default(),
        };
        Box::pin(comparator)
    }
    pub fn compare(&self, val1: InvocationArg, val2: InvocationArg) -> Ordering {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm
            .invoke(&self.instance, "compare", &[val1, val2])
            .unwrap();
        let cmp_result: i32 = jvm.to_rust(result).unwrap();
        cmp_result.cmp(&0)
    }
}

impl<'a, T, F> Drop for Comparator<'a, T, F>
    where
        T: FromInstance,
        F: Fn(T, T) -> Ordering,
{
    fn drop(&mut self) {
        let comparator: *mut dyn Fn(DataWrapper<Instance>) -> Instance =
            unsafe { transmute(self.internal_closure_raw) };
        let boxed = unsafe { Box::from_raw(comparator) };
        drop(boxed)
    }
}
