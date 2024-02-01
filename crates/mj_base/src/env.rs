use j4rs::Instance;

/// 这个特征可以返回 java 中 Class 对象，监听事件的时候用。
/// 为了做泛型搞的。之后可能会改动。
pub trait GetClassTypeTrait {
    /// 获取该类在 `Java` 中的 `Class` 对象。
    fn get_class_type() -> Instance;

    fn cast_to_this_type(instance: Instance) -> Instance;
}

pub trait GetEnvTrait {
    fn get_instance(&self) -> Instance;
}

/// 通过 `j4rs::Instance` 获得当前结构体。
pub trait FromInstance {
    fn from_instance(instance: Instance) -> Self;
}
