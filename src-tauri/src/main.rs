// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::OpenOptions;
use std::io::{Write, BufWriter};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn signup(email: String, password: String) -> Result<String, String> {
    use std::fs::OpenOptions;
    use std::io::{Write, BufWriter};

    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("user_credentials.csv")
        .map_err(|_| "Failed to open or create file")?;

    let mut writer = BufWriter::new(file);
    write!(writer, "{},{}\n", email, password)
        .map_err(|_| "Failed to write to file")?;

    Ok("Signup successful".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, signup])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
