use tokio::process::Command;
use std::path::{Path, PathBuf};
use log::info;
use tokio::fs;
use url::Url;

use crate::ffmpeg::prepare_ffmpeg;


pub async fn prepare_ytdlp() -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    if Command::new("yt-dlp").arg("--version").output().await.is_ok() {
        info!("Using system yt-dlp.");
        return Ok(PathBuf::from("yt-dlp"))
    }

    let local_path = Path::new("lib").join("yt-dlp");
    if local_path.exists() {
        info!("Using local yt-dlp at {:?}", local_path);

        let _ = tokio::process::Command::new(&local_path)
            .arg("-U")
            .output()
            .await;

        return Ok(tokio::fs::canonicalize(local_path).await?);
    }

    println!("yt-dlp missing. Downloading standalone binary...");
    download_ytdlp().await?;

    Ok(tokio::fs::canonicalize(&local_path).await?)
}

async fn download_ytdlp() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    fs::create_dir_all("lib").await?; 

    // Linux 64-bit (Static Build)
    let url = "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux";
    let dest_path = Path::new("lib").join("yt-dlp");

    let resp = reqwest::get(url).await?;
    fs::write(&dest_path, resp.bytes().await?).await?;

  // Make executable (Linux/Mac specific)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&dest_path).await?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&dest_path, perms).await?;
    }

    Ok(())
}

pub async fn download_video(link: Url) ->Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {

    let ytdlp_path = prepare_ytdlp().await?;
    let ffmpeg_path = prepare_ffmpeg().await?;

    let download_dir = Path::new("downloads");
    
    if !download_dir.exists() {
        fs::create_dir_all(download_dir).await?;
    }

    let abs_download_dir = std::fs::canonicalize(download_dir)?;

    info!("Starting download for: {}", link);

    let mut  cmd = Command::new(&ytdlp_path);

    cmd.arg(link.as_str())
        .arg("-P").arg(abs_download_dir)
        .arg("-o") .arg("%(title)s.%(ext)s") 
        .arg("--print") .arg("filename")
        .arg("--no-simulate");

    if let Some(ffmpeg_str) = ffmpeg_path.to_str() {
        cmd.arg("--ffmpeg-location").arg(ffmpeg_str);
    }

    let output = cmd.output().await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("yt-dlp failed: {}", stderr).into());
    }

    let path_string = String::from_utf8(output.stdout)?;
    let final_path = PathBuf::from(path_string.trim());

    info!("Download completed: {:?}", final_path);

    Ok(final_path)
}