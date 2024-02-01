use mj_tests::get_test_bot;

#[test]
fn template() {
    let bot = get_test_bot(); // 这一行的背后定义了 `Env`, 所以一切操作都需要放在这之后。
    bot.login();
    //
    bot.close();
}
