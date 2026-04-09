use crate::app::{App, RecentProject as SlintRecentProject};
use slint::{ModelRc, SharedString, VecModel};
use std::rc::Rc;
use crate::io::json::load_app_config;

pub fn init_state(ui: &App) {
    let config = load_app_config();

    let mut slint_recent_projects = Vec::new();
    for p in config.recent_projects.into_iter() {
        slint_recent_projects.push(SlintRecentProject {
            name: SharedString::from(p.name),
            last_modified: SharedString::from(p.last_modified),
            path: SharedString::from(p.path),
        });
    }

    let recent_projects_model = Rc::new(VecModel::from(slint_recent_projects));
    ui.set_recent_projects(ModelRc::from(recent_projects_model.clone()));
}