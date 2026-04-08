slint::include_modules!();

pub fn run() {
    let ui = App::new().unwrap();

    crate::ui::state::init_state(&ui);
    crate::ui::bindings::setup_bindings(&ui);

    ui.run().unwrap();
}