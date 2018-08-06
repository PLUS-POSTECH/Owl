use chrono::{DateTime, Utc};
use model::FileEntry;

#[derive(Serialize, Deserialize)]
pub struct ServiceVariantFetchParams {
    pub name: String,
}

// Filter conditions are combined using OR within field and combined using AND across fields
#[derive(Serialize, Deserialize)]
pub struct ServiceVariantListParams {
    pub show_all: bool,
    pub filter_providers: Vec<String>,
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
    Add(ServiceVariantAddParams),
    Delete(ServiceVariantDeleteParams),
    Update(ServiceVariantUpdateParams),
}

#[derive(Serialize, Deserialize)]
pub struct ServiceVariantAddParams {
    pub service_name: String,
    pub publisher_name: String,
    pub files: Vec<FileEntry>,
}

pub type ServiceVariantDeleteParams = ServiceVariantFetchParams;

#[derive(Serialize, Deserialize)]
pub struct ServiceVariantUpdateParams {
    pub name: String,
    pub service_name: Option<String>,
    pub publisher_name: Option<String>,
    pub sla_pass: Option<Option<bool>>,
}
