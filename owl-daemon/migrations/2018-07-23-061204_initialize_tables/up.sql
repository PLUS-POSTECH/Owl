CREATE TABLE teams (
    id serial PRIMARY KEY,
    name varchar NOT NULL UNIQUE,
    description text NOT NULL,
    points integer NOT NULL
);

CREATE TABLE services (
    id serial PRIMARY KEY,
    name varchar NOT NULL UNIQUE,
    description text NOT NULL,
    enabled boolean NOT NULL,
    published_time timestamp with time zone NOT NULL
);

CREATE TABLE service_variants (
    id serial PRIMARY KEY,
    service_id serial REFERENCES services ON DELETE CASCADE,
    name varchar NOT NULL UNIQUE,
    published_team_id serial REFERENCES teams ON DELETE RESTRICT,
    published_time timestamp with time zone NOT NULL
);

CREATE TABLE service_variant_attachments (
    id serial PRIMARY KEY,
    service_variant_id serial REFERENCES service_variants ON DELETE CASCADE,
    name varchar NOT NULL,
    data bytea NOT NULL
);

CREATE TABLE service_providers (
    id serial PRIMARY KEY,
    team_id serial REFERENCES teams ON DELETE RESTRICT,
    service_variant_id serial REFERENCES service_variants ON DELETE RESTRICT,
    connection_string text NOT NULL,
    sla_pass boolean,
    exploited boolean,
    published_time timestamp with time zone NOT NULL
);

CREATE TABLE exploits (
    id serial PRIMARY KEY,
    name varchar NOT NULL UNIQUE,
    description text NOT NULL,
    enabled boolean NOT NULL,
    retry_option integer NOT NULL,
    timeout_option integer NOT NULL,
    flag_auth boolean NOT NULL,
    last_modified_time timestamp with time zone NOT NULL,
    deleted boolean NOT NULL
);

CREATE TABLE exploit_attachments (
    id serial PRIMARY KEY,
    exploit_id serial REFERENCES exploits ON DELETE CASCADE,
    name varchar NOT NULL,
    data bytea NOT NULL
);

CREATE TABLE exploit_targets (
    exploit_id serial REFERENCES exploits ON DELETE CASCADE,
    service_variant_id serial REFERENCES service_variants ON DELETE CASCADE,
    CONSTRAINT exploit_targets_pk PRIMARY KEY (exploit_id, service_variant_id)
);

CREATE TYPE exploit_status AS ENUM ('pending', 'running', 'authing', 'ok', 'run_failed', 'auth_failed');

CREATE TABLE exploit_tasks (
    id serial PRIMARY KEY,
    exploit_id serial REFERENCES exploits ON DELETE RESTRICT,
    service_provider_id serial REFERENCES service_providers ON DELETE RESTRICT,
    retry_option_override integer,
    timeout_option_override integer,
    flag_auth_override boolean,
    retries integer NOT NULL,
    status exploit_status NOT NULL,
    published_time timestamp with time zone NOT NULL,
    last_updated_time timestamp with time zone NOT NULL
);
