drop table if exists user_table;
drop table if exists user_table;

create table user_table(
    id serial primary key,
    user_name varchar(150) not null,
    password varchar(150) not null
);


insert into user_table (user_name, password) values ('wale', 1234), ('Znb', 5678);


