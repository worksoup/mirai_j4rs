use j4rs::errors::J4RsError;
use j4rs::{Instance, InvocationArg, Jvm};
use std::cmp::Ordering;

/// 这个特征可以返回 java 中 Class 对象，监听事件的时候用。
/// 为了做泛型搞的。之后可能会改动。
pub trait GetClassTypeTrait {
    /// 获取该类在 `Java` 中的 `Class` 对象。
    fn get_class_type() -> Instance;

    fn cast_to_this_type(instance: Instance) -> Instance;

    fn get_type_name() -> String;

    fn is_this_type(instance: &Instance) -> bool;
}

pub trait GetInstanceTrait {
    fn get_instance(&self) -> Result<Instance, J4RsError>;
}
impl GetInstanceTrait for bool {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        InvocationArg::try_from(self).map(|a| Instance::try_from(a).unwrap())
    }
}
impl GetInstanceTrait for Ordering {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        Instance::try_from(InvocationArg::try_from(unsafe {
            *{ self as *const Ordering as *const i8 }
        })?)
    }
}
impl<P1, P2> GetInstanceTrait for (P1, P2)
where
    P1: GetInstanceTrait,
    P2: GetInstanceTrait,
{
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let v1 = InvocationArg::from(self.0.get_instance()?);
        let v2 = InvocationArg::from(self.1.get_instance()?);
        jvm.create_instance("rt.lea.LumiaPair", &[v1, v2])
    }
}
pub trait AsInstanceTrait {
    fn as_instance(&self) -> &Instance;
}

/// 通过 `j4rs::Instance` 获得当前结构体。
pub trait TryFromInstanceTrait: Sized {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError>;
}
/// 通过 `j4rs::Instance` 获得当前结构体。
pub trait FromInstanceTrait: Sized {
    fn from_instance(instance: Instance) -> Self;
}
impl<T: TryFromInstanceTrait> FromInstanceTrait for T {
    default fn from_instance(instance: Instance) -> Self {
        Self::try_from_instance(instance).unwrap()
    }
}
impl TryFromInstanceTrait for bool {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        Jvm::attach_thread()?.to_rust(instance)
    }
}

impl<P1, P2> TryFromInstanceTrait for (P1, P2)
where
    P1: TryFromInstanceTrait,
    P2: TryFromInstanceTrait,
{
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        let jvm = Jvm::attach_thread()?;
        let instance = jvm.cast(&instance, "rt.lea.LumiaPair")?;
        let val1 = jvm.invoke(&instance, "first", InvocationArg::empty())?;
        let val2 = jvm.invoke(&instance, "second", InvocationArg::empty())?;
        let val1 = P1::try_from_instance(val1)?;
        let val2 = P2::try_from_instance(val2)?;
        Ok((val1, val2))
    }
}

impl TryFromInstanceTrait for Ordering {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        let cmp_result: i32 = Jvm::attach_thread().unwrap().to_rust(instance).unwrap();
        Ok(cmp_result.cmp(&0))
    }
}
