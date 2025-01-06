use std::io;
use std::process::Command;

fn main() {
    if !is_yt_dlp_installed() {
        println!("yt-dlp is not installed. Please install it first.");
        return;
    }

    println!("Enter YouTube URL:");
    let mut url = String::new();
    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read line");

    match convert_youtube_to_audio(&url.trim()) {
        Ok(_) => println!("Conversion completed successfully!"),
        Err(e) => println!("Error during conversion: {}", e),
    }
}

fn convert_youtube_to_audio(url: &str) -> Result<(), String> {
    let output = Command::new("yt-dlp")
        .args([
            "-x",
            "--audio-format", "mp3",
            url,
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn is_yt_dlp_installed() -> bool {
    Command::new("yt-dlp")
        .arg("--version")
        .output()
        .is_ok()
}