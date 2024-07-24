use mirai_j4rs::{
    contact::{Group, SendMessageSupportedTrait},
    event::{FriendMessageEvent, GroupMessageEvent, MessageEventTrait},
    message::{
        data::{Audio, MarketFaceAll, RockPaperScissors, SingleMessage},
        MarketFaceTrait, MessageTrait,
    },
    utils::{contact::file::AbsoluteFileFolderTrait, just_for_examples::bot_group_member},
};

/// 该函数接受一个 SingleMessage, 然后做出对应的反应。
fn match_single_message(msg: SingleMessage, contact: Option<Group>) {
    match msg {
        SingleMessage::At(at) => {
            println!("At {}", at.get_target())
        }
        SingleMessage::AtAll(at_all) => {
            println!("AtAll {}", at_all.to_string())
        }
        SingleMessage::Audio(audio) => match audio {
            Audio::OfflineAudio(_) => panic!("预料之外的错误：收到的语音不应该为离线语音。"),
            Audio::OnlineAudio(audio) => {
                println!("语音：{}", audio.get_url_for_download())
            }
        },
        SingleMessage::Face(face) => {
            println!("表情 {}", face.get_id())
        }
        SingleMessage::FileMessage(file_message) => {
            // 好友的文件消息 Mirai 似乎不支持？
            if let Some(contact) = contact {
                // to_absolute_file 本不应当拿走 contact 的所有权，之后我会修改一下。
                println!(
                    "文件 {}",
                    file_message
                        .to_absolute_file(contact)
                        .unwrap()
                        .get_absolute_path()
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
        SingleMessage::MarketFaceAll(market_face_all) => {
            // 🎲 和剪子包袱锤似乎被下线了，不过还能被 Mirai 接受和发送。
            match market_face_all {
                MarketFaceAll::Dice(dice) => {
                    println!("市场表情：🎲 {}", dice.get_value())
                }
                MarketFaceAll::MarketFace(market_face) => {
                    println!("市场表情：其他市场表情 {}", market_face.get_name());
                }
                MarketFaceAll::RockPaperScissors(rock_paper_scissors) => {
                    let rps = RockPaperScissors::random();
                    if let Some(contact) = contact {
                        let _r = contact.send_message(&rps);
                    }
                    print!("市场表情：剪子包袱锤:");
                    if let Some(win) = rock_paper_scissors.eliminates(RockPaperScissors::random()) {
                        if win {
                            println!("赢麻了。");
                        } else {
                            println!("输惨了。");
                        }
                    } else {
                        println!("平局了。");
                    }
                }
            }
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
        SingleMessage::UnsupportedMessage(_) => {
            println!("不支持的消息！")
        }
        SingleMessage::VipFace(vip_face) => {
            println!("VIP表情 {}", vip_face.to_string())
        }
        SingleMessage::SuperFace(super_face) => {
            println!("超级表情 {}", super_face.to_string())
        }
        SingleMessage::MessageOrigin(_) => {
            //TODO
        }
    }
}

fn main() {
    let (bot, _, _) = bot_group_member("./working_dir"); // 这一行的背后定义了 `Env`, 所以一切操作都需要放在这之后。
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
    let listener_for_group_message_event = event_channel.subscribe_always(on_group_message_event);
    // 监听 FriendMessageEvent.
    let listener_for_friend_message_event =
        event_channel.subscribe_always(on_friend_message_event);
    // 因为监听并不阻塞线程，不阻塞的话程序会直接结束。这里仅供参考。
    let current_thread = std::thread::current();
    ctrlc::set_handler(move || current_thread.unpark()).unwrap();
    std::thread::park();
    // 取消监听。
    listener_for_group_message_event.complete();
    listener_for_friend_message_event.complete();
    bot.close();
}
