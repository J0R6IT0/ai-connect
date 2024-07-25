use managers::{chat::ChatManager, tts::TTSManager};
use std::{fs, time::Duration};
use tokio::{
    process::Command,
    time::{sleep, Instant},
};

mod api;
mod managers;

#[tokio::main]
async fn main() {
    // Kill LocalAI process if its running
    println!("Killing LocalAI process");
    let output = Command::new("pkill")
        .arg("local-ai")
        .output()
        .await
        .expect("Failed to kill LocalAI");

    println!("pkill {}", output.status);

    let home_path = dirs::home_dir().unwrap();
    let localai_path = home_path.join("local-ai");

    let exe_path = std::env::current_exe().expect("Failed to get current exe path");
    let temp_dir = exe_path.parent().unwrap().join("temp");
    fs::create_dir_all(&temp_dir).expect("Failed to create temp folder");
    let temp_path = temp_dir;
    let audio_path = temp_path.join("audio.wav");

    // Execute LocalAI from the home directory
    Command::new(localai_path)
        .spawn()
        .expect("Failed to start LocalAI");

    wait_available().await;

    let chat_manager = ChatManager::new();
    let tts_manager = TTSManager::new();

    loop {
        let mut line = String::new();
        println!("Enter your prompt:");
        std::io::stdin().read_line(&mut line).unwrap();

        let start_time = Instant::now();

        let completion = chat_manager.generate_completion(&line).await;
        let audio_file = tts_manager
            .generate_audio(&completion.unwrap().choices[0].message.content)
            .await
            .unwrap();

        fs::write(&audio_path, audio_file).expect("Failed to write audio file");

        // Calculate execution time
        let end_time = Instant::now();
        let execution_time = end_time - start_time;
        println!("Execution time: {:?}", execution_time);
    }
}

// wait for 0.0.0.0:8080 to be available
async fn wait_available() {
    let mut is_available = false;
    while !is_available {
        let output = Command::new("nc")
            .arg("-z")
            .arg("0.0.0.0")
            .arg("8080")
            .output()
            .await
            .expect("Failed to check availability");

        is_available = output.status.success();

        sleep(Duration::from_secs(1)).await;
    }
}
