// src-tauri/src/lib.rs
use reqwest;
use serde::{Deserialize, Serialize};
use tauri::command;

const API_BASE_URL: &str = "http://localhost:3001/api";

#[derive(Serialize, Deserialize)]
pub struct BotStatus {
    status: String,
    bot_ready: bool,
    guilds: u32,
    users: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SendMessageRequest {
    channel_id: String,
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct SendMessageResponse {
    success: bool,
    message_id: Option<String>,
    timestamp: Option<String>,
    error: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Guild {
    id: String,
    name: String,
    member_count: u32,
    channels: Vec<Channel>,
}

#[derive(Serialize, Deserialize)]
pub struct Channel {
    id: String,
    name: String,
    r#type: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SetPresenceRequest {
    status: Option<String>,
    activity: Option<Activity>,
}

#[derive(Serialize, Deserialize)]
pub struct Activity {
    name: String,
    r#type: Option<u32>,
}

#[command]
pub async fn get_bot_status() -> Result<BotStatus, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/status", API_BASE_URL))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.status().is_success() {
        let status: BotStatus = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(status)
    } else {
        Err(format!("API returned error: {}", response.status()))
    }
}

#[command]
pub async fn send_message(channel_id: String, message: String) -> Result<SendMessageResponse, String> {
    let client = reqwest::Client::new();
    let request_body = SendMessageRequest { channel_id, message };

    let response = client
        .post(&format!("{}/send-message", API_BASE_URL))
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let result: SendMessageResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(result)
}

#[command]
pub async fn get_guilds() -> Result<Vec<Guild>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/guilds", API_BASE_URL))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.status().is_success() {
        let guilds: Vec<Guild> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(guilds)
    } else {
        Err(format!("API returned error: {}", response.status()))
    }
}

#[command]
pub async fn set_bot_presence(status: Option<String>, activity_name: Option<String>) -> Result<String, String> {
    let client = reqwest::Client::new();
    let activity = activity_name.map(|name| Activity {
        name,
        r#type: Some(0), // Playing
    });

    let request_body = SetPresenceRequest { status, activity };

    let response = client
        .post(&format!("{}/set-presence", API_BASE_URL))
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.status().is_success() {
        Ok("Presence updated successfully".to_string())
    } else {
        Err(format!("Failed to update presence: {}", response.status()))
    }
}