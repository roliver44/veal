use crate::app::App;
use rfd::FileDialog;
use slint::{ComponentHandle, SharedString, Weak};
use crate::io::project::create_veal_project;
use crate::io::loader::validate_and_load_project;
use crate::io::json::add_recent_project;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::cell::RefCell;
use crate::io::json::load_json_vector;
use crate::io::npy::load_npy_vector;
use crate::core::stats::{compute_stats, compute_matrix_stats};
use crate::core::types::VectorData;

fn extract_to_temp(archive_path: &Path, internal_path: &str) -> Option<PathBuf> {
    let file = std::fs::File::open(archive_path).ok()?;
    let mut archive = zip::ZipArchive::new(file).ok()?;
    let mut zip_file = archive.by_name(internal_path).ok()?;

    let temp_dir = std::env::temp_dir();
    let file_name = Path::new(internal_path).file_name().unwrap_or_default();
    let temp_path = temp_dir.join(file_name);

    let mut out_file = std::fs::File::create(&temp_path).ok()?;
    std::io::copy(&mut zip_file, &mut out_file).ok()?;
    Some(temp_path)
}

fn reset_dataset_ui_state(ui: &App, shared_dataset: &Rc<RefCell<Option<VectorData>>>) {
    ui.set_global_vector_count(0);
    ui.set_global_vector_dimension(0);
    ui.set_global_min_val(SharedString::from("0.0000"));
    ui.set_global_max_val(SharedString::from("0.0000"));
    ui.set_global_mean_val(SharedString::from("0.0000"));
    ui.set_global_frobenius_norm(SharedString::from("0.0000"));

    *shared_dataset.borrow_mut() = None;
    ui.set_stat_vector_index(-1);
}

fn handle_project_load(path: &Path, ui_handle: Weak<App>, shared_dataset: Rc<RefCell<Option<VectorData>>>) {
    println!("Validating project at: {:?}", path);
    match validate_and_load_project(path) {
        Ok(config) => {
            println!("Successfully loaded project: {}", config.project_name);

            add_recent_project(
                config.project_name.clone(),
                chrono::Local::now().format("%b %e, %I:%M %p").to_string(),
                path.to_string_lossy().to_string()
            );

            if let Some(ui) = ui_handle.upgrade() {
                ui.set_active_project_name(SharedString::from(config.project_name));

                if let Some(dataset_path) = config.dataset_path {
                    ui.set_active_dataset_name(SharedString::from(dataset_path.clone()));

                    // attempt to extract dataset .veal archive to a tmp file
                    let mut ds_path_buf = PathBuf::from(&dataset_path);
                    if let Some(extracted_path) = extract_to_temp(path, &dataset_path) {
                        ds_path_buf = extracted_path;
                    }

                    let ds_path = ds_path_buf.as_path();

                    let load_result = if ds_path.extension().map_or(false, |ext| ext == "json") {
                        load_json_vector(ds_path)
                    } else if ds_path.extension().map_or(false, |ext| ext == "npy") {
                        load_npy_vector(ds_path)
                    } else {
                        Err("Unsupported format".into())
                    };

                    match load_result {
                        Ok(data) => {
                            // Calculate matrix 'global' stats
                            match &data {
                                VectorData::Single(vec) => {
                                    let stats = compute_stats(vec);
                                    ui.set_global_vector_count(1);
                                    ui.set_global_vector_dimension(stats.dimension as i32);
                                    ui.set_global_min_val(SharedString::from(format!("{:.4}", stats.min_val)));
                                    ui.set_global_max_val(SharedString::from(format!("{:.4}", stats.max_val)));
                                    ui.set_global_mean_val(SharedString::from(format!("{:.4}", stats.mean_val)));
                                    ui.set_global_frobenius_norm(SharedString::from(format!("{:.4}", stats.l2_norm)));
                                }
                                VectorData::Matrix(mat) => {
                                    let global_stats = compute_matrix_stats(mat);
                                    ui.set_global_vector_count(mat.len() as i32);
                                    ui.set_global_vector_dimension(global_stats.dimension as i32);
                                    ui.set_global_min_val(SharedString::from(format!("{:.4}", global_stats.min_val)));
                                    ui.set_global_max_val(SharedString::from(format!("{:.4}", global_stats.max_val)));
                                    ui.set_global_mean_val(SharedString::from(format!("{:.4}", global_stats.mean_val)));
                                    ui.set_global_frobenius_norm(SharedString::from(format!("{:.4}", global_stats.l2_norm)));
                                }
                            }

                            // Cache it to RefCell
                            *shared_dataset.borrow_mut() = Some(data);

                            // unselect vector visually
                            ui.set_stat_vector_index(-1);
                        }
                        Err(e) => {
                            eprintln!("Failed to load vectors: {}", e);
                            reset_dataset_ui_state(&ui, &shared_dataset);
                        }
                    }

                } else {
                    ui.set_active_dataset_name(SharedString::from("No Dataset Loaded"));
                    reset_dataset_ui_state(&ui, &shared_dataset);
                }

                ui.set_is_project_loaded(true);

                // max window auto when project loads
                // This works for now
                // in future we need to migrate to two slint Apps
                // one for entry and one for main...
                ui.window().set_maximized(true);
            }
        }
        Err(e) => {
            eprintln!("Failed to load project: {}", e);
        }
    }
}

pub fn setup_bindings(ui: &App) {
    if let Some(mut doc_dir) = dirs::document_dir() {
        doc_dir.push("veal");
        ui.set_default_location(SharedString::from(doc_dir.to_string_lossy().as_ref()));
    }

    let ui_handle = ui.as_weak();
    let shared_dataset: Rc<RefCell<Option<VectorData>>> = Rc::new(RefCell::new(None));

    // Handle requesting stats for an individual vector
    ui.on_request_vector_stats({
        let ui_handle = ui_handle.clone();
        let shared_dataset = shared_dataset.clone();
        move |index| {
            if let Some(ui) = ui_handle.upgrade() {
                if let Some(data) = &*shared_dataset.borrow() {
                    let maybe_vec = match data {
                        VectorData::Single(vec) => if index == 0 { Some(vec) } else { None },
                        VectorData::Matrix(mat) => mat.get(index as usize),
                    };

                    if let Some(vec) = maybe_vec {
                        let stats = compute_stats(vec);
                        ui.set_stat_vector_dimension(stats.dimension as i32);
                        ui.set_stat_min_val(SharedString::from(format!("{:.4}", stats.min_val)));
                        ui.set_stat_max_val(SharedString::from(format!("{:.4}", stats.max_val)));
                        ui.set_stat_mean_val(SharedString::from(format!("{:.4}", stats.mean_val)));
                        ui.set_stat_l2_norm(SharedString::from(format!("{:.4}", stats.l2_norm)));
                        ui.set_stat_is_normalized(stats.l2_norm >= 0.99 && stats.l2_norm <= 1.01);
                    }
                }
            }
        }
    });

    ui.on_request_file_open({
        let ui_handle = ui_handle.clone();
        let shared_dataset = shared_dataset.clone();
        move || {
            if let Some(path) = FileDialog::new()
                .add_filter("Veal Project", &["veal", "zip"])
                .set_title("Open Veal Project")
                .pick_file()
            {
                handle_project_load(&path, ui_handle.clone(), shared_dataset.clone());
            }
        }
    });

    ui.on_request_open_recent({
        let ui_handle = ui_handle.clone();
        let shared_dataset = shared_dataset.clone();
        move |path_str| {
            let path = Path::new(path_str.as_str());
            handle_project_load(&path, ui_handle.clone(), shared_dataset.clone());
        }
    });

    ui.on_request_select_location({
        let ui_handle = ui_handle.clone();
        move || {
            if let Some(path) = FileDialog::new()
                .set_title("Select Project Location")
                .pick_folder()
            {
                if let Some(ui) = ui_handle.upgrade() {
                    ui.set_default_location(SharedString::from(path.to_string_lossy().as_ref()));
                }
            }
        }
    });

    ui.on_request_select_dataset({
        let ui_handle = ui_handle.clone();
        move || {
            if let Some(path) = FileDialog::new()
                .add_filter("Dataset", &["npy", "json"])
                .set_title("Select Initial Dataset")
                .pick_file()
            {
                if let Some(ui) = ui_handle.upgrade() {
                    ui.set_initial_dataset(SharedString::from(path.to_string_lossy().as_ref()));
                }
            }
        }
    });

    ui.on_request_project_create({
        let ui_handle = ui_handle.clone();
        let shared_dataset = shared_dataset.clone();
        move |project_name, initial_dataset, project_location, target_text, threat_model, experiment_objective| {
            println!("Creating project: {}", project_name);

            match create_veal_project(
                &project_name,
                &initial_dataset,
                &project_location,
                &target_text,
                &threat_model,
                &experiment_objective,
            ) {
                Ok(path) => {
                    println!("Successfully created project at: {:?}", path);

                    add_recent_project(
                        project_name.to_string(),
                        chrono::Local::now().format("%b %e, %I:%M %p").to_string(),
                        path.to_string_lossy().to_string()
                    );

                    // Route user to main lab interface
                    handle_project_load(&path, ui_handle.clone(), shared_dataset.clone());
                }
                Err(e) => {
                    eprintln!("Failed to create project: {}", e);
                }
            }
        }
    });
}