use crate::app::App;
use rfd::FileDialog;

pub fn setup_bindings(ui: &App) {
    ui.on_request_file_open(move || {
        // prompt user to select a file
        let file = FileDialog::new()
            .add_filter("Veal Project", &["veal", "zip"])
            .set_title("Open Veal Project")
            .pick_file();

        if let Some(_path) = file {
            // validate file or return here
        }
    });
}