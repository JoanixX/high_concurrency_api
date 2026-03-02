# Backend — API de Alta Concurrencia (Rust)

API de alto rendimiento para la validación de apuestas en eventos en vivo, construida con **Rust** y **Arquitectura Hexagonal** (Ports & Adapters). Optimizada para baja latencia y alta disponibilidad bajo ráfagas masivas de concurrencia.

## 🛠️ Stack

| tecnología        | propósito                                    |
| :---------------- | :------------------------------------------- |
| Rust              | lenguaje principal                           |
| Actix-Web 4       | framework web asíncrono                      |
| Tokio             | runtime asíncrono (multi-thread)             |
| SQLx 0.7          | acceso a PostgreSQL (compile-time checked)   |
| PostgreSQL (Neon) | persistencia principal (serverless en prod)  |
| Redis / Upstash   | caché de validación rápida                   |
| Argon2            | hashing de contraseñas                       |
| Tracing + Bunyan  | observabilidad y logging estructurado (JSON) |
| Actix-Web-Prom    | métricas Prometheus                          |
| Docker            | contenedorización                            |
| k6                | testing de carga                             |

## 🏛️ Arquitectura Hexagonal (Ports & Adapters)

el backend sigue una arquitectura hexagonal estricta. el dominio puro está desacoplado de la infraestructura mediante traits (puertos) y sus implementaciones concretas (adaptadores).

### capas

| capa           | directorio            | responsabilidad                            | puede depender de                       |
| :------------- | :-------------------- | :----------------------------------------- | :-------------------------------------- |
| domain         | `src/domain/`         | entidades, errores tipados, ports (traits) | solo crates puros (serde, uuid, chrono) |
| application    | `src/application/`    | casos de uso (orquestan lógica via ports)  | solo `domain/`                          |
| infrastructure | `src/infrastructure/` | adaptadores secundarios (driven)           | `domain/` + crates de infra             |
| handlers       | `src/handlers/`       | adaptadores primarios (driving, HTTP)      | `application/`                          |
| errors         | `src/errors/`         | mapeo DomainError → HTTP response          | `domain/` + `actix-web`                 |

### regla de dependencia

las dependencias apuntan siempre hacia adentro:

```
handlers → application → domain ← infrastructure
```

- `domain/` nunca importa `infrastructure/`
- `handlers/` nunca acceden directamente a la base de datos
- toda la inyección de dependencias se resuelve en `lib.rs` (composition root)

### diagrama

```
         ┌──────────────────────────────────┐
         │        ADAPTADORES PRIMARIOS     │
         │    (HTTP Handlers / Tests)        │
         │  handlers/betting.rs, auth.rs     │
         └──────────┬───────────────────────┘
                    │ llama a
         ┌──────────▼───────────────────────┐
         │       CASOS DE USO              │
         │    (application/)               │
         │  PlaceBetUseCase                │
         │  RegisterUserUseCase            │
         │  LoginUserUseCase               │
         └──────────┬───────────────────────┘
                    │ depende de (via traits)
         ┌──────────▼───────────────────────┐
         │       DOMINIO (CORE)            │
         │    (domain/)                    │
         │  BetTicket, User, BetStatus     │
         │  DomainError                    │
         │  Ports: BetRepository,          │
         │         UserRepository,         │
         │         CachePort,              │
         │         PasswordHasher          │
         └──────────┬───────────────────────┘
                    │ implementado por
         ┌──────────▼───────────────────────┐
         │     ADAPTADORES SECUNDARIOS     │
         │    (infrastructure/)            │
         │  PostgresBetRepository          │
         │  PostgresUserRepository         │
         │  RedisCacheAdapter / Upstash    │
         │  Argon2Hasher                   │
         └─────────────────────────────────┘
```

### inyección de dependencias (composition root)

la composición se realiza en `lib.rs`:

- los adaptadores secundarios se instancian con `Arc<dyn Port>`
- los casos de uso reciben los puertos por constructor injection
- Actix-Web distribuye los casos de uso entre threads via `web::Data`

## 📂 Estructura

```
backend/
├── src/
│   ├── domain/                 ← core: cero deps de framework
│   │   ├── models.rs           (entidades: BetTicket, User, BetStatus)
│   │   ├── errors.rs           (errores de dominio tipados con thiserror)
│   │   └── ports.rs            (traits: BetRepository, UserRepository, CachePort, PasswordHasher)
│   ├── application/            ← casos de uso: orquestan lógica via ports
│   │   ├── place_bet.rs        (validar + persistir apuesta)
│   │   ├── register_user.rs    (hashear + persistir usuario)
│   │   └── login_user.rs       (verificar credenciales)
│   ├── infrastructure/         ← adaptadores secundarios (driven)
│   │   ├── persistence/        (Postgres: PostgresBetRepository, PostgresUserRepository)
│   │   ├── cache/              (Redis/Upstash: RedisCacheAdapter)
│   │   ├── security/           (Argon2Hasher)
│   │   └── database.rs         (pool de conexiones)
│   ├── handlers/               ← adaptadores primarios (driving)
│   │   ├── dto.rs              (request/response DTOs HTTP)
│   │   ├── betting.rs          (HTTP → PlaceBetUseCase → HTTP)
│   │   ├── auth.rs             (HTTP → RegisterUser/LoginUser → HTTP)
│   │   └── health_check.rs     (endpoint de salud)
│   ├── errors/                 ← mapeo DomainError → HttpResponse
│   ├── config/                 ← configuración multi-entorno (YAML + env vars)
│   ├── middlewares/            ← middlewares personalizados
│   ├── routes/                 ← definición de rutas
│   ├── telemetry/              ← tracing estructurado (Bunyan JSON)
│   ├── lib.rs                  ← composition root (DI)
│   └── main.rs                 ← punto de entrada
├── configuration/
│   ├── base.yaml               (config local por defecto)
│   └── production.yaml         (overrides para producción)
├── migrations/                 ← migraciones SQL (SQLx)
├── k6/                         ← scripts de load testing
├── tests/                      ← tests de integración
├── Cargo.toml
└── Dockerfile
```

## 🔄 Flujo de una Apuesta

```
HTTP POST /bets
  → handlers/betting.rs (parsea DTO, traduce a BetTicket)
    → application/place_bet.rs (valida reglas de dominio)
      → domain/ports::BetRepository.save() (trait)
        → infrastructure/persistence/bet_repository.rs (INSERT SQL)
      → domain/ports::CachePort.set() (trait)
        → infrastructure/cache/ (Redis/Upstash SET)
    ← PlaceBetResult
  ← HttpResponse::Ok(PlaceBetResponse)
```

## 🚀 Ejecución Local

```bash
# 1. levantar postgres y redis con docker
cd infrastructure
docker-compose up -d

# 2. ejecutar migraciones
cd backend
sqlx migrate run

# 3. iniciar el servidor
cargo run --release
```

la API estará disponible en `http://localhost:8000`.

## 🧪 Load Testing con k6

```bash
# requiere k6 instalado localmente
cd backend/k6
k6 run load_test.js
```

## ⚙️ Variables de Entorno

las variables de entorno se manejan con archivos `.env` y configuración YAML en `configuration/`.

- **local**: `.env` + `configuration/base.yaml`
- **producción**: `.env.production` + `configuration/production.yaml`

ver `.env.example` para la plantilla con todas las variables necesarias y cómo obtenerlas.

## 🏗️ Decisiones Arquitectónicas

### ¿por qué arquitectura hexagonal?

1. **dominio puro**: las reglas de negocio (validaciones de odds, límites de apuesta) viven en `domain/` sin importar si la persistencia es Postgres, DynamoDB o un mock en memoria.
2. **testabilidad**: los puertos (`BetRepository`, `CachePort`, `PasswordHasher`) se pueden sustituir por mocks en tests unitarios sin infraestructura real.
3. **escalabilidad de equipo**: un dev puede modificar lógica de validación sin tocar SQL, y otro puede optimizar queries sin modificar reglas de negocio.
4. **performance**: usamos `Arc<dyn Trait>` para DI. el costo del dispatch dinámico (~1-2 ns por call) es despreciable frente a la latencia de I/O (database: ~1-5ms, redis: ~0.5ms).

### consecuencias

- **positivas**: dominio desacoplado, testeable con mocks, extensible sin romper capas existentes.
- **negativas**: más archivos y un paso de indirección adicional. aceptable dado que el overhead de `dyn Trait` es insignificante frente al I/O.

### decisiones complementarias

- **`async-trait`**: usado para definir puertos async de forma ergonómica.
- **`DomainError`**: errores de dominio tipados con `thiserror`, mapeados a HTTP en `errors/mod.rs`.
- **composition root en `lib.rs`**: toda la inyección de dependencias centralizada en un solo lugar.

## 📈 Escalabilidad

- **horizontal**: la API es stateless y puede replicarse sin conflictos.
- **base de datos**: PostgreSQL con pool de conexiones (SQLx) optimizado para alta concurrencia.
- **testabilidad**: los puertos permiten inyectar mocks en tests sin necesidad de infraestructura real.

---

**Motor de Validación de Alta Concurrencia | Rust + Actix-Web | 2026**
