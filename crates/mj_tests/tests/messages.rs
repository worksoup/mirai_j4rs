use mirai_j4rs::{
    contact::{
        contact_trait::{ContactOrBotTrait, FileSupportedTrait, SendMessageSupportedTrait},
        group::Group,
    },
    file::AbsoluteFileFolderTrait,
    message::{
        data::{
            at::At, at_all::AtAll, face::Face, forward_message::ForwardMessageBuilder,
            plain_text::PlainText,
        },
        message_trait::{CodableMessageTrait, MessageHashCodeTrait, MessageTrait},
    },
};
use mj_tests::{get_group_id, get_member_id, get_test_bot};

#[test]
fn at() {
    let bot = get_test_bot(); // 这一行的背后定义了 `Env`, 所以一切操作都需要放在这之后。
    bot.login();
    // `At`
    let at = At::new(get_member_id());
    let group = Group::new(&bot, get_group_id()).unwrap();
    println!("{}", at.to_display_string(&group));
    println!("{}", at.to_code());
    println!("{}", at.to_content());
    println!("{}", at.hash_code());
    println!("{}", at.to_string());
    let r = group.send_message(at);
    r.recall();
    bot.close();
}

#[test]
fn at_all() {
    let bot = get_test_bot(); // 这一行的背后定义了 `Env`, 所以一切操作都需要放在这之后。
    bot.login();
    // `AtAll`
    let at_all = AtAll::new();
    let group = Group::new(&bot, get_group_id()).unwrap();
    println!("{}", AtAll::get_display());
    println!("{}", at_all.to_code());
    println!("{}", at_all.to_content());
    println!("{}", at_all.hash_code());
    println!("{}", at_all.to_string());
    let r = group.send_message(at_all);
    r.recall();
    bot.close();
}

#[test]
fn face() {
    let bot = get_test_bot(); // 这一行的背后定义了 `Env`, 所以一切操作都需要放在这之后。
    bot.login();
    // `Face`
    let face = Face::from(123);
    let group = Group::new(&bot, get_group_id()).unwrap();
    println!("{}", face.get_name());
    println!("{}", face.to_code());
    println!("{}", face.to_content());
    println!("{}", face.hash_code());
    println!("{}", face.to_string());
    let r = group.send_message(face);
    r.recall();
    bot.close();
}
#[test]
fn file_message() {
    let bot = get_test_bot(); // 这一行的背后定义了 `Env`, 所以一切操作都需要放在这之后。
    bot.login();
    // `FileMessage`
    let group = Group::new(&bot, get_group_id()).unwrap();
    let remote_files = group.get_files();
    let group = remote_files.get_contact();
    assert_eq!(group.get_id(), get_group_id());
    let root = remote_files.get_root();
    println!("根目录名：{}", root.get_name());
    let root_children = root.children().to_vec();
    for file in root_children {
        println!(
            "文件/目录名：{}，上传时间：{}，修改时间：{}，文件-目录：{}-{}，上传者：{}",
            file.get_name(),
            file.get_upload_time(),
            file.get_last_modified_time(),
            file.is_file(),
            file.is_folder(),
            file.get_uploader_id()
        );
        if file.is_file() {
            let file = file.to_file();
            let name = file.get_name();
            if name == "1.docx" {
                if file.delete() {
                    println!("已删除。");
                }
            } else {
                println!("url: {}", file.get_url());
                println!("md5: {:?}", file.get_md5());
                println!("sha1: {:?}", file.get_sha1());
                println!("size: {:?}", file.get_size());
            }
        }
    }
    let _ = root.upload_new_file("aaa.toml", "./base_config.toml");
    bot.close();
}

#[test]
fn forward_message() {
    let bot = get_test_bot(); // 这一行的背后定义了 `Env`, 所以一切操作都需要放在这之后。
    bot.login();
    // `ForwardMessage`
    let group = Group::new(&bot, get_group_id()).unwrap();
    let message = ForwardMessageBuilder::new(&group)
        .add(&bot, PlainText::from("asdasdasd"), 1706798170)
        .add_(3141592654_i64, "(｢・ω・)｢", AtAll::new(), 1706798166)
        .build();
    let _r = group.send_message(message);
    bot.close();
}
