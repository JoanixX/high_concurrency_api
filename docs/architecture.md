# Vista General de la Arquitectura del Sistema

## Estrategia de Monorepo

El proyecto está organizado como un monorepo para mantener la atomicidad entre la API y el Cliente.

## Capas del Sistema

- **Frontend (Lado del Cliente):** Aplicación Next.js 14 (App Router) con Zustand, TanStack Query y WebSocket en tiempo real.
- **Backend (Lado del Servidor):** API de Rust de alto rendimiento utilizando Actix-Web. Sigue la **Arquitectura Hexagonal** (Ports & Adapters).
- **Infraestructura:** Orquestación de contenedores a través de Docker y configuraciones listas para la nube.

## Arquitectura del Backend (Hexagonal / Ports & Adapters)

```
         ┌──────────────────────────────────┐
         │        ADAPTADORES PRIMARIOS     │
         │    (HTTP Handlers / CLI / Tests)  │
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
         │  RedisCacheAdapter              │
         │  Argon2Hasher                   │
         └─────────────────────────────────┘
```

1. **Dominio (Core):** Entidades puras, errores tipados y puertos (traits). Sin dependencias de framework.
2. **Casos de Uso (Application):** Orquestan la lógica de negocio usando solo los puertos del dominio.
3. **Adaptadores Primarios (Handlers):** Traducen HTTP → caso de uso → HTTP response. Cero SQL, cero crypto.
4. **Adaptadores Secundarios (Infrastructure):** Implementan los puertos con tecnologías concretas (Postgres, Redis, Argon2).

## Flujo de Datos

`Cliente → Handler (parsea DTO) → Use Case (valida dominio) → Port (trait) → Adapter (SQL/Redis) → PostgreSQL/Redis`

## Inyección de Dependencias

La composición se realiza en `lib.rs` (Composition Root):

- Los adaptadores secundarios se instancian con `Arc<dyn Port>`
- Los casos de uso reciben los puertos por constructor injection
- Actix-Web distribuye los casos de uso entre threads via `web::Data`

## Escalabilidad

- **Escalabilidad Horizontal:** La API es "stateless" y puede ser replicada.
- **Escalabilidad de DB:** PostgreSQL con pool de conexiones (SQLx) optimizado para alta concurrencia.
- **Testabilidad:** Los puertos permiten inyectar mocks en tests sin necesidad de infraestructura real.
