-- begin table
create table if not exists server_configuration
(
    id    bigint  not null default 1
        constraint one_row check (id = 1)
        constraint server_configuration_pk primary key,
    name  text    not null default '',
    port  text    not null default '',
    https boolean not null default false
);