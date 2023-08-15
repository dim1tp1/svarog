use lbm;

-- Due to existence of foreign keys,
-- change the order of "drop table" statements with caution.
drop table if exists mpc_session_message;
drop table if exists mpc_session_attendance;
drop table if exists mpc_session_params;
drop table if exists mpc_session;
drop table if exists mpc_key_signature;
drop table if exists mpc_key_shard;
drop table if exists mpc_key;

create table mpc_key
(
    -- id of root private key.
    key_id    char(32) charset utf8mb4  not null,
    -- id of root public key
    root_xpub char(128) charset utf8mb4 not null,

    th        int unsigned              not null,
    primary key (key_id),
    constraint mpc_key_pk unique (root_xpub)
) row_format = DYNAMIC;

create table mpc_key_shard
(
    key_id   char(32) charset utf8mb4 not null,
    owner_id char(32) charset utf8mb4 not null,
    primary key (key_id, owner_id),
    constraint fk_key_id_of_shard
        foreign key (key_id) references mpc_key (key_id)
            on update cascade on delete cascade
) row_format = DYNAMIC;

create table mpc_key_signature
(
    tx_hash binary(64) not null primary key,
    key_id char(32) charset utf8mb4 not null,
    create_time_utc timestamp(3) not null,
    derv_path varchar(255) charset utf8mb4 not null,
    r binary(64) not null,
    s binary(64) not null,
    v tinyint(1) not null,

    -- JSON array of owner_id
    participants JSON not null,

    constraint mpc_signature_pk2
        unique (key_id, create_time_utc),
    constraint fk_mpc_signature
        foreign key (key_id) references mpc_key (key_id)
            on update cascade on delete cascade
) row_format = DYNAMIC;

create table mpc_session
(
    session_id   char(32)             not null comment 'UUID'
        primary key,
    expire_at    bigint                  not null comment 'unixepoch',
    session_type enum ('keygen', 'sign') not null
) row_format = DYNAMIC;

create table mpc_session_params
(
    session_id char(32)    not null
        primary key,
    threshold   int unsigned   null,
    n_keygen    int unsigned   null,
    key_id      char(32)       null,
    derv_path   varchar(255)   null,
    tx_hash     binary(64)  null,
    constraint fk_params
        foreign key (session_id) references mpc_session (session_id)
            on update cascade on delete cascade
) row_format = DYNAMIC;

create table mpc_session_attendance
(
    session_id char(32)                              not null,
    party_id   int unsigned                              null,
    owner_id   char(32)                              not null,
    opinion    enum ('unknown', 'agree', 'disagree') not null,
    primary key (session_id, owner_id),
    constraint session_id
        unique (session_id, party_id),
    constraint fk_attendance_session_id
        foreign key (session_id) references mpc_session (session_id)
            on update cascade on delete cascade
) row_format = DYNAMIC;

create table mpc_session_message
(
    session_id  char(32)     not null,
    round       int unsigned not null,
    party_from  int unsigned not null,
    party_to    int unsigned not null,
    message     blob         not null,
    hash        char(8)     not null,
    is_finished tinyint(1)   not null default 0,

    primary key (session_id, party_from, party_to, round),
    constraint fk_party_from
        foreign key (session_id, party_from) references mpc_session_attendance (session_id, party_id)
            on update cascade on delete cascade,
    constraint fk_party_to
        foreign key (session_id, party_to) references mpc_session_attendance (session_id, party_id)
            on update cascade on delete cascade
) row_format = DYNAMIC;
