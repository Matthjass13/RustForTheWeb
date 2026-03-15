# Frontend
FROM rust:1.88-slim AS frontend-builder

RUN apt-get update && apt-get install -y \
    curl pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

WORKDIR /app
COPY . .

WORKDIR /app/frontend
RUN trunk build --release

# Backend
FROM rust:1.88-slim AS backend-builder

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev musl-tools \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml ./backend/
COPY shared/Cargo.toml ./shared/
COPY frontend/Cargo.toml ./frontend/

RUN mkdir -p backend/src shared/src frontend/src \
    && echo "fn main() {}" > backend/src/main.rs \
    && echo "" > shared/src/lib.rs \
    && echo "fn main() {}" > frontend/src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl -p rust_rocket_app

RUN rm -rf backend/src shared/src frontend/src

COPY backend/src ./backend/src
COPY shared/src ./shared/src
COPY frontend/src ./frontend/src

RUN cargo build --target x86_64-unknown-linux-musl -p rust_rocket_app

# Runtime — image encore plus légère car binaire statique
FROM scratch AS runtime

COPY --from=backend-builder /app/target/x86_64-unknown-linux-musl/debug/rust_rocket_app /rust_rocket_app
COPY --from=frontend-builder /app/frontend/dist /static

EXPOSE 8000
CMD ["/rust_rocket_app"]