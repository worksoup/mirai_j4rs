```rust
use mirai_j4rs::contact::group::Group;
use mirai_j4rs::event::message::FriendMessageEvent;
use mirai_j4rs::message::message_trait::MarketFaceTrait;
use mirai_j4rs::message::{RockPaperScissors, SingleMessage};
use mirai_j4rs::{
    contact::{bot::BotBuilder, contact_trait::ContactOrBotTrait},
    event::{event_trait::MessageEventTrait, message::GroupMessageEvent},
    message::message_trait::{AbsoluteFileFloder, MessageTrait},
    other::enums::MiraiProtocol,
};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Deserialize, Serialize)]
pub(crate) struct BotInfo {
    pub(crate) bot_id: i64,
    pub(crate) bot_passwd: String,
}

/// 该函数接受一个 SingleMessage, 然后做出对应的反应。
fn match_single_message(msg: SingleMessage, contact: Option<Group>) {
    match msg {
        SingleMessage::At(at) => {
            println!("At {}", at.get_target())
        }
        SingleMessage::AtAll(at_all) => {
            println!("AtAll {}", at_all.to_string())
        }
        // 🎲 和剪子包袱锤似乎被下线了，不过还能被 Mirai 接受和发送。
        SingleMessage::Dice(dice) => {
            println!("🎲 {}", dice.get_value())
        }
        SingleMessage::Face(face) => {
            println!("表情 {}", face.get_id())
        }
        SingleMessage::FileMessage(file_message) => {
            // 好友的文件消息 Mirai 似乎不支持？
            if let Some(contact) = contact {
                // to_absolute_file 本不应当拿走 contact 的所有权，之后我会修改一下。
                println!(
                    "文件 {}",
                    file_message.to_absolute_file(contact).get_absolute_path()
                )
            }
        }
        SingleMessage::ForwardMessage(forward_message) => {
            println!("转发消息 {}", forward_message.to_string())
        }
        SingleMessage::Image(image) => {
            println!("图片 {}", image.query_url())
        }
        SingleMessage::LightApp(light_app) => {
            println!("小程序 {}", light_app.to_string())
        }
        SingleMessage::MarketFace(market_face) => {
            println!("其他市场表情 {}", market_face.get_name())
        }
        SingleMessage::MessageSource(message_source) => {
            println!("消息源 {}", message_source.to_string())
        }
        SingleMessage::MusicShare(music_share) => {
            println!("音乐分享 {}", music_share.to_string())
        }
        SingleMessage::PlainText(plain_text) => {
            println!("纯文本 {}", plain_text.get_content())
        }
        SingleMessage::PokeMessage(poke_message) => {
            println!("放大招之类的戳一戳消息 {}", poke_message.to_string())
        }
        SingleMessage::QuoteReply(quot_reply) => {
            println!("消息引用 {}", quot_reply.get_source().to_string())
        }
        SingleMessage::RockPaperScissors(rock_paper_scissors) => {
            if let Some(win) = rock_paper_scissors.eliminates(RockPaperScissors::random()) {
                if win {
                    println!("剪子包袱锤：赢麻了。");
                } else {
                    println!("剪子包袱锤：输惨了。");
                }
            } else {
                println!("剪子包袱锤：平局了。");
            }
        }
        SingleMessage::UnsupportedMessage(_) => {
            println!("不支持的消息！")
        }
        SingleMessage::VipFace(vip_face) => {
            println!("VIP表情 {}", vip_face.to_string())
        }
    }
}

fn main() {
    /// 如下结构体可在本文件找到定义：
    /// ``` rust
    /// #[derive(Deserialize, Serialize)]
    /// pub(crate) struct BotInfo {
    ///     pub(crate) bot_id: i64,
    ///     pub(crate) bot_passwd: String,
    /// }
    /// ```
    /// 所以 `bot_config.toml` 应当类似于：
    /// ```
    /// bot_id = 114514
    /// bot_passwd = "1919810"
    /// ```
    let bot_info: BotInfo = toml::from_str(
        std::fs::read_to_string("./bot_config.toml")
            .unwrap()
            .as_str(),
    )
        .unwrap();
    /// 这个路径是 `env_config.toml` 所在的目录。该配置文件如下：
    /// ``` toml
    /// jar_paths = [
    ///     "/path/to/jvm_side.jar",
    /// ]
    /// java_opts = []
    /// ```
    let config_dir = Path::new(".");
    let bot = BotBuilder::new(config_dir)
        .id(bot_info.bot_id)
        .password(bot_info.bot_passwd.clone())
        .file_based_device_info(None)
        .fix_protocol_version_fetch(MiraiProtocol::A, "latest".to_string())
        .build();
    bot.login();
    let event_channel = bot.get_event_channel();
    let on_group_message_event: Box<dyn Fn(GroupMessageEvent)> = Box::new(|group_message_event| {
        let msg_chain = group_message_event.get_message();
        // into_iter 会拿走所有权，之后我会实现一个 as_iter.
        for msg in msg_chain.into_iter() {
            println!("群组消息");
            match_single_message(msg, Some(group_message_event.get_subject()));
        }
    });
    let on_friend_message_event: Box<dyn Fn(FriendMessageEvent)> =
        Box::new(|friend_message_event| {
            let msg_chain = friend_message_event.get_message();
            // into_iter 会拿走所有权，之后我会实现一个 as_iter.
            for msg in msg_chain.into_iter() {
                println!("好友消息");
                match_single_message(msg, None);
            }
        });
    // 监听 GroupMessageEvent.
    let listener_for_group_message_event = event_channel.subscribe_always(&on_group_message_event);
    // 监听 FriendMessageEvent.
    let listener_for_friend_message_event =
        event_channel.subscribe_always(&on_friend_message_event);
    // 因为监听并不阻塞线程，不阻塞的话程序会直接结束。这里仅供参考。
    let current_thread = std::thread::current();
    ctrlc::set_handler(move || current_thread.unpark()).unwrap();
    std::thread::park();
    // 取消监听。
    listener_for_group_message_event.complete();
    listener_for_friend_message_event.complete();
    println!("Hello, world!");
}
```