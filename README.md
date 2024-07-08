# ɒiM_J

**ɒiM_J is Mirai_J4rs.**

使用 [`j4rs`](https://crates.io/crates/j4rs) 库简易(陋)地封装了一下 [`Mirai`](https://docs.mirai.mamoe.net/), api
基本与 `Mirai` 保持一致。

本仓库仅含 rust 代码，本项目的 jvm_side 请见于[此](https://github.com/worksoup/mirai_j4rs_jvm_side)。

## 登录相关

**由于之前签名服务受到重创，所以请谨慎使用。**

目前简单适配了 `Mirai` 的二维码登录。似乎可以正常登录。

**NOTE** 最后一次经过测试的版本应当是 `commit#cd555d6d`. 由于 mirai 没什么动静了，故本项目将不再开发。

## 协议支持

请查看该 [issue](https://github.com/worksoup/mirai_j4rs/issues/2#issue-2114138266).

## 使用方法
在 `Cargo.toml` 中添加：

 ``` toml
[dependencies]
mirai_j4rs={ git = "https://github.com/worksoup/mirai_j4rs.git" }
 ```

可以参考 [示例](./examples)。

## 关于开源协议

根据 Mirai 的协议和强烈建议，以及 `fix-protocol-version` 的协议，此项目应当是 `AGPL`.
同样地，本项目严禁用于商业用途并强烈建议直接或间接接触到本软件的项目使用 `AGPL` 协议进行分发（以该协议原文和 `Mirai`
相关附加条款为准）。
