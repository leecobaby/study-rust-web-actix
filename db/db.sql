drop table if exists course;

create table course
(
    id serial primary key,
    teacher_id int not null,
    name varchar(255) not null,
    time timestamp default now()
);

insert into course (id, teacher_id, name, time) values (1, 1, 'Math', '2022-01-01 00:00:00');
insert into course (id, teacher_id, name, time) values (2, 1, 'English', '2022-01-02 00:00:00');