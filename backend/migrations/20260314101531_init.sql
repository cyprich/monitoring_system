-- Add migration script here

-- drop table notifications;
-- drop table notification_severity;
-- drop table users;
-- drop table metrics;
-- drop table metric_type;
-- drop table hosts;
-- drop table _sqlx_migrations;

---------- CREATE TABLES ----------

create table hosts (
    id serial,
    name varchar not null,
    system_name varchar,
    host_name varchar,
    kernel_version varchar,
    total_memory integer,
    cpu_count integer,
    primary key (id)
);

create table metric_type (
    id serial,
    name varchar,
    description varchar,
    primary key (id)
);

create table metrics (
    timestamp timestamp,
    value double precision,
    -- TODO make this host name
    host_id integer,
    type_id integer,
    component_id integer default 0,  -- used when we have multiple disks/network interfaces
    primary key (timestamp, host_id, value, type_id, component_id),
    foreign key (host_id) references hosts(id),
    foreign key (type_id) references metric_type(id)
);

create table users (
    id serial,
    name varchar,
    password_hash varchar,
    primary key (id)
);

create table notification_severity (
    id serial,
    severity varchar,
    primary key (id)
);

create table notifications (
    id serial,
    host_id integer,
    severity integer,
    description varchar,
    timestamp timestamp,
    viewed boolean,
    primary key (id),
    foreign key (host_id) references hosts(id),
    foreign key (severity) references notification_severity(id)
);

---------- INSERT REQUIRED DATA ----------

insert into metric_type (name)
values
    ('used_mem'),
    ('cpu_usage');

-- temp
select * from hosts;

insert into hosts (name, system_name, host_name, kernel_version, total_memory)
values ('pokus name', 'pokus systemname', 'pokus hostname', '0', 16_000);
