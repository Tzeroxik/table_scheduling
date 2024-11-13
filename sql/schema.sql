begin transaction;

-- begin table
create table if not exists server_configuration
(
);

-- fields
alter table if exists server_configuration
    add column if not exists id bigint not null default 1
        constraint one_row check (id = 1);

alter table if exists server_configuration
    add column if not exists name text not null default '';

alter table if exists server_configuration
    add column if not exists port text not null default '';

alter table if exists server_configuration
    add column if not exists https boolean not null default false;

-- contraints
alter table if exists server_configuration
    drop constraint if exists server_configuration_pk;

alter table if exists server_configuration
    add constraint server_configuration_pk primary key (id);
-- end table

commit;