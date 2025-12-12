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

