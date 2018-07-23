CREATE TABLE teams (
    id serial PRIMARY KEY,
    name varchar NOT NULL,
    description text NOT NULL
);

CREATE TABLE services (
    id serial PRIMARY KEY,
    name varchar NOT NULL,
    description text NOT NULL
);

CREATE TABLE service_variants (
    id serial PRIMARY KEY,
    service_id serial REFERENCES services(id),
    description text NOT NULL,
    published_team_id serial REFERENCES teams(id),
    published_time timestamp with time zone NOT NULL
);

CREATE TABLE service_variant_attachments (
    id serial PRIMARY KEY,
    service_variant_id serial REFERENCES service_variants(id),
    name varchar NOT NULL,
    data bytea NOT NULL
);

CREATE TABLE service_providers (
    id serial PRIMARY KEY,
    team_id serial REFERENCES teams(id),
    connection_string text NOT NULL,
    service_variant_id serial REFERENCES services(id)
);

CREATE TABLE exploits (
    id serial PRIMARY KEY,
    name varchar NOT NULL,
    description text NOT NULL,
    last_modified_time timestamp with time zone NOT NULL
);

CREATE TABLE exploit_attachments (
    id serial PRIMARY KEY,
    exploit_id serial REFERENCES exploits(id),
    name varchar NOT NULL,
    data bytea NOT NULL
);

CREATE TABLE exploit_targets (
    exploit_id serial REFERENCES exploits(id),
    service_variant_id serial REFERENCES service_variants(id),
    CONSTRAINT exploit_targets_pk PRIMARY KEY (exploit_id, service_variant_id)
);

CREATE TABLE exploit_requests (
    id serial PRIMARY KEY,
    exploit_id serial REFERENCES exploits(id),
    retry_option integer NOT NULL
);

CREATE TABLE exploit_request_targets (
    id serial PRIMARY KEY,
    exploit_request_id serial REFERENCES exploit_requests(id),
    service_provider_id serial REFERENCES service_providers(id),
    CONSTRAINT exploit_request_targets_unique UNIQUE (exploit_request_id, service_provider_id)
);

CREATE TYPE exploit_status AS ENUM ('Pending', 'Running', 'Failed', 'OK');

CREATE TABLE exploit_logs (
    id SERIAL PRIMARY KEY,
    exploit_request_target_id SERIAL REFERENCES exploit_request_targets(id),
    status exploit_status NOT NULL
);

