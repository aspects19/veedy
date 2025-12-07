use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub description: String,
    pub thumbnail_url: Url,
    pub video_url: Url
}

#[derive(Deserialize, Debug)]
struct SearchListResponse {
    #[serde(default)]
    items: Vec<SearchResult>
}

#[derive(Deserialize, Debug)]
struct SearchResult {
    id: Id,
    snippet: Snippet,
}

#[derive(Deserialize, Debug)]
struct Id {
    #[serde(rename = "videoId")]
    video_id: Option<String>
}

#[derive(Deserialize, Debug)]
struct Snippet {
    title: String,
    description: String,
    thumbnails: Thumbnails,
}

#[derive(Deserialize, Debug)]
struct Thumbnails {
    default: Option<Thumbnail>,
    medium: Option<Thumbnail>,
    high: Option<Thumbnail>,
}

#[derive(Deserialize, Debug)]
struct Thumbnail {
    url: String,
}

#[derive(Deserialize, Debug)]
struct ApiErrorResponse {
    error: ApiError,
}

#[derive(Deserialize, Debug)]
struct ApiError {
    code: u32,
    message: String,
}

fn shorten_title(title: &str) -> String {
    let cleaned = title.replace('\u{200b}', " ");
    let words: Vec<String> = cleaned
        .split_whitespace()
        .filter(|w| !w.starts_with('#'))
        .map(|s| s.to_string())
        .collect();
    let mut short = words.join(" ");
    if short.is_empty() {
        short = title.to_string();
    }
    if short.len() > 100 {
        short = short.chars().take(100).collect::<String>() + "...";
    }
    short.trim().to_string()
}

pub async fn yt_url(
    query: &str,
    api_key: String,
    max_results: i32,
) -> Result<Vec<Video>, Box<dyn Error + Send + Sync>> {

    if query.is_empty() { return Ok(Vec::<Video>::new()) };

    let url = format!(
        "https://www.googleapis.com/youtube/v3/search?part=snippet&q={}&type=video&key={}&maxResult={}",
        query, api_key, max_results
    );

    let body: Value = reqwest::get(&url).await?.json().await?;

    // Check for error response
    if body.get("error").is_some() {
        let api_err: ApiErrorResponse = serde_json::from_value(body)?;
        return Err(format!(
            "YouTube API error: code {} - {}",
            api_err.error.code, api_err.error.message
        )
        .into());
    }

    

    let resp: SearchListResponse = serde_json::from_value(body)?;

    let mut videos = Vec::new();

    for item in resp.items {
        let Some(video_id) = item.id.video_id else {
            continue; 
        };

        let shortened_title = shorten_title(&item.snippet.title);

        let description = item.snippet.description;

        let Some(thumb) = item.snippet.thumbnails.high
            .or(item.snippet.thumbnails.medium)
            .or(item.snippet.thumbnails.default) else {
            continue; 
        };
        
        let video_url = Url::parse(&format!("https://www.youtube.com/watch?v={}", video_id))?;

        let thumbnail_url = Url::parse(&thumb.url)?;

        videos.push(Video {
            id: video_id,
            title: shortened_title,
            description,
            thumbnail_url,
            video_url
        });
    }

    Ok(videos)
}