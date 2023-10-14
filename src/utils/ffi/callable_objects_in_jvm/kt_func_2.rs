use crate::env::{FromInstance, GetEnvTrait};
use crate::utils::internal::data_wrapper::DataWrapper;
use crate::utils::internal::instance_from_i8_16;
use j4rs::{Instance, InvocationArg, Jvm};
use std::{marker::PhantomData, mem::transmute};

pub struct KtFunc2<'a, P1, P2, R>
    where
        P1: FromInstance,
        P2: FromInstance,
        R: GetEnvTrait + FromInstance,
{
    instance: Instance,
    internal_closure_raw: [i8; 16],
    _p1: PhantomData<P1>,
    _p2: PhantomData<P2>,
    __a: PhantomData<&'a ()>,
    _r: PhantomData<R>,
}

impl<'a, P1, P2, R> KtFunc2<'a, P1, P2, R>
    where
        P1: FromInstance,
        P2: FromInstance,
        R: GetEnvTrait + FromInstance,
{
    #[inline]
    fn internal_closure_as_i8_16<F: Fn(P1, P2) -> R, >(f: &'a F) -> [i8; 16] {
        let call_from_java = |value: DataWrapper<Instance>| -> Instance {
            let value = value.get::<DataWrapper<(P1, P2)>>();
            let (val1, val2) = value.get_pair();
            f(val1, val2).get_instance()
        };
        let call_from_java_raw: *mut dyn Fn(DataWrapper<Instance>) -> Instance = Box::into_raw(Box::new(call_from_java));
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    pub fn new<F: Fn(P1, P2) -> R, >(closure: &'a F) -> KtFunc2<'a, P1, P2, R> {
        let jvm = Jvm::attach_thread().unwrap();
        let internal_closure_raw = Self::internal_closure_as_i8_16(closure);
        let kt_func_2 = instance_from_i8_16::<"rt.lea.LumiaFunction">(internal_closure_raw);
        let instance = jvm
            .create_instance(
                "rt.lea.LumiaKtFunc2",
                &[InvocationArg::try_from(kt_func_2).unwrap()],
            )
            .unwrap();
        KtFunc2 {
            instance,
            internal_closure_raw,
            _p1: PhantomData::default(),
            _p2: PhantomData::default(),
            __a: PhantomData::default(),
            _r: PhantomData::default(),
        }
    }
    pub fn to_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
    pub fn invoke(&self, val1: InvocationArg, val2: InvocationArg) -> R {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm.invoke(&self.instance, "invoke", &[val1, val2]).unwrap();
        R::from_instance(result)
    }

    pub fn get_internal_closure_raw(&self) -> *mut dyn Fn(DataWrapper<Instance>) -> Instance {
        unsafe { transmute(self.internal_closure_raw) }
    }
    pub(super) fn drop_internal_closure_raw(&self) {
        let _boxed = unsafe { Box::from_raw(self.get_internal_closure_raw()) };
    }
}