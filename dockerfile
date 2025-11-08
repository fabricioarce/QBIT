# Build stage
FROM rust:1.91-bullseye as builder

WORKDIR /usr/src/app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code and sqlx data
COPY src ./src
COPY .sqlx ./.sqlx

# Install dependencies needed for building
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set timezone to Costa Rica
ENV TZ=America/Costa_Rica
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1000 botuser && chown -R botuser:botuser /app

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/bot-olim-p-code /app/bot

# Switch to non-root user
USER botuser

# Set environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Run the bot
CMD ["/app/bot"]
