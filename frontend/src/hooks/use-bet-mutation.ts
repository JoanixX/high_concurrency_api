'use client';

import { useMutation } from '@tanstack/react-query';
import { validateBet } from '@/lib/api';
import { useBettingStore } from '@/store/betting-store';
import type { ValidateBetRequest } from '@/types/domain';

// mutación de apuesta con TanStack Query — maneja loading, error, y éxito
export function useBetMutation() {
  const { addLogEntry, setLastBet } = useBettingStore();

  return useMutation({
    mutationFn: async (data: ValidateBetRequest) => {
      const start = performance.now();
      const result = await validateBet(data);
      const latency = performance.now() - start;
      return { result, latency };
    },
    onSuccess: ({ result, latency }) => {
      setLastBet(result);
      addLogEntry({
        timestamp: new Date().toISOString(),
        amount: result.amount,
        latency_ms: Math.round(latency * 100) / 100,
      });
    },
  });
}
