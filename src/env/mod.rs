use j4rs::Instance;

pub trait GetClassTypeTrait {
    fn get_class_type() -> Instance;
}

pub trait GetEnvTrait {
    fn get_instance(&self) -> Instance;
}

/// 通过 `j4rs::Instance` 获得当前结构体。
pub trait FromInstance {
    fn from_instance(instance: Instance) -> Self;
}