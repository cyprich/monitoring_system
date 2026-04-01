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
    total_swap_mb integer not null,
    cpu_count integer not null,
    primary key (id)
);

create table drives (
    mountpoint varchar, 
    collector_id integer, 
    capacity_gb integer not null, 
    file_system varchar not null,
    primary key (mountpoint, collector_id), 
    foreign key (collector_id) references collectors(id)
);

create table network_interfaces (
    name varchar, 
    collector_id integer, 
    mac varchar not null, 
    primary key (name, collector_id), 
    foreign key (collector_id) references collectors(id)
);

-- TODO component as separate table 

-- create table components (
--     id serial, 
--     collector_id integer, 
--     name varchar not null, 
--     threshold_value double precision, -- if this value is exceeded
--     threshold_count integer, -- number of consecutive exceeds to be notified
--     primary key (id), 
--     foreign key (collector_id) references collectors(id)
-- );

create table metric_type (
    name varchar primary key
);

create table metrics (
    timestamp timestamp,
    value double precision,
    metric_type varchar,
    collector_id integer,
    component_name varchar ,  -- used when we have multiple disks/network interfaces
    primary key (timestamp, value, metric_type, collector_id, component_name),
    foreign key (metric_type) references metric_type(name),
    foreign key (collector_id) references collectors(id)
);

-- create type request_method as enum (
--     'get', 
-- );

create table endpoints (
    id serial, 
    collector_id integer, 
    -- method request_method, 
    url varchar, 
    expected_codes integer[],
    primary key (id),
    foreign key (collector_id) references collectors(id),
    unique(collector_id, url)
);

create table endpoints_result (
    endpoint_id integer, 
    timestamp timestamp,
    result boolean not null, 
    latency_microseconds bigint,
    primary key (endpoint_id, timestamp), 
    foreign key (endpoint_id) references endpoints(id)
)

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
    metric_type varchar not null, 
    component_name varchar not null, 
    threshold_value double precision not null, 
    measured_values double precision[] not null,
    -- severity integer,
    timestamp timestamp not null,
    primary key (id),
    foreign key (collector_id) references collectors(id),
    foreign key (metric_type) references metric_type(name)
    -- foreign key (severity) references notification_severity(id)
);

create table thresholds (
    id serial, 
    collector_id integer not null, 
    component_name varchar not null, 
    metric_type varchar not null, 
    value double precision not null,
    primary key (id), 
    foreign key (collector_id) references collectors(id), 
    foreign key (metric_type) references metric_type(name),
    unique(collector_id, component_name, metric_type)
);

--------------------- insert metric types ---------------------

insert into metric_type (name) values 
   ('cpu_usage'), 
   ('used_memory_mb'), 
   ('used_swap_mb'), 
   ('drive_used_space'), 
   ('network_download'), 
   ('network_upload')
;
