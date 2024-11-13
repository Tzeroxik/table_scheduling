create table if not exists server_configuration
(
    id     bigint default 1 not null
        constraint server_configuration_pk
            primary key,
    name   text             not null,
    active boolean          not null,
    constraint one_row
        check (id = 1)
);