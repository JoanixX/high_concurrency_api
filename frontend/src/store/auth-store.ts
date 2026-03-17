'use client';

import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { AuthResponse } from '@/types/domain';

interface AuthState {
  // Estado
  user_id: string | null;
  name: string | null;
  token: string | null;
  isAuthenticated: boolean;

  // Acciones
  login: (response: AuthResponse & { token: string }) => void;
  logout: () => void;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      user_id: null,
      name: null,
      token: null,
      isAuthenticated: false,

      login: (response) => set({
        user_id: response.user_id,
        name: response.name ?? null,
        token: response.token,
        isAuthenticated: true,
      }),

      logout: () => {
        set({
          user_id: null,
          name: null,
          token: null,
          isAuthenticated: false,
        });
      },
    }),
    {
      name: 'auth-storage', // nombre del item en el storage (debe ser unico)
    }
  )
);
