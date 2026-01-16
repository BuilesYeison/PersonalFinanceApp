use serde::Serialize;

#[derive(serde::Serialize)]
pub struct WorkspaceContext {
    pub name: String,
    pub path: String,
    pub currency: String,
    pub theme: String,
}
