export const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8000';
export const WS_BASE_URL = process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:8000';
export const HEALTH_CHECK_INTERVAL_MS = 30_000;
export const ACTIVITY_LOG_MAX_ENTRIES = 8;

// endpoints del backend rust
export const ENDPOINTS = {
  HEALTH_CHECK: '/health_check',
  BETS: '/bets',
  REGISTER: '/register',
  LOGIN: '/login',
} as const;
