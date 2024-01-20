create table if not exists todos
(
    id         integer primary key autoincrement not null,
    title      text,
    completed  boolean  default false,
    created_at datetime default current_timestamp
);

