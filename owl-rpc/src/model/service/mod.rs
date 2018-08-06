use chrono::{DateTime, Utc};

pub mod provider;
pub mod variant;

#[derive(Serialize, Deserialize)]
pub struct ServiceData {
    pub name: String,
    pub description: String,
    pub published_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub enum ServiceEditParams {
    Add(ServiceAddParams),
    Delete(ServiceDeleteParams),
    Update(ServiceUpdateParams),
}

#[derive(Serialize, Deserialize)]
pub struct ServiceAddParams {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceDeleteParams {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceUpdateParams {
    pub name: String,
    pub description: Option<String>,
    pub enabled: Option<bool>,
}
