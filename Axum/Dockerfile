# --- Stage 1: Builder ---
FROM rust:1.88-slim AS builder

WORKDIR /app

# Pre-build dependencies for caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {} " > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

# Copy actual source and static files
COPY src ./src
COPY static ./static

# Build the real binary
RUN touch src/main.rs && cargo build --release

# --- Stage 2: Runtime ---
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 1. Copy the binary from the builder stage
COPY --from=builder /app/target/release/AxumSite .

# 2. Copy the static folder from the builder stage so Axum can find it
COPY --from=builder /app/static ./static

EXPOSE 3000
CMD ["./AxumSite"]