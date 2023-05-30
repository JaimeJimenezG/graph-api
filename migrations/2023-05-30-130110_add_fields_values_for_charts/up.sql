-- Your SQL goes here
ALTER TABLE charts DROP user_id;

create table chart_user (
    id serial primary key,
    chart_id int not null,
    user_id int not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    constraint fk_chart_user_chart_id foreign key (chart_id) references charts(id),
    constraint fk_chart_user_user_id foreign key (user_id) references users(id)
);

create table fields (
    id serial primary key,
    name varchar(255) not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);

create table chart_field (
    id serial primary key,
    chart_id int not null,
    field_id int not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    constraint fk_chart_field_chart_id foreign key (chart_id) references charts(id),
    constraint fk_chart_field_field_id foreign key (field_id) references fields(id)
);

create table values (
    id serial primary key,
    field_id int not null,
    value int not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    constraint fk_value_field_id foreign key (field_id) references fields(id)
);