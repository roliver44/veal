use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::{write::SimpleFileOptions, ZipWriter};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectConfig {
    pub project_name: String,
    pub threat_model: String,
    pub experiment_objective: String,
    pub target_text: Option<String>,
    pub dataset_path: Option<String>,
    pub created_at: String,
}

pub fn create_veal_project(
    project_name: &str,
    initial_dataset: &str,
    project_location: &str,
    target_text: &str,
    threat_model: &str,
    experiment_objective: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Prepare Paths
    let base_path = Path::new(project_location);
    if !base_path.exists() {
        std::fs::create_dir_all(base_path)?;
    }

    let project_filename = format!("{}.veal", project_name);
    let full_project_path = base_path.join(project_filename);

    // Create the zip container
    let file = File::create(&full_project_path)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    // Create inner directory structure
    zip.add_directory("data/", options)?;
    zip.add_directory("reports/", options)?;

    // Handle initial dataset
    let dataset_internal_path = if !initial_dataset.is_empty() {
        let ds_path = Path::new(initial_dataset);
        if ds_path.exists() {
            let filename = ds_path.file_name().unwrap_or_default().to_string_lossy();
            let internal_path = format!("data/{}", filename);

            // Read and copy file contents into zip
            zip.start_file(internal_path.clone(), options)?;
            let mut ds_file = File::open(ds_path)?;
            let mut buffer = Vec::new();
            ds_file.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;

            Some(internal_path)
        } else {
            None
        }
    } else {
        None
    };

    // Generate config.json
    let config = ProjectConfig {
        project_name: project_name.to_string(),
        threat_model: threat_model.to_string(),
        experiment_objective: experiment_objective.to_string(),
        target_text: if target_text.is_empty() { None } else { Some(target_text.to_string()) },
        dataset_path: dataset_internal_path,
        created_at: chrono::Local::now().to_rfc3339(),
    };

    let config_json = serde_json::to_string_pretty(&config)?;
    zip.start_file("config.json", options)?;
    zip.write_all(config_json.as_bytes())?;

    zip.finish()?;

    Ok(full_project_path)
}