use std::fs::File;
use std::path::{Path};
use zip::ZipArchive;
use crate::io::project::ProjectConfig;

#[derive(Debug)]
pub enum ProjectLoadError {
    IoError(std::io::Error),
    ZipError(zip::result::ZipError),
    ParseError(serde_json::Error),
    InvalidFormat(String),
}

impl From<std::io::Error> for ProjectLoadError {
    fn from(err: std::io::Error) -> Self {
        ProjectLoadError::IoError(err)
    }
}

impl From<zip::result::ZipError> for ProjectLoadError {
    fn from(err: zip::result::ZipError) -> Self {
        ProjectLoadError::ZipError(err)
    }
}

impl From<serde_json::Error> for ProjectLoadError {
    fn from(err: serde_json::Error) -> Self {
        ProjectLoadError::ParseError(err)
    }
}

impl std::fmt::Display for ProjectLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectLoadError::IoError(e) => write!(f, "IO Error: {}", e),
            ProjectLoadError::ZipError(e) => write!(f, "Zip Error: {}", e),
            ProjectLoadError::ParseError(e) => write!(f, "Parse Error: {}", e),
            ProjectLoadError::InvalidFormat(msg) => write!(f, "Invalid Format: {}", msg),
        }
    }
}

impl std::error::Error for ProjectLoadError {}

pub fn validate_and_load_project(path: &Path) -> Result<ProjectConfig, ProjectLoadError> {
    // Verify file exists and has correct extension
    if !path.exists() {
        return Err(ProjectLoadError::InvalidFormat("File does not exist.".to_string()));
    }

    if let Some(ext) = path.extension() {
        if ext != "veal" && ext != "zip" {
            return Err(ProjectLoadError::InvalidFormat("File must have a .veal or .zip extension.".to_string()));
        }
    } else {
        return Err(ProjectLoadError::InvalidFormat("File has no extension.".to_string()));
    }

    // Open as Zip Archive
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;

    // check config.json at the root
    let mut config_file = match archive.by_name("config.json") {
        Ok(file) => file,
        Err(_) => return Err(ProjectLoadError::InvalidFormat("Missing config.json in project root.".to_string())),
    };

    // Parse config.json
    let config: ProjectConfig = serde_json::from_reader(&mut config_file)?;

    Ok(config)
}