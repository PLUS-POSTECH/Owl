use super::schema::*;
use chrono::{DateTime, Utc};

#[derive(Queryable, Identifiable)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Queryable, Identifiable)]
pub struct Service {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub published_time: DateTime<Utc>,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Service)]
#[belongs_to(Team, foreign_key = "publisher_id")]
pub struct ServiceVariant {
    pub id: i32,
    pub service_id: i32,
    pub name: String,
    pub sla_pass: Option<bool>,
    pub publisher_id: i32,
    pub published_time: DateTime<Utc>,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(ServiceVariant)]
pub struct ServiceVariantAttachment {
    pub id: i32,
    pub service_variant_id: i32,
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Team)]
#[belongs_to(ServiceVariant)]
pub struct ServiceProvider {
    pub id: i32,
    pub team_id: i32,
    pub service_variant_id: i32,
    pub connection_string: String,
    pub published_time: DateTime<Utc>,
}

#[derive(Queryable, Identifiable)]
pub struct Exploit {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub retry_option: i32,
    pub timeout_option: i32,
    pub auth_option: bool,
    pub last_modified_time: DateTime<Utc>,
    pub deleted: bool,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Exploit)]
pub struct ExploitAttachment {
    pub id: i32,
    pub exploit_id: i32,
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Exploit)]
#[belongs_to(ServiceVariant)]
#[primary_key(exploit_id, service_variant_id)]
pub struct ExploitTarget {
    pub exploit_id: i32,
    pub service_variant_id: i32,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Exploit)]
#[belongs_to(ServiceProvider)]
pub struct ExploitTask {
    pub id: i32,
    pub exploit_id: i32,
    pub service_provider_id: i32,
    pub retry_option_override: Option<i32>,
    pub timeout_option_override: Option<i32>,
    pub auth_option_override: Option<i32>,
    pub retries: i32,
    pub status: ExploitStatus,
    pub published_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
}
