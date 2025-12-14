pub mod ffmpeg;
pub mod inlinekeyboard;
pub mod inlinequery;
pub mod youtubeurl;
pub mod ytdlp;

use crate::inlinequery::{handle_chosen_inline_result, handle_inline_query};
use dotenvy::dotenv;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_inline_query().endpoint(handle_inline_query))
        .branch(Update::filter_chosen_inline_result().endpoint(handle_chosen_inline_result));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
