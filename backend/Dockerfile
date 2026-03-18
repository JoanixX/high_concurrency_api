# Etapa 1: Planner
# genera el recipe.json con el manifiesto de dependencias
FROM lukemathwalker/cargo-chef:latest-rust-1.84.0-bookworm AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Etapa 2: Builder
# compila dependencias cacheadas y luego el codigo fuente
FROM lukemathwalker/cargo-chef:latest-rust-1.84.0-bookworm AS builder
WORKDIR /app

# compilamos solo las dependencias (capa cacheada si no cambian)
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# copiamos el codigo fuente real y compilamos
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Etapa 3: Runtime
# imagen minima solo con el binario
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# certificados raiz para llamadas https salientes
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# secops: usuario sin privilegios
RUN groupadd --system appgroup && useradd --system --gid appgroup appuser

# copiamos el binario y la configuracion
COPY --from=builder /app/target/release/high_concurrency_api high_concurrency_api
COPY configuration configuration
COPY migrations migrations

# permisos y usuario
RUN chown -R appuser:appgroup /app
USER appuser

EXPOSE 8000

ENV APP_ENVIRONMENT=production

CMD ["./high_concurrency_api"]