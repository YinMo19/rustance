create table if not exists amount_record (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    amount INTEGER not null default 0,
    in_or_out BOOLEAN not null,
    append_msg TEXT not null,
    created_at DATETIME not null default CURRENT_TIMESTAMP,
    updated_at DATETIME not null default CURRENT_TIMESTAMP
);