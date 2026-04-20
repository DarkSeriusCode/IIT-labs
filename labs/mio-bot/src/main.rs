use log::LevelFilter;

use teloxide::{
    prelude::*,
};

mod openrouter;

#[tokio::main]
async fn main() {
    colog::basic_builder()
        .filter(None, LevelFilter::Off)
        .filter(Some(env!("CARGO_CRATE_NAME")), LevelFilter::Trace)
        .init();

    log::info!("Starting bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(text) = msg.text() {
            match openrouter::ask_ai(text).await {
                Ok(reply) => {
                    log::info!("@{}: {}", msg.from().unwrap().clone().username.unwrap(), text);
                    bot.send_message(msg.chat.id, reply).await?;
                }
                Err(err) => {
                    log::error!("On {}: {}", text, err);
                    bot.send_message(msg.chat.id, "Что-то пошло не так").await?;
                }
            }
        } else {
            bot.send_message(msg.chat.id, "Я понимаю только текст ;-;").await?;
        }
        Ok(())
    }).await;
}
