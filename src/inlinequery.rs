use dotenvy::var;
use log::{error, info};
use teloxide::{
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQuery, InlineQueryResult, InlineQueryResultArticle, InputFile, InputMedia, InputMediaVideo, InputMessageContent, InputMessageContentText, Recipient
    },
};
use url::Url;

use crate::{youtubeurl::yt_url, ytdlp::download_video};

pub async fn handle_inline_query(
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

    let mut results: Vec<InlineQueryResult> = vec![];

    let query_results = yt_url(&query.query, var("YOUTUBE_API_KEY")?, 5).await?;

    for result in query_results {
        results.push(InlineQueryResult::Article(InlineQueryResultArticle {
            id: result.id,
            title: result.title,
            input_message_content: InputMessageContent::Text(InputMessageContentText {
                message_text: "Preparing".to_string(),
                parse_mode: None,
                link_preview_options: None,
                entities: None,
            }),
            description: Some(result.description),
            url: Some(result.video_url),
            hide_url: Some(true),
            reply_markup: Some(InlineKeyboardMarkup::new(vec![vec![
                InlineKeyboardButton::callback("⏳ Pending...", "ignore_this"),
            ]])),
            thumbnail_url: Some(result.thumbnail_url),
            thumbnail_width: Some(120),
            thumbnail_height: Some(90),
        }))
    }

    bot.answer_inline_query(query.id, results)
        .cache_time(300)
        .await?;

    Ok(())
}

pub async fn handle_chosen_inline_result(
    bot: Bot,
    chosen: ChosenInlineResult,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let video_id = chosen.result_id;
    let video_url_str = format!("https://www.youtube.com/watch?v={video_id}");
    let video_url = Url::parse(&video_url_str)?;

    let inline_id = match chosen.inline_message_id {
        Some(id) => id,
        None => return Ok(()),
    };


    bot.edit_message_text_inline(&inline_id, "⏳ Downloading… Please wait")
        .await?;

    let file_path = match download_video(video_url).await {
        Ok(path) => path,
        Err(e) => {
            error!("Download failed: {}", e);
            bot.edit_message_text_inline(&inline_id, format!("❌ Error: Failed to download video.\nReason: {}", e)).await?;
            return Ok(());
        }
    };

    let channel_id_str = var("STORE_CHANNEL_ID")?; 
    let channel_id = channel_id_str.parse::<i64>()?;
    
    let channel_msg = bot.send_video(Recipient::Id(ChatId(channel_id)), InputFile::file(&file_path))
        .await?;

    let video_file_id = match channel_msg.video() {
        Some(v) => v.file.id.clone(),
        None => {
            bot.edit_message_text_inline(&inline_id, "❌ Error: Could not retrieve video ID from channel.").await?;
            return Ok(());
        }
    };

    bot.edit_message_media_inline(
        &inline_id, 
        InputMedia::Video(
            InputMediaVideo::new(InputFile::file_id(video_file_id))
                .supports_streaming(true)
        )
    ).await?;

    
    if let Err(e) = tokio::fs::remove_file(&file_path).await {
        error!("Failed to delete temp file {:?}: {}", file_path, e);
    } else {
        info!("Cleaned up file: {:?}", file_path);
    }

    Ok(())
}
