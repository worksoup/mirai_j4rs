# ɒiM_J

**ɒiM_J is Mirai_J4rs.**

**近期听闻签名服务受到重创，由于本项目还未较好地适配安卓手表协议，所以请谨慎使用。**
目前简单适配了 Mirai 的二维码登录。
可以使用 `j4rs` 库直接操作 Jvm 对象使用该协议。

使用 [`j4rs`](https://crates.io/crates/j4rs) 库简易(陋)地封装了一下 [`Mirai`](https://docs.mirai.mamoe.net/), api
基本与 `Mirai` 保持一致。

~~同时集成了 [`fix-protocol-version`](https://github.com/cssxsh/fix-protocol-version) 以解决登陆问题。~~ 移除了。

一直都是自己的其他项目在用，所以完全没有什么设计可言，也没有完全覆盖 `Mirai` 的功能。

当然也欢迎使用本库，既然打算开源出来自然也打算维护它……~~如果我能力够的话。~~

本仓库仅含 rust 代码，本项目的 jvm_side 请见于[此](https://github.com/worksoup/mirai_j4rs_jvm_side)。

## 协议支持

请查看 该 [issue](https://github.com/worksoup/mirai_j4rs/issues/2#issue-2114138266).

# 使用方法：

可以参考 [demo.md](./demo.md).

在 `Cargo.toml` 中添加：

 ``` toml
[dependencies]
mirai_j4rs={ git = "https://github.com/worksoup/mirai_j4rs.git" }
 ```

## 机器人

第一种方法，适用于单个 `Bot`（因为内部没有措施防止 `Env` 重复定义）。

``` rust
use mirai_j4rs::prelude::*; // prelude 还没写。请自行导入所需的模块。
fn main(){
    let bot = BotBuilder::new()
        .id(i64/*这里是你机器人的 id.*/)
        // 通过 `BotAuthorization` 枚举选择登录方式，可选：`Password`, `Md5`, `QrCode`
        .authorization(/*这里选择登录方式，为 `BotAuthorization` 枚举。*/)
        // 这些配置函数几乎一一对应于 Mirai 中
        // BotConfiguration 类，只是 mirai_j4rs 均使用蛇形命名法。
        // 对于一些在 Mirai 中有可选参数的函数，以 Option 值代替。
        .file_based_device_info(None)
        .build();
    bot.login();
}
```

第二种方法，和 Mirai 类似，可配置项与 Mirai 相同，<s>应该</s>和 Mirai 行为一致。

``` rust
use mirai_j4rs::prelude::*;
fn main(){
    // 以下语句中的两参数均为 `Vec<String>` 类型。
    let env = mirai_j4rs::contact::bot::Env::new(jar_paths, java_opts);
    // env 和 config 中各有一部分配置项。
    // env 中一般是一些集成进去的插件（暂时没有额外集成的插件）的功能。
    // 而 config 则是由 BotConfiguration 类提供的配置项。
    let config = env.new_bot_configuration();
    config.file_based_device_info(None);
    config.setProtocol(MiraiProtocol::W);
    let bot = env.new_bot_with_configuration(id, bot_authorization, config);
    bot.login();
}
```

## 事件

``` rust
let event_channel = bot.get_event_channel();
// 回调函数：
let on_group_message_event: Box<dyn Fn(GroupMessageEvent)> =
    Box::new(
        |event: GroupMessageEvent|{
            /*做你想做的事情。*/
        }
    );
// 监听并获取 Listener:
let listener_for_group_message_event = event_channel.subscribe_always(&on_group_message_event);

// NOTICE:
// 这里的 api 极有发生可能改动。
```

# 关于开源协议

根据 Mirai 的协议和强烈建议，以及 `fix-protocol-version` 的协议，此项目应当是 `AGPL`.
同样地，本项目严禁用于商业用途并强烈建议直接或间接接触到本软件的项目使用 `AGPL` 协议进行分发（以该协议原文和 `Mirai`
相关附加条款为准）。
