use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Oblivion<R>> {
    Ok(Oblivion(app.clone()))
}

/// Access to the oblivion APIs.
pub struct Oblivion<R: Runtime>(AppHandle<R>);
