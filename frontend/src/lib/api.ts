import axios from 'axios';
import { API_BASE_URL, ENDPOINTS } from './constants';
import type {
  ValidateBetRequest,
  ValidateBetResponse,
  CreateUserRequest,
  LoginRequest,
  AuthResponse,
} from '@/types/domain';

// cliente axios preconfigurado con baseURL y timeout
const api = axios.create({
  baseURL: API_BASE_URL,
  timeout: 10_000,
  headers: {
    'Content-Type': 'application/json',
  },
});

// interceptor para inyectar token de auth si existe
api.interceptors.request.use((config) => {
  if (typeof window !== 'undefined') {
    const token = localStorage.getItem('auth_token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
  }
  return config;
});

// Health Check
export async function checkHealth(): Promise<boolean> {
  try {
    const { status } = await api.get(ENDPOINTS.HEALTH_CHECK);
    return status === 200;
  } catch {
    return false;
  }
}

// Apuestas
export async function validateBet(data: ValidateBetRequest): Promise<ValidateBetResponse> {
  const { data: result } = await api.post<ValidateBetResponse>(ENDPOINTS.BETS, data);
  return result;
}

// Auth
export async function registerUser(data: CreateUserRequest): Promise<AuthResponse> {
  const { data: result } = await api.post<AuthResponse>(ENDPOINTS.REGISTER, data);
  return result;
}

export async function loginUser(data: LoginRequest): Promise<AuthResponse> {
  const { data: result } = await api.post<AuthResponse>(ENDPOINTS.LOGIN, data);
  return result;
}

export default api;
