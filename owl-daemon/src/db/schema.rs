table! {
    exploit_attachments (id) {
        id -> Int4,
        exploit_id -> Int4,
        name -> Varchar,
        data -> Bytea,
    }
}

table! {
    exploits (id) {
        id -> Int4,
        service_id -> Int4,
        name -> Varchar,
        description -> Text,
        enabled -> Bool,
        max_retries -> Nullable<Int4>,
        timeout -> Nullable<Int4>,
        skip_auth -> Bool,
        last_modified_time -> Timestamptz,
        deleted -> Bool,
    }
}

table! {
    exploit_targets (exploit_id, service_variant_id) {
        exploit_id -> Int4,
        service_variant_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use db::models::ExploitStatusMapping;

    exploit_tasks (id) {
        id -> Int4,
        exploit_id -> Int4,
        service_provider_id -> Int4,
        retries -> Int4,
        status -> ExploitStatusMapping,
        message -> Text,
        published_time -> Timestamptz,
        last_updated_time -> Timestamptz,
    }
}

table! {
    service_providers (id) {
        id -> Int4,
        team_id -> Int4,
        service_variant_id -> Int4,
        connection_string -> Text,
        published_time -> Timestamptz,
    }
}

table! {
    services (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        enabled -> Bool,
        published_time -> Timestamptz,
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
        name -> Varchar,
        sla_pass -> Nullable<Bool>,
        publisher_id -> Int4,
        published_time -> Timestamptz,
    }
}

table! {
    teams (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
    }
}

joinable!(exploits -> services (service_id));
joinable!(exploit_attachments -> exploits (exploit_id));
joinable!(exploit_targets -> exploits (exploit_id));
joinable!(exploit_targets -> service_variants (service_variant_id));
joinable!(exploit_tasks -> exploits (exploit_id));
joinable!(exploit_tasks -> service_providers (service_provider_id));
joinable!(service_providers -> service_variants (service_variant_id));
joinable!(service_providers -> teams (team_id));
joinable!(service_variant_attachments -> service_variants (service_variant_id));
joinable!(service_variants -> services (service_id));
joinable!(service_variants -> teams (publisher_id));

allow_tables_to_appear_in_same_query!(
    exploit_attachments,
    exploits,
    exploit_targets,
    exploit_tasks,
    service_providers,
    services,
    service_variant_attachments,
    service_variants,
    teams,
);
