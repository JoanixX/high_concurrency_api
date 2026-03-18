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

export const BET_STATUS = {
  PENDING: 'Pending',
  ACCEPTED: 'Accepted',
  REJECTED: 'Rejected',
  WON: 'Won',
  LOST: 'Lost',
} as const;

export const BET_SELECTION = {
  HOME_WIN: 'HomeWin',
  AWAY_WIN: 'AwayWin',
  DRAW: 'Draw',
} as const;

export const MATCH_STATUS = {
  NOT_STARTED: 'NotStarted',
  IN_PLAY: 'InPlay',
  FINISHED: 'Finished',
  SUSPENDED: 'Suspended',
} as const;

export const WS_EVENTS = {
  BET_ACCEPTED: 'bet:accepted',
  BET_REJECTED: 'bet:rejected',
  ODDS_UPDATED: 'odds:updated',
  MATCH_STATUS_CHANGED: 'match:status_changed',
} as const;