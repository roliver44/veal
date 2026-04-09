use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecentProject {
    pub name: String,
    pub last_modified: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AppConfig {
    pub recent_projects: Vec<RecentProject>,
}

pub fn get_app_data_dir() -> PathBuf {
    // Uses OS native app data directory
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("veal");
    if !path.exists() {
        let _ = fs::create_dir_all(&path);
    }
    path
}

pub fn get_config_path() -> PathBuf {
    let mut path = get_app_data_dir();
    path.push("config.json");
    path
}

pub fn load_app_config() -> AppConfig {
    let path = get_config_path();
    if let Ok(contents) = fs::read_to_string(path) {
        if let Ok(config) = serde_json::from_str(&contents) {
            return config;
        }
    }
    AppConfig::default()
}

pub fn save_app_config(config: &AppConfig) -> Result<(), std::io::Error> {
    let path = get_config_path();
    let json = serde_json::to_string_pretty(config)?;
    fs::write(path, json)
}

pub fn add_recent_project(name: String, last_modified: String, path: String) {
    let mut config = load_app_config();

    // Prevent duplicates
    config.recent_projects.retain(|p| p.path != path);

    // Append to front of list
    config.recent_projects.insert(0, RecentProject {
        name,
        last_modified,
        path,
    });

    // Only keep up to recent 5
    config.recent_projects.truncate(5);

    let _ = save_app_config(&config);
}