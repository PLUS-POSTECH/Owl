use super::schema::*;
use chrono::{DateTime, Utc};

#[derive(Queryable, Identifiable)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(AsChangeset)]
#[table_name = "teams"]
pub struct TeamChangeset {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Queryable, Identifiable)]
pub struct Service {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub published_time: DateTime<Utc>,
}

#[derive(AsChangeset)]
#[table_name = "services"]
pub struct ServiceChangeset {
    pub name: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
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

#[derive(Insertable)]
#[table_name = "service_variants"]
pub struct ServiceVariantInsertable {
    pub service_id: i32,
    pub name: String,
    pub publisher_id: i32,
}

#[derive(AsChangeset)]
#[table_name = "service_variants"]
pub struct ServiceVariantChangeset {
    pub service_id: Option<i32>,
    pub name: Option<String>,
    pub sla_pass: Option<Option<bool>>,
    pub publisher_id: Option<i32>,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(ServiceVariant)]
pub struct ServiceVariantAttachment {
    pub id: i32,
    pub service_variant_id: i32,
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "service_variant_attachments"]
pub struct ServiceVariantAttachmentInsertable {
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

#[derive(Insertable)]
#[table_name = "service_providers"]
pub struct ServiceProviderInsertable {
    pub team_id: i32,
    pub service_variant_id: i32,
    pub connection_string: String,
}

#[derive(AsChangeset)]
#[table_name = "service_providers"]
pub struct ServiceProviderChangeset {
    pub connection_string: Option<String>,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Service)]
pub struct Exploit {
    pub id: i32,
    pub service_id: i32,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub max_retries: Option<i32>,
    pub timeout: Option<i32>,
    pub skip_auth: bool,
    pub last_modified_time: DateTime<Utc>,
    pub deleted: bool,
}

#[derive(Insertable)]
#[table_name = "exploits"]
pub struct ExploitInsertable {
    pub service_id: i32,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub max_retries: Option<i32>,
    pub timeout: Option<i32>,
    pub skip_auth: bool,
    pub deleted: bool,
}

#[derive(AsChangeset)]
#[table_name = "exploits"]
pub struct ExploitChangeset {
    pub service_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
    pub max_retries: Option<Option<i32>>,
    pub timeout: Option<Option<i32>>,
    pub skip_auth: Option<bool>,
    pub deleted: Option<bool>,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Exploit)]
pub struct ExploitAttachment {
    pub id: i32,
    pub exploit_id: i32,
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "exploit_attachments"]
pub struct ExploitAttachmentInsertable {
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
    pub retries: i32,
    pub status: ExploitStatus,
    pub published_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
}
