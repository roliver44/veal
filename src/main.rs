#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod core;
mod ui;
mod io;

fn main() {
    app::run();
}