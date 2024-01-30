use std::fs::File;

#[tauri::command]
pub fn signup(email: String, password: String) -> Result<String, String> {
    use std::fs::OpenOptions;
    use std::io::{Write, BufWriter};

    /* To be replaced with a call to the API */
    let file: File = OpenOptions::new().write(true)
        .append(true)
        .create(true)
        .open("user_credentials.csv")
        .map_err(|_| "Failed to open or create file")?;

    let mut writer: BufWriter<File> = BufWriter::new(file);
    write!(writer, "{},{}\n", email, password)
        .map_err(|_| "Failed to write to file")?;
    /*****************************************/

    Ok("Signup successful".to_string())
}