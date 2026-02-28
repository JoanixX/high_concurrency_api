# Real-Time Betting Validation API (High Concurrency)

API de alto rendimiento desarrollada en **Rust** con **Arquitectura Hexagonal** (Ports & Adapters), diseñada para la validación crítica de apuestas en eventos en vivo. El motor está optimizado para baja latencia y alta disponibilidad, capaz de procesar ráfagas masivas de transacciones concurrentes.

## 🚀 Enfoque Principal: Alta Concurrencia

Este proyecto no es solo una API CRUD; es un ejercicio de ingeniería de rendimiento que implementa:

- **Arquitectura Hexagonal**: Dominio puro desacoplado de la infraestructura mediante puertos (traits) y adaptadores.
- **Asincronía Extrema**: Construido sobre `Actix-Web` y `Tokio` para maximizar el uso de CPU.
- **Validación con Baja Latencia**: Procesamiento sub-10ms por ticket de apuesta.
- **Pooling Eficiente**: Conexiones a base de datos (Postgres via SQLx) y caché (Redis) optimizadas.
- **Observabilidad**: Tracing estructurado para identificar cuellos de botella en milisegundos.

## 🛠️ Stack Tecnológico

- **Backend**: Rust (Actix-Web, SQLx, Redis-RS).
- **Caché**: Redis Alpine (Capa de validación rápida).
- **Persistencia**: PostgreSQL.
- **Infraestructura**: Docker Compose.
- **Testing de Carga**: k6 (Grafana).
- **Frontend**: Next.js 14 (Dashboard de métricas y simulador en tiempo real).

## 🏛️ Arquitectura Hexagonal

```
backend/src/
├── domain/                   ← CORE: cero deps de framework
│   ├── models.rs             (entidades: BetTicket, User, BetStatus)
│   ├── errors.rs             (errores de dominio tipados)
│   └── ports.rs              (traits: BetRepository, UserRepository, CachePort, PasswordHasher)
├── application/              ← CASOS DE USO: orquestan lógica via ports
│   ├── place_bet.rs          (validar + persistir apuesta)
│   ├── register_user.rs      (hashear + persistir usuario)
│   └── login_user.rs         (verificar credenciales)
├── infrastructure/           ← ADAPTADORES SECUNDARIOS (driven)
│   ├── persistence/          (Postgres: implementa BetRepository, UserRepository)
│   ├── cache/                (Redis/Upstash: implementa CachePort)
│   ├── security/             (Argon2: implementa PasswordHasher)
│   └── database.rs           (pool de conexiones)
├── handlers/                 ← ADAPTADORES PRIMARIOS (driving)
│   ├── dto.rs                (request/response DTOs HTTP)
│   ├── betting.rs            (HTTP → PlaceBetUseCase → HTTP)
│   └── auth.rs               (HTTP → RegisterUser/LoginUser → HTTP)
├── errors/                   ← mapeo DomainError → HTTP
├── config/                   ← configuración multi-entorno
├── telemetry/                ← tracing estructurado (Bunyan JSON)
└── lib.rs                    ← composition root (DI)
```

### Flujo de una Apuesta

```
HTTP POST /bets
  → handlers/betting.rs (parsea DTO, traduce a BetTicket)
    → application/place_bet.rs (valida reglas de dominio)
      → domain/ports::BetRepository.save() (trait)
        → infrastructure/persistence/bet_repository.rs (INSERT SQL)
      → domain/ports::CachePort.set() (trait)
        → infrastructure/cache/ (Redis SET)
    ← PlaceBetResult
  ← HttpResponse::Ok(PlaceBetResponse)
```

## 📊 Simulación & Pruebas de Estrés

### 1. Levantar Infraestructura

```bash
cd infrastructure
docker-compose up -d
```

### 2. Ejecutar el Motor (Backend)

```bash
cd backend
sqlx migrate run
cargo run --release
```

### 3. Simulador UI (Frontend)

El simulador permite enviar apuestas manualmente y observar la latencia en tiempo real.

```bash
cd frontend
npm install
npm run dev
```

### 🚀 4. Load Testing con k6

Para validar que el sistema soporta miles de peticiones por segundo:

```bash
# Requiere k6 instalado localmente
cd backend/k6
k6 run load_test.js
```

---

**Desarrollado para Escenarios de Misión Crítica | 2026**
