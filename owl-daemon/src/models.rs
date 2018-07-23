use std::io::Write;

use chrono::{DateTime, Utc};
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};

use schema::*;

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
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Service)]
#[belongs_to(Team, foreign_key="published_team_id")]
pub struct ServiceVariant {
    pub id: i32,
    pub service_id: i32,
    pub description: String,
    pub published_team_id: i32,
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
    pub connection_string: String,
    pub service_variant_id: i32,
}

#[derive(Queryable, Identifiable)]
pub struct Exploit {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub last_modified_time: DateTime<Utc>,
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
pub struct ExploitRequest {
    pub id: i32,
    pub exploit_id: i32,
    pub retry_option: i32,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(ExploitRequest)]
#[belongs_to(ServiceProvider)]
pub struct ExploitRequestTarget {
    pub id: i32,
    pub exploit_request_id: i32,
    pub service_provider_id: i32,
}

#[derive(SqlType)]
#[postgres(type_name="Exploit_status")]
pub struct ExploitStatusType;

#[derive(Debug, FromSqlRow, AsExpression)]
#[sql_type = "ExploitStatusType"]
pub enum ExploitStatus {
    Pending,
    Running,
    Failed,
    OK,
}

impl ToSql<ExploitStatusType, Pg> for ExploitStatus {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            ExploitStatus::Pending => out.write_all(b"Pending")?,
            ExploitStatus::Running => out.write_all(b"Running")?,
            ExploitStatus::Failed => out.write_all(b"Failed")?,
            ExploitStatus::OK => out.write_all(b"OK")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<ExploitStatusType, Pg> for ExploitStatus {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"Pending" => Ok(ExploitStatus::Pending),
            b"Running" => Ok(ExploitStatus::Running),
            b"Failed" => Ok(ExploitStatus::Failed),
            b"OK" => Ok(ExploitStatus::OK),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(ExploitRequestTarget)]
pub struct ExploitLog {
    pub id: i32,
    pub exploit_request_target_id: i32,
    pub status: ExploitStatus,
}
