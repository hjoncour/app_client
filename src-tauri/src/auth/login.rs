use std::fs::File;
use std::io::{BufReader, BufRead};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct LoginResponse {
    success: bool,
    message: String,
}

#[tauri::command]
pub fn login(email: String, password: String) -> Result<String, String> {

     /* To be replaced with a call to the API */
    let file: File = File::open("user_credentials.csv").map_err(|_| serde_json::to_string(&LoginResponse {
        success: false,
        message: "Failed to open file".to_string(),
    }).unwrap())?;
    let reader: BufReader<File> = BufReader::new(file);
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
    /*****************************************/
    Ok(serde_json::to_string(&LoginResponse {
        success: false,
        message: "Invalid email or password".to_string(),
    }).unwrap())
}
