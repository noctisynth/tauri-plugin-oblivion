use oblivion::models::client::Client;
use tauri::{command, AppHandle, Runtime, State, Window};

use crate::{Connections, Result};

#[command]
pub(crate) async fn connect<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, Connections>,
    entrance: &str,
) -> Result<String> {
    let mut connections = state.0.lock().await;

    let client = Client::connect(entrance).await.unwrap();

    connections.insert(entrance.to_string(), client);
    Ok("success".to_string())
}
