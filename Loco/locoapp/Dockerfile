# ─── Stage 1 : Build ───────────────────────────────────────────────────────────
FROM rust:1.88-slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 1. Copy manifests first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# 2. Fake src/main.rs to let cargo fetch & compile deps without our source code
RUN mkdir -p src/bin && \
    echo "fn main() {}" > src/bin/main.rs && \
    echo "fn main() {}" > src/bin/tool.rs && \
RUN cargo build --release 2>/dev/null; true

# 3. Now copy the real source and config
RUN rm -rf src
COPY src ./src
COPY config ./config
COPY migration ./migration

# 4. Build the real binary (cargo will only recompile our code, not deps)
RUN cargo build --release

# ─── Stage 2 : Runtime ─────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates libssl3 libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/locoapp-cli /app/locoapp

# Copy config folder
COPY --from=builder /app/config /app/config
COPY --from=builder /app/src/fixtures ./src/fixtures
# SQLite database will be stored here (mounted as a volume)
RUN mkdir -p /app/data

EXPOSE 5150

CMD ["/app/locoapp", "start"]
