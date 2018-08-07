use chrono::{DateTime, Utc};
use model::FileEntry;

// Filter conditions are combined using OR within field and combined using AND across fields
#[derive(Serialize, Deserialize)]
pub struct ServiceVariantListParams {
    pub show_all: bool,
    pub filter_teams: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceVariantData {
    pub name: String,
    pub service_name: String,
    pub publisher_name: String,
    pub published_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceVariantDownloadParams {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceVariantAttachmentData {
    pub file_entries: Vec<FileEntry>,
}

#[derive(Serialize, Deserialize)]
pub enum ServiceVariantEditParams {
    Add {
        service_name: String,
        publisher_name: String,
        files: Vec<FileEntry>,
    },
    Delete {
        name: String,
    },
    Update {
        name: String,
        service_name: Option<String>,
        publisher_name: Option<String>,
        sla_pass: Option<Option<bool>>,
    },
}
