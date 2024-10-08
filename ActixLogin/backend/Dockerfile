# Builder stage
FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef AS planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
# Build our project
RUN cargo build --release --bin backend

# Runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app
# Install OpenSSL
# Install ca-certificates
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/backend backend
# We need the settings file at runtime!
COPY settings settings
COPY templates templates
ENV APP_ENVIRONMENT=production
ENV APP_DEBUG=false
ENTRYPOINT ["./backend"]