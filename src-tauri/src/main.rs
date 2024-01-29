// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
use std::fs::File;
use std::io::{BufReader, BufRead};
use serde::{Serialize, Deserialize};

use auth::login::login;
use auth::signup::signup;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, signup, login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
