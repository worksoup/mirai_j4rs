#[cfg(test)]
mod tests {
    use mirai_j4rs::{
        contact::{
            AudioSupportedTrait, ContactOrBotTrait, FileSupportedTrait, Group,
            SendMessageSupportedTrait,
        },
        message::{
            data::{
                At, AtAll, Dice, Face, ForwardMessageBuilder, Image, PlainText, PokeMessage,
                PokeMessageEnum, RockPaperScissors,
            },
            AudioTrait, CodableMessageTrait, MessageHashCodeTrait, MessageTrait,
        },
        utils::{
            backend::Overflow,
            contact::file::{AbsoluteFileFolderTrait, ExternalResource},
            just_for_examples::bot_group_member_overflow,
        },
    };

    static WORKING_DIR: &str = "../../working_dir";
    static IMAGE_PATH: &str = "../../working_dir/mirai.png";
    static AUDIO_PATH: &str = "../../working_dir/test.mp3";
    static FILE_PATH: &str = AUDIO_PATH;
    #[test]
    fn test_all() {
        // 这一行的背后定义了 `Env`, 所以一切操作都需要放在这之后。
        let (bot, group_id, member_id) = bot_group_member_overflow(WORKING_DIR, 1106);
        let group = Group::new(&bot, group_id).unwrap();
        let friend = bot.get_friend(member_id).expect("Bot 没有该好友。");
        // bot.login(); // Overflow 后端无登录逻辑。
        // `At`
        let at = At::new(member_id);
        println!("{}", at.to_display_string(&group));
        println!("{}", at.to_code());
        println!("{}", at.to_content());
        println!("{}", at.hash_code());
        println!("{}", at.to_string());
        let r = group.send_message(&at);
        r.recall();
        // `AtAll`
        let at_all = AtAll::new();
        println!("{}", <AtAll<Overflow>>::get_display());
        println!("{}", at_all.to_code());
        println!("{}", at_all.to_content());
        println!("{}", at_all.hash_code());
        println!("{}", at_all.to_string());
        let r = group.send_message(&at_all);
        r.recall();
        // `Audio`
        // 支持 `*.amr` 或 `*.silk` 格式。如果使用了 `mirai-silk-converter` 的话也可以支持 `*.mp3` 格式。
        let resource = ExternalResource::create_from_file(AUDIO_PATH);
        let audio = group.upload_audio(&resource);
        println!("{}", audio.get_extra_data());
        println!("{}", audio.get_file_md5());
        println!("{}", audio.get_file_name());
        println!("{}", audio.get_file_size());
        println!("{}", audio.to_content());
        println!("{}", audio.to_string());
        let _r = group.send_message(&audio);
        resource.close();
        // `Face`
        let face = Face::from(123);
        println!("{}", face.get_name());
        println!("{}", face.to_code());
        println!("{}", face.to_content());
        println!("{}", face.hash_code());
        println!("{}", face.to_string());
        let r = group.send_message(&face);
        r.recall();
        // `FileMessage`
        let remote_files = group.get_files();
        let group_ = remote_files.get_contact();
        assert_eq!(group_.get_id(), group_id);
        let root = remote_files.get_root();
        println!("根目录名：{}", root.get_name());
        // not impl yet.
        // let root_children = root.children().to_vec();
        // for file in root_children {
        //     println!(
        //         "文件/目录名：{}，上传时间：{}，修改时间：{}，文件-目录：{}-{}，上传者：{}",
        //         file.get_name(),
        //         file.get_upload_time(),
        //         file.get_last_modified_time(),
        //         file.is_file(),
        //         file.is_folder(),
        //         file.get_uploader_id()
        //     );
        //     if file.is_file() {
        //         let file = file.to_file();
        //         let name = file.get_name();
        //         if name == "mirai_j4rs_test.mp3" {
        //             if file.delete() {
        //                 println!("已删除。");
        //             }
        //         } else {
        //             println!("url: {}", file.get_url());
        //             println!("md5: {:?}", file.get_md5());
        //             println!("sha1: {:?}", file.get_sha1());
        //             println!("size: {:?}", file.get_size());
        //         }
        //     }
        // }
        // not impl yet.
        // let res = ExternalResource::create_from_file(FILE_PATH);
        // let _ = root.upload_new_file("mirai_j4rs_test.mp3", &res);
        // res.close();
        // `ForwardMessage`
        let message = ForwardMessageBuilder::new(&group)
            .add(
                &bot,
                &PlainText::from("这条消息的时间戳是1706798170"),
                1706798170,
            )
            .add_(3141592654_i64, "(｢・ω・)｢", &AtAll::new(), 1706798166)
            .build();
        let _r = group.send_message(&message);
        // `Image`
        let image = group.upload_image_from_file(IMAGE_PATH);
        println!("{}", image.get_image_id());
        // ERROR on Overflow backend!
        // println!("{}", image.get_md5());
        println!("{}", image.get_size());
        println!("{}", image.to_code());
        println!("{}", image.to_content());
        println!("{}", image.to_string());
        println!("{}", image.is_emoji());
        println!("{}", image.query_url());
        println!("{}", image.get_width());
        println!("{}", image.get_height());
        let image = Image::from_id(image.get_image_id());
        let _r = group.send_message(&image);
        // `Dice`
        // 目前新版客户端可以接受该类型消息，但是不会显示点数。
        // 可以直接指定。
        // let dice = Dice::new(2);
        // 随机点数。
        let dice = Dice::random();
        let _r = group.send_message(&dice);
        // `RockPaperScissors`
        // 目前新版客户端可以接受该类型消息，但是不会显示结果。
        // 可以直接指定。
        // let rps = RockPaperScissors::paper();
        // 随机结果。
        let rps = RockPaperScissors::random();
        let _r = group.send_message(&rps);
        // `MarketFace` 其他市场表情。
        // 不支持直接构造和发送。可以转发。
        // 只有安卓手机协议和苹果平板协议支持。其余协议会报错。
        // 用法如下：
        // `BotNudge`
        // let bot_nudge = bot.nudge();
        // let friend = member_id;
        // let friend = bot.get_friend(friend).expect("Bot 没有该好友。");
        // bot_nudge.send_to(friend);
        // `PokeMessage`
        // 在群里可以发 SVIP 的戳一戳。
        // 官方客户端似乎不能在群里发该类型消息。
        let poke_message: PokeMessage<Overflow> = PokeMessageEnum::召唤术.into();
        let r = group.send_message(&poke_message);
        r.recall();
        // 但是只能给好友发普通戳一戳。
        let poke_message: PokeMessage<Overflow> = PokeMessageEnum::六六六.into();
        let r = friend.send_message(&poke_message);
        r.recall();
        // `PlainText`
        let plain_text = PlainText::from("你好！");
        let _r = group.send_message(&plain_text);
        let _r = group.send_string("Hello!");
        // `PlainText`
        let plain_text = PlainText::from("你好！");
        let _r = group.send_message(&plain_text);
        let _r = group.send_string("Hello!");
    }
}
