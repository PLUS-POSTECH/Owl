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
    Add {
        name: String,
        description: String,
    },
    Delete {
        name: String,
    },
    Update {
        name: String,
        description: Option<String>,
        enabled: Option<bool>,
    },
}

#[derive(Serialize, Deserialize)]
pub struct ServiceListParams {
    pub show_all: bool,
}
