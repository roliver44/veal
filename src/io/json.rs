use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::error::Error;
use crate::core::types::VectorData;

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

pub fn load_json_vector(file_path: &Path) -> Result<VectorData, Box<dyn Error>> {
    let file = File::open(file_path).map_err(|e| format!("Failed to open JSON file '{:?}': {}", file_path, e))?;
    let json_value: serde_json::Value = serde_json::from_reader(file).map_err(|e| format!("Failed to parse JSON file: {}", e))?;

    if let Some(arr) = json_value.as_array() {
        if arr.is_empty() {
            return Ok(VectorData::Single(vec![]));
        }

        // Check if it's an array of floats just a single vector
        if arr[0].is_f64() {
            let mut vector = Vec::with_capacity(arr.len());
            for val in arr {
                vector.push(val.as_f64().ok_or("Expected a float value")? as f32);
            }
            return Ok(VectorData::Single(vector));
        }

        // Check if it's an array of arrays aka a matrix
        if arr[0].is_array() {
            let mut matrix = Vec::with_capacity(arr.len());
            for row in arr {
                let row_arr = row.as_array().ok_or("Expected an array of arrays")?;
                let mut vector = Vec::with_capacity(row_arr.len());
                for val in row_arr {
                    vector.push(val.as_f64().ok_or("Expected a float value inside the array")? as f32);
                }
                matrix.push(vector);
            }
            return Ok(VectorData::Matrix(matrix));
        }
    }

    Err("JSON File is wrong. Expected an array or an array of arrays.".into())
}