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
