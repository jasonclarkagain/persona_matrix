# --- STAGE 1: Build ---
FROM rust:1.75-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# --- STAGE 2: Runtime ---
FROM debian:bookworm-slim
WORKDIR /app

# Install system dependencies for the Shield and Engine
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/persona_engine .

# Port for the Control Panel
EXPOSE 3000

CMD ["./persona_engine"]
