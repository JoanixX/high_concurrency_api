'use client';

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { useState, type ReactNode } from 'react';

// Provider de TanStack Query — debe ser Client Component
export function QueryProvider({ children }: { children: ReactNode }) {
  const [queryClient] = useState(
    () =>
      new QueryClient({
        defaultOptions: {
          queries: {
            // no refetch al volver a la pestaña (lo controlamos nosotros)
            refetchOnWindowFocus: false,
            // reintentar una vez en caso de error
            retry: 1,
            // datos se consideran frescos por 30s
            staleTime: 30_000,
          },
        },
      })
  );

  return (
    <QueryClientProvider client={queryClient}>
      {children}
    </QueryClientProvider>
  );
}
