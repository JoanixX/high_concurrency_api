'use client';

import { useMutation, useQueryClient } from '@tanstack/react-query';
import { placeBet } from '@/lib/api';
import { useBettingStore } from '@/store/betting-store';
import type { ValidateBetRequest } from '@/types/domain';
import { toast } from 'sonner';

// la mutación para colocar apuesta:
// 1. REST POST → backend valida y retorna bet_id
// 2. Agrega a pendingBets en zustand
// 3. WS confirma/rechaza (resuelto en useSocket)
// 4. Invalida cache de historial para que se refresque
export function usePlaceBet() {
  const queryClient = useQueryClient();
  const addPendingBet = useBettingStore((s) => s.addPendingBet);

  return useMutation({
    mutationFn: async (data: ValidateBetRequest) => {
      const start = performance.now();
      const result = await placeBet(data);
      const latency = performance.now() - start;
      return { result, latency, request: data };
    },

    onSuccess: ({ result, request, latency }) => {
      // movemos a pending, debemos esperar que se confirme con el WS
      addPendingBet(result.bet_id, {
        user_id: request.user_id,
        match_id: request.match_id,
        amount: request.amount,
        odds: request.odds,
      });

      // invalidamos historial para que se refresque al navegar
      queryClient.invalidateQueries({ queryKey: ['bet-history'] });
      
      toast.success('Apuesta encolada', {
        description: `La apuesta de $${request.amount} entra en la red... (${Math.round(latency)}ms)`,
      });
    },
    
    onError: (error: any) => {
      const message = error.response?.data?.message || error.message || 'Error desconocido al validar';
      toast.error('Rechazado por el motor', {
        description: message,
      });
    },
  });
}