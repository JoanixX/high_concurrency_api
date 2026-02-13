# Real-Time Betting Validation API (High Concurrency)

API de alto rendimiento desarrollada en **Rust** diseñada para la validación crítica de apuestas en eventos en vivo. El motor está optimizado para baja latencia y alta disponibilidad, capaz de procesar ráfagas masivas de transacciones concurrentes.

## 🚀 Enfoque Principal: Alta Concurrencia

Este proyecto no es solo una API CRUD; es un ejercicio de ingeniería de rendimiento que implementa:

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

## 📋 Arquitectura de Validación

La API implementa un patrón **en Capas (Layered)** donde las reglas de negocio (odds, límites de usuario, estado del partido) se validan en una capa de dominio desacoplada, permitiendo escalar el motor horizontalmente.

---

**Desarrollado para Escenarios de Misión Crítica | 2026**
