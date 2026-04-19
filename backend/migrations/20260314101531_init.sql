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

create table metric_type (
    name varchar primary key
);

create table metrics (
    time timestamptz not null,
    value double precision not null,
    metric_type varchar not null,
    collector_id integer not null,
    component_name varchar not null,  -- used when we have multiple disks/network interfaces
    primary key (time, value, metric_type, collector_id, component_name),
    foreign key (metric_type) references metric_type(name),
    foreign key (collector_id) references collectors(id)
);

create table endpoints (
    id serial not null, 
    collector_id integer not null, 
    url varchar not null, 
    expected_codes integer[] not null,
    primary key (id),
    foreign key (collector_id) references collectors(id),
    unique(collector_id, url)
);

create table endpoints_results (
    endpoint_id integer not null, 
    time timestamptz not null,
    result boolean not null, 
    latency_microseconds bigint,
    primary key (endpoint_id, time), 
    foreign key (endpoint_id) references endpoints(id)
);

create table notifications (
    id serial not null,
    collector_id integer not null,
    cause varchar not null, 
    description varchar,
    -- severity integer,
    time timestamptz not null,
    primary key (id),
    foreign key (collector_id) references collectors(id)
    -- foreign key (severity) references notification_severity(id)
);

create table metrics_thresholds (
    id serial not null, 
    collector_id integer not null, 
    metric_type varchar not null, 
    component_name varchar not null, 
    value double precision not null,
    count integer not null,
    primary key (id), 
    foreign key (collector_id) references collectors(id), 
    foreign key (metric_type) references metric_type(name),
    unique(collector_id, metric_type, component_name)
);

create table endpoints_thresholds (
    id serial not null, 
    endpoint_id integer not null, 
    count integer not null, 
    primary key (id), 
    foreign key (endpoint_id) references endpoints(id),
    unique(endpoint_id)
);

create table ports (
    id serial, 
    collector_id integer not null, 
    address varchar not null,
    port integer not null,
    protocol char(3) not null,
    last_update timestamptz not null,
    primary key (id), 
    foreign key (collector_id) references collectors(id),
    unique (collector_id, address, port, protocol)
);

create table ports_notifications_settings (
    collector_id integer, 
    show_for_opened boolean not null default false,  
    show_for_closed boolean not null default false,  
    primary key (collector_id),
    foreign key (collector_id) references collectors(id)
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
