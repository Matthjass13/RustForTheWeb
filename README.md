# Rust For The Web

Rust is a general-purpose programming language known for its focus on performance, type safety, concurrency, and memory safety. The goal of this project is to determine if Rust can replace Ruby on Rails, Python+Django or Spring Boot to write web applications. Keywords: Rust, WebAssembly, web development, OWASP Top 10

# How to setup the environment

1. Install Docker, and launch Docker Daemon
2. Create a .env file in the project root for with the following environment variables:
```
POSTGRES_USER=user1
POSTGRES_PASSWORD=password
POSTGRES_DB=postgres_db
```

3. In project root run command ```docker compose up```

4. After containers are built, Axum website is served at localhost:3001, Django at localhost:8000, Rocket at localhost:8001


# How to run our examples

See documentation in the wiki section

# Project management

We managed the project with the Scrum methodology, using Taiga.
Here is the link of the project : https://tree.taiga.io/project/matthjass-rust-for-the-web/timeline
