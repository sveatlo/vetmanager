CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR UNIQUE NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    password VARCHAR NOT NULL,

    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL
);
