use std::marker::PhantomData;

use j4rs::{errors::J4RsError, Instance, InvocationArg};
use jbuchong::{
    java_type, AsInstanceTrait, GetInstanceTrait, IntoArgTrait, NewType, ToArgTrait,
    TryFromInstanceTrait,
};

pub struct DefaultConvert;
pub struct PrimitiveConvert;

#[derive(NewType)]
#[java_type("java.lang.String", Data = &str, Marker = DefaultConvert)]
#[java_type("java.lang.String", Data = String, Marker = DefaultConvert)]
#[java_type("java.lang.Boolean", Data = bool, Marker = DefaultConvert)]
#[java_type("java.lang.Character", Data = char, Marker = DefaultConvert)]
#[java_type("java.lang.Byte", Data = i8, Marker = DefaultConvert)]
#[java_type("java.lang.Short", Data = i16, Marker = DefaultConvert)]
#[java_type("java.lang.Integer", Data = i32, Marker = DefaultConvert)]
#[java_type("java.lang.Long", Data = i64, Marker = DefaultConvert)]
#[java_type("java.lang.Float", Data = f32, Marker = DefaultConvert)]
#[java_type("java.lang.Double", Data = f64, Marker = DefaultConvert)]
pub struct DataWrapper<Data, Marker = DefaultConvert> {
    data: Data,
    _m: PhantomData<Marker>,
}

impl<Data: ToArgTrait, Marker> ToArgTrait for DataWrapper<Data, Marker> {
    default fn to_arg(&self) -> Result<InvocationArg, J4RsError> {
        self.data.to_arg()
    }
}

impl<Data: IntoArgTrait, Marker> IntoArgTrait for DataWrapper<Data, Marker> {
    default fn into_arg(self) -> Result<InvocationArg, J4RsError> {
        self.data.into_arg()
    }
}
impl<Data: ToArgTrait> ToArgTrait for DataWrapper<Data, PrimitiveConvert> {
    fn to_arg(&self) -> Result<InvocationArg, J4RsError> {
        self.data.to_arg()?.into_primitive()
    }
}

impl<Data: IntoArgTrait> IntoArgTrait for DataWrapper<Data, PrimitiveConvert> {
    fn into_arg(self) -> Result<InvocationArg, J4RsError> {
        self.data.into_arg()?.into_primitive()
    }
}

impl<Data: GetInstanceTrait, Marker> GetInstanceTrait for DataWrapper<Data, Marker> {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        self.data.get_instance()
    }
}
impl<Data: AsInstanceTrait, Marker> AsInstanceTrait for DataWrapper<Data, Marker> {
    fn as_instance(&self) -> &Instance {
        self.data.as_instance()
    }
}

impl<Data: TryFromInstanceTrait, Marker> TryFromInstanceTrait for DataWrapper<Data, Marker> {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        Data::try_from_instance(instance).map(Self::from)
    }
}
