use std::process::Command;
use std::path::Path;
use std::os::unix::fs::PermissionsExt; //TODO: Solve for Windows
use tokio::fs;


pub async fn prepare_ytdlp() -> Result<(), Box<dyn std::error::Error>> {
    if Command::new("yt-dlp").arg("--version").output().is_ok() {
        println!("System yt-dlp Found. Skipping download.");
        return Ok(());
    }

    if Path::new("lib/yt-dlp").exists() {
        println!("Local yt-dlp found in lib/.");
        return Ok(());
    }

    println!("yt-dlp missing. Downloading standalone binary...");
    download_ytdlp().await?;
    Ok(())
}

async fn download_ytdlp() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("lib").await?; 

    // TODO: Dynamify this for different environments/OS (Windows?)
    let url = "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux";
    let dest_path = "lib/yt-dlp";

    let resp = reqwest::get(url).await?;
    fs::write(dest_path, resp.bytes().await?).await?;

    // TODO: handle other Environments
    let mut perms = fs::metadata(dest_path).await?.permissions();
    perms.set_mode(0o755); // rwxr-xr-x
    fs::set_permissions(dest_path, perms).await?;

    println!("yt-dlp is ready");

    Ok(())
}