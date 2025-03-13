use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::env;
use dirs;

#[derive(Serialize, Deserialize)]
struct Config {
    api_url: String,
}

fn get_config_path() -> PathBuf {
    let base_dir = dirs::config_dir().unwrap_or_else(|| env::current_dir().unwrap());
    base_dir.join("tagum-marketplace").join("config.json")
}

#[tauri::command]
async fn get_config() -> Result<String, String> {
    let path = get_config_path();
    match fs::read_to_string(&path) {
        Ok(contents) => Ok(contents),
        Err(_) => {
            // Default config if file is missing
            let default_config = Config {
                api_url: "http://localhost:3000".to_string(),
            };
            let default_json = serde_json::to_string(&default_config).unwrap();
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, &default_json).unwrap();
            Ok(default_json)
        }
    }
}
#[tauri::command]
async fn set_config(new_config: String) -> Result<(), String> {
    let path = get_config_path();
    fs::write(path, new_config).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_config, set_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
