// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lib;

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![
            lib::get_bot_status,
            lib::send_message,
            lib::get_guilds,
            lib::set_bot_presence
        ])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}