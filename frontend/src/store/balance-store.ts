'use client';

import { create } from 'zustand';

// todo el estado financiero se maneja en centavos enteros (i64 en el backend)
// la conversion a dolares y el formateo visual se delega a la capa de presentacion

interface BalanceState {
  // saldo en centavos (es entero, nunca float)
  balance: number;

  setBalance: (amountCents: number) => void;
  decreaseBalanceOptimistic: (amountCents: number) => void;
  rollbackBalance: (amountCents: number) => void;
}

export const useBalanceStore = create<BalanceState>((set) => ({
  balance: 0,

  setBalance: (amountCents) => set({ balance: Math.trunc(amountCents) }),

  decreaseBalanceOptimistic: (amountCents) =>
    set((state) => ({
      balance: state.balance - Math.trunc(amountCents),
    })),

  rollbackBalance: (amountCents) =>
    set((state) => ({
      balance: state.balance + Math.trunc(amountCents),
    })),
}));