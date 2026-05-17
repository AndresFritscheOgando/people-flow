# ── Build stage ──────────────────────────────────────────────────────────────
FROM rust:1.88-slim AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache dependencies separately
COPY Cargo.toml Cargo.lock ./
COPY migration/Cargo.toml ./migration/Cargo.toml

# Dummy source to cache deps
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN mkdir -p migration/src && echo "pub struct Migrator;" > migration/src/lib.rs
RUN cargo build --release --bin people-flow 2>/dev/null || true

# Build real source
COPY . .
RUN touch src/main.rs migration/src/lib.rs
RUN cargo build --release --bin people-flow

# ── Runtime stage ─────────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/people-flow ./people-flow

EXPOSE 3000

CMD ["./people-flow"]
