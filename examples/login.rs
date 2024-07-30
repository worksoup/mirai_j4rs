use mirai_j4rs::utils::just_for_examples::bot_group_member;
use mirai_j4rs::utils::other::enums::MiraiProtocol;

fn main() {
    let (bot, _, _) = bot_group_member("./working_dir"); // 这一行的背后定义了 `Env`, 所以一切操作都需要放在这之后。
    println!(
        "安卓手表协议是否支持戳一戳{}",
        MiraiProtocol::W.is_nudge_supported()
    );
    println!(
        "安卓手机协议是否支持戳一戳{}",
        MiraiProtocol::A.is_nudge_supported()
    );
    println!(
        "苹果电脑协议是否支持戳一戳{}",
        MiraiProtocol::M.is_nudge_supported()
    );
    println!(
        "苹果平板协议是否支持戳一戳{}",
        MiraiProtocol::I.is_nudge_supported()
    );
    println!(
        "安卓平板协议是否支持戳一戳{}",
        MiraiProtocol::P.is_nudge_supported()
    );
    println!(
        "安卓手表协议是否支持二维码{}",
        MiraiProtocol::W.is_qr_login_supported()
    );
    println!(
        "安卓手机协议是否支持二维码{}",
        MiraiProtocol::A.is_qr_login_supported()
    );
    println!(
        "苹果电脑协议是否支持二维码{}",
        MiraiProtocol::M.is_qr_login_supported()
    );
    println!(
        "苹果平板协议是否支持二维码{}",
        MiraiProtocol::I.is_qr_login_supported()
    );
    println!(
        "安卓平板协议是否支持二维码{}",
        MiraiProtocol::P.is_qr_login_supported()
    );
    bot.login();
    bot.close();
}
