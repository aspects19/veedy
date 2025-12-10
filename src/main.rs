
pub mod inlinekeyboard;
pub mod inlinequery;
pub mod youtubeurl;
pub mod ytdlp;
pub mod ffmpeg;

use dotenvy::dotenv;
use teloxide::prelude::*;
use crate::inlinequery::handle_inline_query;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();


    let bot = Bot::from_env(); // Reads TELOXIDE_TOKEN

    Dispatcher::builder(
        bot,
        Update::filter_inline_query().endpoint(handle_inline_query),
    )
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}