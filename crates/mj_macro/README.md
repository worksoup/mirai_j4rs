# mj_macro

这里存放了开发 `mirai_j4rs` 时用到的一些宏。

## Derive

### `GetInstanceDerive`

为特定的结构体和枚举类型实现 `GetInstanceTrait`.

这些类型需要满足如下条件：

- 结构体必须拥有 `instance: j4rs::Instance,` 字段。
- 枚举值则必须类似于此：
  ```rust
  enum Enum{
    A(AType),
    B(BType),
  }
  ```
  并且如上代码，`AType` 和 `BType` 都必须实现 `GetInstanceTrait`.

### `AsInstanceDerive`

与上类似。

### `FromInstanceDerive`

为特定的结构体和枚举类型实现 `FromInstanceTrait`.

这些类型需要满足如下条件：

- 结构体必须拥有 `instance: j4rs::Instance,` 字段，且其余字段必须都是 `PhantomData` 类型。
- 枚举值则必须类似于此：

  ```rust
  enum Enum{
    A(AType),
    #[fall] // 使用 `FromInstanceDerive` 时可选为分支添加 `fall` 属性。
    B(BType),
  }
  ```

  并且如上代码，`AType` 和 `BType` 都必须实现 `FromInstanceTrait`.
  其中 `fall` 意味着未能成功转换的类型将会落到该分支中。如果没有该属性，未能成功转换时将会造成 `panic!`, 如果存在多个，则最后一个有效。

### `MiraiEventDerive`

为结构体和枚举类型实现 `MiraiEventTrait`.

对结构体或枚举等没有特殊要求。`MiraiEventTrait` 特型会有部分要求，请参看 `mj_internal` 代码。

## 属性

### `java_type`

为结构体、枚举等实现 `GetClassTypeTrait`.

接受一个字符串字面值参数，类似于此：

```rust
#[java_type("rt.lea.LumiaUtils")]
struct LumiaUtils{}
```

对结构体或枚举等没有特殊要求。

### `mj_all`

同时应用`GetInstanceDerive`, `AsInstanceDerive`, `FromInstanceDerive` 和 `java_type`.

接受一个字符串字面值参数传递给 `java_type` 属性。

### `mj_event`

根据结构体名称应用 `mj_all` 和 `MiraiEventDerive`. 类似于此：

```rust
#[mj_event]
pub struct FriendAddEvent {
    instance: Instance,
}

// 相当于下述代码：
// 这里的前缀是固定的。
#[mj_macro::mj_all("net.mamoe.mirai.event.events.FriendAddEvent")]
pub struct FriendAddEvent {
    instance: Instance,
}
```

也可以接受一个字符串字面值参数传递给 `java_type` 属性，避免生成默认值。

### `mj_event_without_default_traits`

与 `mj_event` 类似，只是没有应用 `MiraiEventDerive`.
