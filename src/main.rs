
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


    let bot = Bot::from_env(); 

    let handler = Update::filter_message()
        .branch(
            Update::filter_inline_query()
            .endpoint(handle_inline_query)
        );

    Dispatcher::builder(
        bot,
        handler,
    )
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}