\c postgres_db;

-- code for making test users
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100),
    email VARCHAR(100)
);

INSERT INTO users (name, email) VALUES
    ('Benjamin', 'benjamin@example.com'),
    ('Hugo', 'hugo@example.com'),
    ('Matthias', 'matthias@example.com');

-- code for making giant test array 
CREATE TABLE sorting_test (
    id serial PRIMARY KEY,
    numbers integer[]
);

-- make the giant array
INSERT INTO sorting_test (numbers)
SELECT array_agg(floor(random() * 1000000)::int)
FROM generate_series(1, 100000);