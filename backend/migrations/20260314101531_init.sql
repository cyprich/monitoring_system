-- Add migration script here

-- drop table notifications;
-- -- drop table notification_severity;
-- -- drop table users;
-- drop table metrics;
-- drop table metric_type;
-- drop table collectors;
-- drop table _sqlx_migrations;

---------- CREATE TABLES ----------

create table collectors (
    id serial,
    name varchar not null,
    system_name varchar not null,
    host_name varchar not null,
    kernel_version varchar not null,
    total_memory_mb integer not null,
    cpu_count integer not null,
    primary key (id)
);

-- create table metric_type (
--     id serial,
--     name varchar not null,
--     primary key (id)
-- );

create type metric_type as enum (
    'cpu_usage',
    'used_memory_mb'
);

create table metrics (
    timestamp timestamp,
    value double precision,
    metric_type metric_type,
    collector_id integer,
    component_name varchar,  -- used when we have multiple disks/network interfaces
    primary key (timestamp, value, type, collector_id, component_name),
    foreign key (collector_id) references collectors(id)
);

-- create table users (
--     id serial,
--     name varchar,
--     password_hash varchar,
--     primary key (id)
-- );

-- create table notification_severity (
--     id serial,
--     severity varchar,
--     primary key (id)
-- );

create table notifications (
    id serial,
    collector_id integer not null,
    -- severity integer,
    description varchar not null,
    timestamp timestamp not null,
    viewed boolean default false,
    primary key (id),
    foreign key (collector_id) references collectors(id)
    -- foreign key (severity) references notification_severity(id)
);
