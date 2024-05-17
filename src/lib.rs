use oblivion::models::client::Client;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};
use tokio::sync::Mutex;

use std::collections::HashMap;

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Oblivion;
#[cfg(mobile)]
use mobile::Oblivion;

#[derive(Default)]
struct Connections(Mutex<HashMap<String, Client>>);

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the oblivion APIs.
pub trait OblivionExt<R: Runtime> {
    fn oblivion(&self) -> &Oblivion<R>;
}

impl<R: Runtime, T: Manager<R>> crate::OblivionExt<R> for T {
    fn oblivion(&self) -> &Oblivion<R> {
        self.state::<Oblivion<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("oblivion")
        .invoke_handler(tauri::generate_handler![commands::connect])
        .setup(|app, api| {
            #[cfg(mobile)]
            let oblivion = mobile::init(app, api)?;
            #[cfg(desktop)]
            let oblivion = desktop::init(app, api)?;
            app.manage(oblivion);

            // manage state so it is accessible by the commands
            app.manage(Connections::default());
            Ok(())
        })
        .build()
}
