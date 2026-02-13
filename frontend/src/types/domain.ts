// refleja backend/src/domain/models.rs

// Apuestas
export interface BetTicket {
  user_id: string;    // UUID como string
  match_id: string;   // UUID como string
  amount: number;     // f64
  odds: number;       // f64
}

export type BetStatus = 'Pending' | 'Validated' | 'Rejected';

export interface ValidateBetRequest {
  user_id: string;
  match_id: string;
  amount: number;
  odds: number;
}

export type ValidateBetResponse = BetTicket;

// Usuarios
export interface User {
  id: string;
  email: string;
  name: string;
  created_at: string; // ISO 8601
}

export interface CreateUserRequest {
  email: string;
  password: string;
  name: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface AuthResponse {
  status: 'created' | 'authenticated';
  user_id: string;
  name?: string;
}

// Sistema

export interface HealthCheckResponse {
  status: 'ok';
}

export interface ActivityLogEntry {
  timestamp: string;
  amount: number;
  latency_ms: number;
}

// Eventos de Websocket
export type WSEventType = 
  | 'bet:validated'
  | 'bet:rejected'
  | 'odds:updated'
  | 'match:status_changed';

export interface WSBetEvent {
  type: 'bet:validated' | 'bet:rejected';
  payload: BetTicket & { status: BetStatus };
}

export interface WSOddsUpdate {
  type: 'odds:updated';
  payload: {
    match_id: string;
    odds: number;
  };
}

export type MatchStatus = 'upcoming' | 'live' | 'finished' | 'suspended';

export interface WSMatchStatusEvent {
  type: 'match:status_changed';
  payload: {
    match_id: string;
    status: MatchStatus;
  };
}

export type WSEvent = WSBetEvent | WSOddsUpdate | WSMatchStatusEvent;
