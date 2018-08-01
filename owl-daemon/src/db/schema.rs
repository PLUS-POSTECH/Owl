#[derive(SqlType)]
#[postgres(type_name = "exploit_status")]
pub struct ExploitStatusType;

table! {
    exploit_attachments (id) {
        id -> Int4,
        exploit_id -> Int4,
        name -> Varchar,
        data -> Bytea,
    }
}

table! {
    use diesel::sql_types::*;
    use super::ExploitStatusType;

    exploit_logs (id) {
        id -> Int4,
        exploit_request_target_id -> Int4,
        status -> ExploitStatusType,
    }
}

table! {
    exploit_request_targets (id) {
        id -> Int4,
        exploit_request_id -> Int4,
        service_provider_id -> Int4,
    }
}

table! {
    exploit_requests (id) {
        id -> Int4,
        exploit_id -> Int4,
        retry_option -> Int4,
    }
}

table! {
    exploit_targets (exploit_id, service_variant_id) {
        exploit_id -> Int4,
        service_variant_id -> Int4,
    }
}

table! {
    exploits (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        last_modified_time -> Timestamptz,
    }
}

table! {
    service_providers (id) {
        id -> Int4,
        team_id -> Int4,
        connection_string -> Text,
        service_variant_id -> Int4,
    }
}

table! {
    service_variant_attachments (id) {
        id -> Int4,
        service_variant_id -> Int4,
        name -> Varchar,
        data -> Bytea,
    }
}

table! {
    service_variants (id) {
        id -> Int4,
        service_id -> Int4,
        description -> Text,
        published_team_id -> Int4,
        published_time -> Timestamptz,
    }
}

table! {
    services (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
    }
}

table! {
    teams (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
    }
}

joinable!(exploit_attachments -> exploits (exploit_id));
joinable!(exploit_logs -> exploit_request_targets (exploit_request_target_id));
joinable!(exploit_request_targets -> exploit_requests (exploit_request_id));
joinable!(exploit_request_targets -> service_providers (service_provider_id));
joinable!(exploit_requests -> exploits (exploit_id));
joinable!(exploit_targets -> exploits (exploit_id));
joinable!(exploit_targets -> service_variants (service_variant_id));
joinable!(service_providers -> services (service_variant_id));
joinable!(service_providers -> teams (team_id));
joinable!(service_variant_attachments -> service_variants (service_variant_id));
joinable!(service_variants -> services (service_id));
joinable!(service_variants -> teams (published_team_id));

allow_tables_to_appear_in_same_query!(
    exploit_attachments,
    exploit_logs,
    exploit_request_targets,
    exploit_requests,
    exploit_targets,
    exploits,
    service_providers,
    service_variant_attachments,
    service_variants,
    services,
    teams,
);
