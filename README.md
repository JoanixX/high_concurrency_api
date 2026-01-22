# High Concurrency API Template (Rust)

## Overview

Este proyecto es una base arquitectónica profesional para construir servicios backend de alto rendimiento en Rust. Diseñado para soportar cargas de alta concurrencia, servir como punto de partida para APIs productivas y demostrar mejores prácticas en Backend Engineering.

**NO es un proyecto educativo básico.** Está configurado con prácticas de producción desde el día 1.

## Architecture

El proyecto sigue una arquitectura estratificada (Layered Architecture) para garantizar la separación de responsabilidades y la testabilidad.

### Estructura de Carpetas

- `src/main.rs`: **Entry Point**. Inicializa la telemetría, carga la configuración y arranca el servidor.
- `src/lib.rs`: **Application Setup**. Configura el `App` de Actix-Web, inyecta dependencias (DB Pool) y define el `startup` del servidor.
- `src/config/`: **Configuration**. Carga configuración desde archivos YAML y variables de entorno. Usa tipos seguros para evitar errores en runtime.
- `src/domain/`: **Domain Logic**. Contiene la lógica de negocio pura, agnóstica de la base de datos y del servidor web.
- `src/handlers/`: **HTTP Handlers**. Controladores que reciben requests HTTP, llaman a la lógica de dominio y retornan respuestas HTTP. Son la única capa acoplada al framework web.
- `src/routes/`: **Routing**. Define las rutas y las asocia a los handlers.
- `src/db/`: **Database**. Abstracciones y configuración del pool de conexiones a la base de datos.
- `src/middlewares/`: **Middlewares**. Lógica transversal como autenticación, rate limiting, etc.
- `src/telemetry/`: **Observability**. Configuración de `tracing` para logs estructurados distribuidos.
- `src/errors/`: **Error Handling**. Tipos de errores centralizados y mapeo a respuestas HTTP consistentes.

### Decisiones Técnicas

1.  **Rust & Actix-Web**: Elegidos por su performance "bare-metal", seguridad de memoria y ecosistema asíncrono maduro (`tokio`). Actix-Web domina en benchmarks de throughput.
2.  **SQLx**: Pure Rust SQL mapper. Provee verificación de queries en tiempo de compilación (compile-time checked queries) y manejo asíncrono nativo.
3.  **Observabilidad (Tracing)**: En lugar de logs de texto plano, usamos `tracing` con formato Bunyan (JSON) para permitir ingestión estructurada en ELK/Datadog. Esto es obligatorio para debugging en sistemas distribuidos.
4.  **Configuración Jerárquica**: Base YAML + Overrides por Entorno + Variables de Entorno. Permite gestión flexible de secretos y configs por deploy.
5.  **Docker Multi-stage Build**: Imagen final optimizada (distroless/slim) para reducir superficie de ataque y tamaño de imagen.

## Performance Tuning

### Database Pool

Configurado para alta concurrencia en `src/db/mod.rs`:

- `max_connections`: 100 (ajustar según límitaciones de la instancia DB)
- `max_lifetime`: 30 minutos (evita conexiones stale)
- `acquire_timeout`: 2 segundos (fail-fast si la DB está saturada)

### Load Testing

Se incluye configuración para **k6** en `k6/load_test.js`.
Objetivo: Validar que el sistema maneja picos de tráfico manteniendo latencias p95/p99 estables.

## Getting Started

### Prerrequisitos

- Rust & Cargo
- Docker & Docker Compose
- psql (PostgreSQL client)

### Setup Local

1.  **Levantar infrastructura**:

    ```bash
    docker-compose up -d
    ```

2.  **Preparar la Base de Datos**:

    ```bash
    # (Opcional si usas sqlx-cli)
    cargo install sqlx-cli
    sqlx database create
    sqlx migrate run
    ```

    _Nota: El script `scripts/init_db.sh` automatiza esto._

3.  **Ejecutar la API**:
    ```bash
    cargo run
    ```

### Ejecutar Tests

```bash
cargo test
```

### Ejecutar Load Tests (k6)

```bash
# Instalar k6 si no está instalado
k6 run k6/load_test.js
```

## Estado del Proyecto

Actualmente en **Fase de Inicialización**.

- [x] Estructura base completa
- [x] Dockerización optimizada
- [x] Configuración de logging/tracing
- [x] Conexión a BD resiliente
- [ ] Implementación de lógica de negocio específica (Pendiente de definición)
- [ ] Endpoints transaccionales

## Contribución

Todo cambio debe pasar tests y `cargo clippy`.
Las migraciones de base de datos deben ser inmutables.
