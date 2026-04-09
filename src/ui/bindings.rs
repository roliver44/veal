use crate::app::App;
use rfd::FileDialog;
use slint::{ComponentHandle, SharedString};

pub fn setup_bindings(ui: &App) {
    if let Some(mut doc_dir) = dirs::document_dir() {
        doc_dir.push("veal");
        ui.set_default_location(SharedString::from(doc_dir.to_string_lossy().as_ref()));
    }

    let ui_handle = ui.as_weak();

    ui.on_request_file_open(move || {
        if let Some(_path) = FileDialog::new()
            .add_filter("Veal Project", &["veal", "zip"])
            .set_title("Open Veal Project")
            .pick_file()
        {
            // validate file or return here
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

    ui.on_request_project_create(move |project_name, initial_dataset, project_location, target_text, threat_model, experiment_objective| {
        println!("Creating project: {}", project_name);
        println!("Location: {}", project_location);
        println!("Dataset: {}", initial_dataset);
        println!("Target Text: {}", target_text);
        println!("Threat Model: {}", threat_model);
        println!("Experiment Objective: {}", experiment_objective);

        // Handle creating .veal / .zip file here
    });
}