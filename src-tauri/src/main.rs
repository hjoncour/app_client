// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::{BufReader, BufRead};
use serde::{Serialize, Deserialize};

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

    let mut writer: BufWriter<File> = BufWriter::new(file);
    write!(writer, "{},{}\n", email, password)
        .map_err(|_| "Failed to write to file")?;

    Ok("Signup successful".to_string())
}

#[derive(Serialize, Deserialize)]
struct LoginResponse {
    success: bool,
    message: String,
}

#[tauri::command]
fn login(email: String, password: String) -> Result<String, String> {
    let file = File::open("user_credentials.csv").map_err(|_| serde_json::to_string(&LoginResponse {
        success: false,
        message: "Failed to open file".to_string(),
    }).unwrap())?;
    
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.map_err(|_| serde_json::to_string(&LoginResponse {
            success: false,
            message: "Failed to read line".to_string(),
        }).unwrap())?;
        
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 2 {
            continue; // skip lines that don't have exactly two parts
        }
        if parts[0] == email && parts[1].trim_end() == password {
            return Ok(serde_json::to_string(&LoginResponse {
                success: true,
                message: "Login successful".to_string(),
            }).unwrap());
        }
    }
    
    Ok(serde_json::to_string(&LoginResponse {
        success: false,
        message: "Invalid email or password".to_string(),
    }).unwrap())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, signup, login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
