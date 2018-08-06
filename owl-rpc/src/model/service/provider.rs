use chrono::{DateTime, Utc};

// Filter conditions are combined using OR within field and combined using AND across fields
#[derive(Serialize, Deserialize)]
pub struct ServiceProviderListParams {
    pub show_all: bool,
    pub filter_teams: Vec<String>,
    pub filter_service_variants: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceProviderData {
    pub team_name: String,
    pub service_variant_name: String,
    pub connection_string: String,
    pub published_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceProviderUpdateParams {
    pub team_name: String,
    pub service_variant_name: String,
    pub connection_string: Option<String>,
}
