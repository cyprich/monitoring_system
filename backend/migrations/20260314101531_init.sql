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
    mountpoint varchar not null, 
    collector_id integer not null, 
    capacity_gb integer not null, 
    file_system varchar not null,
    primary key (mountpoint, collector_id), 
    foreign key (collector_id) references collectors(id)
);

create table network_interfaces (
    name varchar not null, 
    collector_id integer not null, 
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
    timestamp timestamp not null,
    value double precision not null,
    metric_type varchar not null,
    collector_id integer not null,
    component_name varchar not null,  -- used when we have multiple disks/network interfaces
    primary key (timestamp, value, metric_type, collector_id, component_name),
    foreign key (metric_type) references metric_type(name),
    foreign key (collector_id) references collectors(id)
);

-- create type request_method as enum (
--     'get', 
-- );

create table endpoints (
    id serial not null, 
    collector_id integer not null, 
    -- method request_method, 
    url varchar not null, 
    expected_codes integer[],
    primary key (id),
    foreign key (collector_id) references collectors(id),
    unique(collector_id, url)
);

create table endpoints_results (
    endpoint_id integer not null, 
    timestamp timestamp not null,
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
    id serial not null,
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

-- TODO count? 
-- like how many consecutive values have to be above limit

create table metrics_thresholds (
    id serial not null, 
    collector_id integer not null, 
    metric_type varchar not null, 
    component_name varchar not null, 
    value double precision not null,
    primary key (id), 
    foreign key (collector_id) references collectors(id), 
    foreign key (metric_type) references metric_type(name),
    unique(collector_id, metric_type, component_name)
);

create table endpoints_thresholds (
    id serial not null, 
    endpoint_id integer not null, 
    value integer not null, 
    primary key (id), 
    foreign key (endpoint_id) references endpoints(id)
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
