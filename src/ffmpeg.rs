use log::info;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::process::Command;

pub async fn prepare_ffmpeg() -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    if Command::new("ffmpeg")
        .arg("-version")
        .output()
        .await
        .is_ok()
    {
        info!("System FFmpeg found.");
        return Ok(PathBuf::from("ffmpeg"));
    }

    let local_path = Path::new("lib").join("ffmpeg");

    if local_path.exists() {
        info!("Using local FFmpeg at {:?}", local_path);
        return Ok(tokio::fs::canonicalize(&local_path).await?);
    }

    info!("FFmpeg missing. Downloading standalone binary...");
    download_ffmpeg().await?;

    return Ok(tokio::fs::canonicalize(&local_path).await?);
}

async fn download_ffmpeg() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    fs::create_dir_all("lib").await?;
    let archive_path = Path::new("lib").join("ffmpeg.tar.xz");

    // Linux 64-bit Static Build (GPL)
    let url = "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-linux64-gpl.tar.xz";

    info!("Downloading FFmpeg archive...");
    let resp = reqwest::get(url).await?;
    fs::write(&archive_path, resp.bytes().await?).await?;

    info!("Extracting FFmpeg...");
    let status = Command::new("tar")
        .args([
            "-xf",
            archive_path.to_str().unwrap(),
            "-C",
            "lib",
            "--strip-components=2",
            "--wildcards",
            "*/bin/ffmpeg",
        ])
        .status()
        .await?;

    if !status.success() {
        return Err("Tar extraction failed.".into());
    }

    let _ = fs::remove_file(archive_path).await;

    Ok(())
}
