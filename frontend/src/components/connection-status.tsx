'use client';

import { Badge } from '@/components/ui/badge';
import { useSocket } from '@/hooks/use-socket';
import { cn } from '@/lib/utils';
import { Wifi, WifiOff, RefreshCw } from 'lucide-react';

// badge que muestra el estado de conexión WebSocket en tiempo real
export function ConnectionStatus() {
  const { connectionState } = useSocket({ autoConnect: false });

  const config = {
    connected: {
      variant: 'default' as const,
      icon: Wifi,
      label: 'Conectado',
      className: 'bg-green-600 hover:bg-green-600',
    },
    connecting: {
      variant: 'secondary' as const,
      icon: RefreshCw,
      label: 'Conectando...',
      className: 'animate-pulse',
    },
    reconnecting: {
      variant: 'secondary' as const,
      icon: RefreshCw,
      label: 'Reconectando...',
      className: 'animate-pulse',
    },
    disconnected: {
      variant: 'destructive' as const,
      icon: WifiOff,
      label: 'Desconectado',
      className: '',
    },
  };

  const { variant, icon: Icon, label, className } = config[connectionState];

  return (
    <Badge variant={variant} className={cn('gap-1', className)}>
      <Icon className="h-3 w-3" />
      {label}
    </Badge>
  );
}