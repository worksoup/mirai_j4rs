# ɒiM_J
 ɒiM_J is Mirai_J4rs.
 使用 `j4rs` 库简易(陋)地封装了一下 `Mirai`, api 与 `Mirai` 保持一致。

 一直都是仅供自己的其他项目使用，所以完全没有什么设计可言，也没有完全覆盖 `Mirai` 的功能。

 感觉免费去使用这个库。既然打算开源出来自然也打算维护它的……如果我能力够的话。

 # 使用方法：
 在 `Cargo.toml` 中添加：
 ``` toml
[dependencies]
mirai_j4rs="*"
 ```

 ## 机器人
 第一种方法，适用于单个 `Bot`.
``` rust
use mirai_j4rs::prelude::*;
fn main(){
    let bot = BotBuilder::new()
        .id(i64/*这里是你机器人的 id.*/)
        .password(String/*这里是你的明文密码。*/)
        .password([u8;16]/*这里是你的密码的 MD5.*/)
        // 上述两种密码二选一，如果都有优先使用明文密码登陆。
        // 暂不支持 Mirai 2.15.0 的二维码登陆。
        .file_based_device_info(None)
        // 这些配置函数几乎一一对应于 Mirai 中
        // BotConfiguration 类，只是 mirai_j4rs 均使用
        // 蛇形命名法。
        // 对于一些在 Mirai 中有可选参数的函数，
        // 这里暂时是以传入 Option 值替代。
        .fix_protocol_version_fetch(MiraiProtocol::A, "latest".to_string())
        // 集成了 fix_protocol_version, 需要调用相关方法使其生效。
        .build();
    bot.login();
}
```
第二种方法，和 Mirai 类似，可配置项与 Mirai 相同，<s>应该</s>和 Mirai 行为一致。
``` rust
use mirai_j4rs::prelude::*;
fn main(){
    let env = mirai_j4rs::contact::bot::Env::new(jar_paths, java_opts);
    // 上述语句中的两参数均为 `Vec<String>` 类型。
    env.fix_protocol_version_fetch(MiraiProtocol::A, "latest".to_string());
    let config = env.new_bot_configuration();
    config.file_based_device_info(None);
    config.setProtocol(MiraiProtocol::W);
    // env 和 config 中各有一部分配置项。
    // env 中一般是原版 Mirai 中没有的配置项，如一些集成进去的插件等。
    // 而 config 则是包装了由 BotConfiguration 类提供的配置项。
    let bot = env.new_bot(id, password/*此处的密码两种类型均支持。*/, config);
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
/// Mirai 中定义：
/// ```
/// public enum class AvatarSpec(@MiraiInternalApi public val size: Int) : Comparable<AvatarSpec> {
///     //最高压缩等级
///     SMALLEST(40),
///
///     //群员列表中的显示大小, 实际上是 40 px, 但会比 [SMALLEST] 好一些
///     SMALL(41),
///
///     //联系人列表中的显示大小
///     MEDIUM(100),
///
///     //消息列表中的显示大小
///     LARGE(140),
///     
///     //联系人详情页面中的显示大小
///     LARGEST(640),
///
///     //原图
///     ORIGINAL(0);
/// }
/// ```
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
<!-- 等我写完项目再研究一下，暂时是 `MIT`（因为此代码平台默认添加了一个 `MIT` 的协议文件，懒得改了）。 根据 Mirai 的协议，此项目后续应当是 `AGPL`. -->
根据 Mirai 的协议，此项目应当是 `AGPL`.