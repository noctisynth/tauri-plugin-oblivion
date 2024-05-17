use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientInstance {
    pub uuid: String,
    pub entrance: String,
    pub message: String,
    pub success: bool,
}
