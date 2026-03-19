# Frontend — Dashboard de Apuestas en Tiempo Real

Interfaz web construida con **Next.js 14** (App Router) que consume la API de alta concurrencia en Rust. Muestra odds en tiempo real, permite colocar apuestas y visualizar la actividad del motor de validación.

## 🛠️ Stack Tecnológico

- **Framework**: Next.js 14 (App Router, Server Components + Client Components)
- **Lenguaje**: TypeScript Estricto (alineado con Dominio de Rust)
- **Estado Global**: Zustand (stores modulares O(1) para alta frecuencia: `odds-store`, `selections-store`, `auth-store`)
- **Data Fetching (Snapshot)**: TanStack Query (React Query) para llamadas REST
- **Tiempo Real (Delta)**: WebSocket nativo con reconexión exponencial y sync de estado
- **Feedback Visual UI**: Toaster vía Sonner y mutación DOM directa (Zero React Renders) para Odds y Balance
- **Matemática Financiera**: Manejo de dinero en **centavos enteros (integers)** para evitar errores de precisión IEEE 754
- **Estilos**: Tailwind CSS + shadcn/ui
- **HTTP Client**: Axios con interceptores de auth y TanStack Query para persistencia local
- **i18n**: Formateo de moneda estandarizado `en-US` / `USD` via `Intl.NumberFormat`

## 📂 Estructura

```
src/
├── app/                  # Rutas de Next.js (App Router)
│   ├── (auth)/           # Rutas de autenticación (login, registro)
│   ├── dashboard/        # Dashboard interactivo Snapshot+Delta
│   │   ├── history/      # Historial financiero de apuestas (TanStack Table)
│   ├── layout.tsx        # Layout raíz con providers (QueryClient, Toaster)
│   └── page.tsx          # Página de inicio
├── components/           # Componentes reutilizables
│   ├── ui/               # Primitivos de shadcn/ui (Button, Table, etc.)
│   ├── balance-indicator.tsx # Componente Zero-Render para saldo vivo
│   ├── betting-slip.tsx  # Boleta de apuestas (Single Bet "Quick Bet")
│   ├── live-odds-row.tsx # Zero-Render Flash Fila de odds en vivo
│   └── live-odds-table.tsx
├── hooks/                # Custom hooks reactivos
│   ├── use-live-odds.ts  # Selector granular O(1) de odds por partido
│   ├── use-place-bet.ts  # Mutación REST + pending en Zustand + Toasts
│   ├── use-socket.ts     # Delta WS → stores y manejador de reconexión (refetch)
│   └── use-active-matches.ts # Fetch REST del Snapshot inicial
├── lib/                  # Utilidades y clientes
│   ├── api.ts            # Cliente Axios para endpoints y mock inicial
│   ├── socket.ts         # Cliente WebSocket con reconnect + heartbeat
│   └── constants.ts      # Ubiquitous Language (espejo the Enums de Rust)
├── store/                # Stores de Zustand centralizados (Modular O(1))
│   ├── auth-store.ts     # Sesiones, Token y User context
│   ├── balance-store.ts  # Saldo en centavos con lógica de Optimistic Update
│   ├── betting-store.ts  # Historial y Pending bets (Frontend side)
│   ├── odds-store.ts     # Record local de odds vivo
│   └── selections-store.ts # Foco en la selección activa para Slip
└── types/                # Tipos TypeScript Strict
    └── domain.ts         # Modelos importados mediante constants
```

## 🏎️ Arquitectura de Alta Frecuencia (Real-Time UI)

1. **Patrón Snapshot + Delta**: La aplicación nunca queda en blanco de forma pasiva. Con `TanStack Query` se levanta un _Snapshot_ a través de REST (por ahora vía `fetchActiveMatches`), y el WebSocket se conecta luego para emitir parches (_Deltas_) directos al Store para el flujo vivo asíncrono. En caídas the WS se vuelve a disparar el snapshot.
2. **Zero React Renders (Flash Highlights)**: El resalto de cuotas y el indicador de saldo (`BalanceIndicator`) prescinden del ciclo de vida de React para efectos visuales. Empleamos manipulación DOM subyacente (`useRef` + `.classList.add`) unida a un timeout con protección _anti memory leaks_, evitando re-renders globales innecesarios en updates de alta frecuencia (>10 updates/sec).
3. **Optimistic Updates & Paridad en Centavos**: Todas las operaciones matemáticas en el cliente se realizan sobre enteros (centavos). La UI asume el éxito de la apuesta inmediatamente restando el saldo localmente, y realiza un `rollback` automático si el backend detecta una violación de saldo o cuota, garantizando consistencia total con el Dominio de Rust.
4. **Feedback Ágil (Sonner)**: Validaciones asíncronas envían eventos de Toast visuales reportando el éxito (`Accepted`) e informando la latencia E2E.

## 🚀 Ejecución Local

```bash
# Instalar dependencias
npm install

# Iniciar servidor de desarrollo
npm run dev
```

Se abre en `http://localhost:3000`. Nota: el estado en vivo real ahora depende de tener la API Rust (`backend/`) corriendo (o al menos un Snapshot resolviendo en API mock). La dependencia estática `mock-socket.ts` ha sido removida para alinear a producción.

## 🏗️ Build de Producción

```bash
npm run build
npm start
```

El build utiliza `output: 'standalone'` para generar un contenedor autocontenido compatible con Docker.

## ⚙️ Variables de Entorno

Crear un archivo `.env` en la carpeta `frontend/`:

```env
NEXT_PUBLIC_API_URL=http://localhost:8000
NEXT_PUBLIC_WS_URL=ws://localhost:8000
```
