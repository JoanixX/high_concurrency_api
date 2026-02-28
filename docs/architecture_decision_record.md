# Registro de Decisiones Arquitectónicas (ADR) - API de Alta Concurrencia

## Contexto

Este servicio está diseñado para manejar un volumen masivo de peticiones concurrentes con latencia mínima. Se requiere una arquitectura que permita evolucionar el dominio de negocio (reglas de validación de apuestas, nuevos tipos de apuestas) sin acoplamiento a la infraestructura.

## Decisión: Arquitectura Hexagonal (Ports & Adapters)

Hemos optado por una **Arquitectura Hexagonal** que separa claramente el dominio de la infraestructura mediante traits (puertos) y sus implementaciones concretas (adaptadores).

### Razón Técnica

1. **Dominio Puro**: Las reglas de negocio (validaciones de odds, límites de apuesta) viven en `domain/` sin importar si la persistencia es Postgres, DynamoDB o un mock en memoria.
2. **Testabilidad**: Los puertos (`BetRepository`, `CachePort`, `PasswordHasher`) se pueden sustituir por mocks en tests unitarios sin necesidad de infraestructura real.
3. **Escalabilidad de Equipo**: Un desarrollador puede modificar la lógica de validación sin tocar SQL, y otro puede optimizar queries sin modificar reglas de negocio.
4. **Performance**: Usamos `Arc<dyn Trait>` para la inyección de dependencias. El costo del dispatch dinámico (~1-2 ns por call) es despreciable frente a la latencia de I/O (database: ~1-5ms, redis: ~0.5ms).

### Estructura de Capas

| Capa           | Directorio        | Responsabilidad                    | Dependencias                   |
| :------------- | :---------------- | :--------------------------------- | :----------------------------- |
| Domain         | `domain/`         | Entidades, errores, ports (traits) | Solo `serde`, `uuid`, `chrono` |
| Application    | `application/`    | Casos de uso                       | Solo `domain/`                 |
| Infrastructure | `infrastructure/` | Adaptadores secundarios            | `sqlx`, `redis`, `argon2`      |
| Handlers       | `handlers/`       | Adaptadores primarios (HTTP)       | `actix-web`, `application/`    |

### Regla de Dependencia

Las dependencias apuntan **siempre hacia adentro**: `handlers → application → domain ← infrastructure`.  
`infrastructure` implementa los traits definidos en `domain`, pero `domain` nunca importa `infrastructure`.

## Consecuencias

- **Positivas**: Dominio desacoplado, testeable con mocks, extensible sin romper capas existentes.
- **Negativas**: Más archivos y un paso de indirección adicional. Aceptable dado que el overhead de `dyn Trait` es insignificante frente al I/O.

## Decisiones Complementarias

- **`async-trait`**: Usado para definir puertos async de forma ergonómica.
- **`DomainError`**: Errores de dominio tipados que se mapean a HTTP en `errors/mod.rs` (el adaptador de errores).
- **Composition Root en `lib.rs`**: Toda la inyección de dependencias se realiza en un solo lugar.

---

_Documento actualizado tras migración a Arquitectura Hexagonal._
