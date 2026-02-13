'use client';

import { create } from 'zustand';
import type { AuthResponse } from '@/types/domain';

interface AuthState {
  // Estado
  user_id: string | null;
  name: string | null;
  isAuthenticated: boolean;

  // Acciones
  login: (response: AuthResponse) => void;
  logout: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  user_id: null,
  name: null,
  isAuthenticated: false,

  login: (response) => set({
    user_id: response.user_id,
    name: response.name ?? null,
    isAuthenticated: true,
  }),

  logout: () => {
    if (typeof window !== 'undefined') {
      localStorage.removeItem('auth_token');
    }
    set({
      user_id: null,
      name: null,
      isAuthenticated: false,
    });
  },
}));
