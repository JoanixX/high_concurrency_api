'use client';

import { create } from 'zustand';
import type { ActivityLogEntry, BetTicket } from '@/types/domain';
import { ACTIVITY_LOG_MAX_ENTRIES } from '@/lib/constants';

interface BettingState {
  // Estado
  lastBet: BetTicket | null;
  activityLog: ActivityLogEntry[];
  isOnline: boolean;

  // Acciones
  addLogEntry: (entry: ActivityLogEntry) => void;
  setLastBet: (bet: BetTicket) => void;
  setOnline: (status: boolean) => void;
  clearLog: () => void;
}

export const useBettingStore = create<BettingState>((set) => ({
  lastBet: null,
  activityLog: [],
  isOnline: false,

  addLogEntry: (entry) => set((state) => ({
    activityLog: [
      entry,
      ...state.activityLog,
    ].slice(0, ACTIVITY_LOG_MAX_ENTRIES),
  })),

  setLastBet: (bet) => set({ lastBet: bet }),

  setOnline: (status) => set({ isOnline: status }),

  clearLog: () => set({ activityLog: [] }),
}));
