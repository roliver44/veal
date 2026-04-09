use crate::app::App;
use rfd::FileDialog;
use slint::{ComponentHandle, SharedString};
use crate::io::project::create_veal_project;
use crate::io::loader::validate_and_load_project;
use crate::io::json::add_recent_project;
use std::path::Path;

fn handle_project_load(path: &Path) {
    println!("Validating project at: {:?}", path);
    match validate_and_load_project(path) {
        Ok(config) => {
            println!("Successfully loaded project: {}", config.project_name);

            // Add to recent projects
            add_recent_project(
                config.project_name,
                chrono::Local::now().format("%b %e, %I:%M %p").to_string(),
                path.to_string_lossy().to_string()
            );

            // Route user to main lab interface
            // if let Some(ui) = ui_handle.upgrade() {
            //       ui.set_is_project_loaded(true);
            // }
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

    ui.on_request_file_open({
        let _ui_handle = ui_handle.clone();
        move || {
            if let Some(path) = FileDialog::new()
                .add_filter("Veal Project", &["veal", "zip"])
                .set_title("Open Veal Project")
                .pick_file()
            {
                handle_project_load(&path);
            }
        }
    });

    ui.on_request_open_recent({
        let _ui_handle = ui_handle.clone();
        move |path_str| {
            let path = Path::new(path_str.as_str());
            handle_project_load(&path);
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
                .add_filter("Dataset", &["csv", "json"])
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
        let _ui_handle = ui_handle.clone();
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

                    // Add to recent projects
                    add_recent_project(
                        project_name.to_string(),
                        chrono::Local::now().format("%b %e, %I:%M %p").to_string(),
                        path.to_string_lossy().to_string()
                    );

                    // if let Some(ui) = _ui_handle.upgrade() {
                    //       ui.set_is_project_loaded(true);
                    // }
                }
                Err(e) => {
                    eprintln!("Failed to create project: {}", e);
                }
            }
        }
    });
}