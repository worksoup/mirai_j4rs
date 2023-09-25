# ɒiM_J

**ɒiM_J is Mirai_J4rs.**

使用 [`j4rs`](https://crates.io/crates/j4rs) 库简易(陋)地封装了一下 [`Mirai`](https://docs.mirai.mamoe.net/), api
基本与 `Mirai` 保持一致。

同时集成了 [`fix-protocol-version`](https://github.com/cssxsh/fix-protocol-version) 以解决登陆问题。

一直都是自己的其他项目在用，所以完全没有什么设计可言，也没有完全覆盖 `Mirai` 的功能。

当然也欢迎使用本库，既然打算开源出来自然也打算维护它……~~如果我能力够的话。~~

本仓库仅含 rust 代码，本项目的 jvm_side 请见于[此](https://github.com/worksoup/mirai_j4rs_jvm_side)。

## 协议支持

### 拟跟进的协议列表

**消息相关**

- [x] `At` 提及群员
- [x] `AtAll` 提及全体成员
- [x] `Face` 表情消息
- [x] `FileMessage` 群文件消息
- [x] `ForwardMessage` 合并转发
- [x] `Image`（包括自定义表情）
- [ ] `LightApp` 小程序
- [x] `MarketFaceTrait` 市场表情
    - [x] `Dice` 🎲
    - [x] `RockPaperScissors` 锤子包袱剪
    - [x] `MarketFace` 其他市场表情
- [x] `Nudge` 戳一戳
    - [x] `BotNudge`
    - [x] `FriendNudge`
    - [x] `MemberNudge`
- [x] `PokeMessage` 放大招等
- [x] `PlainText` 文本消息
- [x] `QuoteReply` 回复消息
- [ ] `VipFace` Vip表情
- [ ] XML，JSON 等富文本消息
- [x] 长消息（4500 字符 + 50 图片）<!--存疑。-->
- [x] 撤回
    - [x] 撤回发送的消息
- [x] 撤回群员消息
- [ ] 语音
- [ ] 闪照
- [ ] 自定义消息
- [ ] 音乐分享
- [ ] ~~短视频~~
- [ ] ~~超级表情~~

**注意**：当前的开发基于 Mirai 2.15.0, 会在其他部分完善后再升级 Mirai 版本。升级后才会支持超级表情和短视频。

**群相关**

- [x] 群列表
- [ ] 成员列表
- [ ] 群员权限
- [x] 禁言
- [ ] 群公告管理
- [ ] 群设置
    - [x] 全体禁言
    - [ ] 自动审批
    - [ ] 入群公告
    - [x] 成员邀请
    - [x] 匿名聊天
- [ ] 处理入群申请
- [x] 移除群员
- [x] 群文件

**好友相关**

- [x] 好友列表
- [x] `NewFriendRequestEvent` 处理新好友申请
- [x] 删除好友

**其他客户端**

- [ ] 同步其他客户端的消息
- [ ] 向其他客户端发送消息

### 不会支持的协议

- 金钱相关，如点赞、收付款
- 敏感操作，如主动添加好友、主动加入群、主动邀请好友加群
- 安全相关，获取账号登录凭证(token，cookie等)

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
use mirai_j4rs::prelude::*; // prelude 还没写。请自行导入所需的模块等。
fn main(){
    let bot = BotBuilder::new()
        .id(i64/*这里是你机器人的 id.*/)
        // 两种密码二选一，如果都有优先使用明文密码登陆。
        // 暂不支持 Mirai 2.15.0 的二维码登陆。
        .password(String/*这里是你的明文密码。*/)
        .password([u8;16]/*这里是你的密码的 MD5.*/)
        // 这些配置函数几乎一一对应于 Mirai 中
        // BotConfiguration 类，只是 mirai_j4rs 均使用蛇形命名法。
        // 对于一些在 Mirai 中有可选参数的函数，以 Option 值代替。
        .file_based_device_info(None)
        // 集成了 fix_protocol_version, 需要调用相关方法使其生效。
        .fix_protocol_version_fetch(MiraiProtocol::A, "latest".to_string())
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
    // env 中一般是一些集成进去的插件（暂时只有 fix-protocol-version 插件）的功能。
    env.fix_protocol_version_fetch(MiraiProtocol::A, "latest".to_string());
    // 而 config 则是由 BotConfiguration 类提供的配置项。
    let config = env.new_bot_configuration();
    config.file_based_device_info(None);
    config.setProtocol(MiraiProtocol::W);
    // 此处实现了两个 trait, 明文密码和 md5 密码是同名的函数。
    // TODO: 添加 passwprd_md5 函数，移除这两个特征，使函数功能更加明确。
    let bot = env.new_bot(id, password, config);
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

## 对于一些枚举值的说明：

``` rust
/// 心跳策略的枚举值。
pub enum HeartbeatStrategy {
    /// `HeartbeatStrategy.STAT_HB`
    S,
    /// `HeartbeatStrategy.REGISTER`
    R,
    /// `HeartbeatStrategy.NONE`
    N,
}
```

``` rust
/// 登陆协议的枚举值。
pub enum MiraiProtocol {
    /// `MiraiProtocol.ANDROID_PHONE`
    A,
    /// `MiraiProtocol.ANDROID_PAD`
    P,
    /// `MiraiProtocol.ANDROID_WATCH`
    W,
    /// `MiraiProtocol.IPAD`
    I,
    /// `MiraiProtocol.MACOS`
    M,
}
```

``` rust
/// 此枚举值用于获取头像链接时指定图片规格。
#[derive(IntoPrimitive)]
#[repr(i32)]
pub enum AvatarSpec {
XS = 40,      //SMALLEST(40),
S = 41,       //SMALL(41),
M = 100,      //MEDIUM(100),
L = 140,      //LARGE(140),
XL = 640,     //LARGEST(640),
ORIGINAL = 0, //ORIGINAL(0);
}
```

# 关于开源协议

根据 Mirai 的协议和强烈建议，以及 `fix-protocol-version` 的协议，此项目应当是 `AGPL`.
同样地，本项目严禁用于商业用途并强烈建议直接或间接接触到本软件的项目使用 `AGPL` 协议进行分发（以该协议原文和 `Mirai`
相关附加条款为准）。
