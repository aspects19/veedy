use std::{path::Path, process::Command};

use tokio::fs;

pub async fn prepare_ffmpeg() -> Result<(), Box<dyn std::error::Error>> {
    if Command::new("ffmpeg").arg("-version").output().is_ok() {
        println!("System FFmpeg Found. Skipping download.");
        return Ok(());
    }

    if Path::new("lib/ffmpeg").exists() {
        println!("Local FFmpeg found.");
        return Ok(());
    }

    println!("FFmpeg missing downloading...");
    download_ffmpeg().await?;
    Ok(())
}

async fn download_ffmpeg() -> Result<(), Box<dyn std::error::Error>>{
    fs::create_dir_all("lib").await?;

    // TODO: Dynamify this for different environments/OS
    let url = "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-linux64-gpl.tar.xz";
    let archive_path = "lib/ffmpeg.tar.xz";

    let resp = reqwest::get(url).await?;
    fs::write(archive_path, resp.bytes().await?).await?;

    let status = Command::new("tar")
        .args(&[
            "-xf", archive_path,
            "-C", "lib",
            "--strip-components=2", 
            "--wildcards", "*/bin/ffmpeg"
        ])
        .status()
        .expect("Failed to run tar");

    if status.success() {
        std::fs::remove_file(archive_path).ok();
        println!("Done! FFmpeg is ready at ./lib/ffmpeg");
    } else {
        eprintln!("Extraction failed.");
    }

    Ok(())

}