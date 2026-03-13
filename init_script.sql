\c postgres_db;

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100),
    email VARCHAR(100)
);

INSERT INTO users (name, email) VALUES
    ('Benjamin', 'benjamin@example.com'),
    ('Hugo', 'hugo@example.com'),
    ('Matthias', 'matthias@example.com');