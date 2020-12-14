create TABLE Users (
    id serial PRIMARY KEY,
    name VARCHAR(30) NOT NULL UNIQUE,
    password VARCHAR(30) NOT NULL
);

create TABLE Teachers (
    id serial PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);

create TABLE Subjects (
    id serial PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);

create TABLE Reviews (
    id serial PRIMARY KEY,
    teacher VARCHAR(255) NOT NULL REFERENCES Teachers (name) ON DELETE CASCADE,
    subject VARCHAR(255) NOT NULL REFERENCES Subjects (name) ON DELETE CASCADE,
    owner VARCHAR(30) NOT NULL REFERENCES Users (name) ON DELETE CASCADE,
    text TEXT,
    mark SMALLINT NOT NULL
);


/// Тестовые данные

insert into Teachers (name) values ('Teacher1'), ('Teacher2');
insert into Subjects (name) values ('Teor'), ('AK'), ('Ver');
insert into TeachersSubjects (teacher, subject) values ('Teacher1', 'Teor'), ('Teacher1', 'Ver'), ('Teacher2', 'AK');
insert into Users (name, password) values ('Kirill', '12345');

insert into Reviews (teacherSubjectId, owner, text, mark) 
    values 
    (
        1,
        'Kirill',
        'Это плохой предмет и препод мне не нравиться!',
        3
    );

///

select teacher from TeachersSubjects where id = 1;
select subject from TeachersSubjects where id = 1;
