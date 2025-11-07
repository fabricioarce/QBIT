# Build stage
FROM rust:1.75-slim as builder

# Instalar dependencias del sistema
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Crear directorio de trabajo
WORKDIR /app

# Copiar solo Cargo.toml primero
COPY Cargo.toml ./

# Crear un proyecto dummy para compilar dependencias
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copiar el código fuente real
COPY src ./src

# Compilar la aplicación (las dependencias ya están en cache)
RUN touch src/main.rs && \
    cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Instalar dependencias de runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Crear usuario no-root
RUN useradd -m -u 1000 botuser

# Crear directorio de trabajo
WORKDIR /app

# Copiar el binario compilado desde el build stage
COPY --from=builder /app/target/release/bot-olim-p-code .

# Cambiar permisos
RUN chown -R botuser:botuser /app

# Cambiar a usuario no-root
USER botuser

# Comando para ejecutar el bot
CMD ["./bot-olim-p-code"]