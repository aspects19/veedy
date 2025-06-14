use teloxide::{prelude::*,  utils::command::BotCommands};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;

}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Settings,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Settings => {
            bot.send_message(msg.chat.id, format!("Choose a video quality to use")).await?
        }
    };

    Ok(())
}