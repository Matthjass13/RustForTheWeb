FROM postgres:latest

ENV POSTGRES_PASSWORD = password 
ENV POSTGRES_USER = user1
ENV POSTGRES_DB = postgres_db

COPY init_script.sql /docker-entrypoint-initdb.d/

EXPOSE 5432