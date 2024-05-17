use oblivion::{models::client::Client, types::client::Response};
use tauri::{command, utils::acl::Value, AppHandle, Runtime, State, Window};

use crate::{ClientInstance, ClientStore, Connections, Error, Result};

#[command]
pub(crate) async fn connect<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, Connections>,
    entrance: &str,
) -> Result<ClientInstance> {
    let mut connections = state.0.lock().await;
    let uuid = uuid::Uuid::new_v4().to_string();

    let client = Client::connect(entrance).await.unwrap();

    connections.insert(uuid.clone(), ClientStore::new(client, entrance.to_string()));
    Ok(ClientInstance {
        uuid,
        entrance: entrance.to_string(),
        message: "Connected".to_string(),
        success: true,
    })
}

#[command]
pub(crate) async fn close<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, Connections>,
    uuid: &str,
) -> Result<bool> {
    let mut connections = state.0.lock().await;

    if !connections.contains_key(uuid) {
        return Err(Error::InstanceNotFound(uuid.to_string()));
    }

    let store = connections.remove(uuid).unwrap();
    match store.client.close().await {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[command]
pub(crate) async fn send_json<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, Connections>,
    uuid: &str,
    data: Value,
) -> Result<bool> {
    let connections = state.0.lock().await;

    if !connections.contains_key(uuid) {
        return Err(Error::InstanceNotFound(uuid.to_string()));
    }

    let store = connections.get(uuid).unwrap();
    match store.client.send_json(data.into(), 200).await {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[command]
pub(crate) async fn recv<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, Connections>,
    uuid: &str,
) -> Result<Response> {
    let connections = state.0.lock().await;

    if !connections.contains_key(uuid) {
        return Err(Error::InstanceNotFound(uuid.to_string()));
    }

    let store = connections.get(uuid).unwrap();

    match store.client.recv().await {
        Ok(response) => Ok(response),
        Err(error) => Err(Error::from(error)),
    }
}

#[command]
pub(crate) async fn entrance<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, Connections>,
    uuid: &str,
) -> Result<String> {
    let connections = state.0.lock().await;

    if !connections.contains_key(uuid) {
        return Err(Error::InstanceNotFound(uuid.to_string()));
    }

    let store = connections.get(uuid).unwrap();

    Ok(store.entrance.clone())
}
