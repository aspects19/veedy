
pub mod inlinekeyboard;
pub mod inlinequery;
pub mod youtubeurl;
// pub mod ytdl;

use dotenvy::{dotenv, var};
use teloxide::{
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText
    },
};

use log::info;

use crate::youtubeurl::yt_url;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    info!("Starting inline bot...");

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

async fn handle_inline_query(
    bot: Bot,
    query: InlineQuery,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Received inline query: {}", query.query);

    // Handle empty queries

    if query.query.is_empty() {
        info!("Empty query received, returning no results");

        bot.answer_inline_query(query.id, vec![]).await?;

        return Ok(());
    }

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("4K", "4K"),
            InlineKeyboardButton::callback("1080P", "1080P"),
            InlineKeyboardButton::callback("720P", "720P"),
        ],
        vec![
            InlineKeyboardButton::callback("480P", "480P"),
            InlineKeyboardButton::callback("360P", "360P"),
            InlineKeyboardButton::callback("256K", "256K"),
            InlineKeyboardButton::callback("128K", "128K"),
        ]
    ]);

    let mut results: Vec<InlineQueryResult> = vec![];

    let query_results = yt_url(query.query.clone().as_str(), var("YOUTUBE_API_KEY")?, 5).await?;

    for result in query_results {
        results.push(
            InlineQueryResult::Article(InlineQueryResultArticle {
                
                id: result.id,
                title: result.title,
                input_message_content: InputMessageContent::Text(InputMessageContentText {
                    message_text: result.video_url.to_string(),
                    parse_mode: None,
                    link_preview_options: None,
                    entities: None,
                }),
                description: Some(result.description),
                url: Some(result.video_url),
                hide_url: Some(true),
                reply_markup: Some(keyboard.clone()),
                thumbnail_url: Some(result.thumbnail_url),
                thumbnail_width: Some(120),
                thumbnail_height: Some(90),
            })
        )
    }

    info!(
        "Sending {} results for query: {}",
        results.len(),
        query.query
    );

    bot.answer_inline_query(query.id, results)
        .cache_time(300)
        .await?;

    Ok(())
}
