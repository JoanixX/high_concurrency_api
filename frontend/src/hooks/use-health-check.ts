'use client';

import { useQuery } from '@tanstack/react-query';
import { checkHealth } from '@/lib/api';
import { HEALTH_CHECK_INTERVAL_MS } from '@/lib/constants';
import { useBettingStore } from '@/store/betting-store';
import { useEffect } from 'react';

// polling del health check con TanStack Query (refetch automático)
export function useHealthCheck() {
  const setOnline = useBettingStore((s) => s.setOnline);

  const query = useQuery({
    queryKey: ['health-check'],
    queryFn: checkHealth,
    refetchInterval: HEALTH_CHECK_INTERVAL_MS,
    // no reintentar mucho para health checks
    retry: 1,
  });

  // Sincronizamos con el store
  useEffect(() => {
    setOnline(query.data === true);
  }, [query.data, setOnline]);

  return query;
}
